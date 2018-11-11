use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::iter::{FromIterator, FusedIterator, TrustedLen};
use std::marker::PhantomData;
use std::ops::Try;
use std::str::{from_utf8, Utf8Error};

use derive_more::From;

use crate::data::{GlobalData, GlobalDataError, LoadGlobalData, Group};

#[derive(Debug, From)]
pub enum LoadError {
    Io(io::Error),
    Utf8Error(Utf8Error),
    OutOfTokens,
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LoadError::OutOfTokens => write!(f, "ran out of input tokens"),
            LoadError::Io(err) => write!(f, "io error while reading token: {}", err),
            LoadError::Utf8Error(err) => write!(f, "error encoding token as UTF-8: {}", err),
        }
    }
}

impl Error for LoadError {
    fn cause(&self) -> Option<&Error> {
        match self {
            LoadError::OutOfTokens => None,
            LoadError::Io(err) => Some(err),
            LoadError::Utf8Error(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub struct CollectionError<E: Error> {
    index: usize,
    error: E,
}

impl<E: Error> Display for CollectionError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "error loading collection at index {}: {}",
            self.index, self.error
        )
    }
}

impl<E: Error> Error for CollectionError<E> {
    fn cause(&self) -> Option<&Error> {
        Some(&self.error)
    }
}

pub trait Tokens: Sized {
    fn next_raw(&mut self) -> Result<&str, LoadError>;

    fn next<T: Group>(&mut self) -> Result<T, T::Err> {
        T::from_tokens(self)
    }

    fn start_problem<T: LoadGlobalData>(
        &mut self,
    ) -> Result<GlobalData<T>, GlobalDataError<T::Err>> {
        T::from_tokens(self)
    }

    fn collect<T: Group, C: FromIterator<T>>(
        &mut self,
        count: usize,
    ) -> Result<C, CollectionError<T::Err>> {
        TokensIter::new(self)
            .take(count)
            .enumerate()
            .map(|(index, result)| result.map_err(|error| CollectionError { index, error }))
            .collect()
    }
}

#[derive(Debug)]
struct TokensIter<'a, T: Tokens, G: Group> {
    tokens: &'a mut T,
    phantom: PhantomData<G>,
}

impl<'a, T: 'a + Tokens, G: Group> TokensIter<'a, T, G> {
    fn new(tokens: &'a mut T) -> Self {
        TokensIter {
            tokens,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tokens, G: Group> Iterator for TokensIter<'a, T, G> {
    type Item = Result<G, G::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tokens.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (::std::usize::MAX, None)
    }

    fn try_fold<B, F, R>(&mut self, mut accum: B, mut fold: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Ok = B>,
    {
        loop {
            accum = fold(accum, self.tokens.next())?
        }
    }
}

impl<'a, T: Tokens, G: Group> ExactSizeIterator for TokensIter<'a, T, G> {
    // Technically a lie, but we rely on .take to constrain it.
    fn len(&self) -> usize {
        ::std::usize::MAX
    }
}

impl<'a, T: Tokens, G: Group> FusedIterator for TokensIter<'a, T, G> {}
unsafe impl<'a, T: Tokens, G: Group> TrustedLen for TokensIter<'a, T, G> {}

#[derive(Debug)]
struct TokenBuffer(Vec<u8>);

#[derive(Debug)]
struct TokenBufferLock<'a>(&'a mut Vec<u8>);

impl TokenBuffer {
    fn lock(&mut self) -> TokenBufferLock {
        self.0.clear();
        TokenBufferLock(&mut self.0)
    }

    fn new() -> Self {
        Self::with_buf(Vec::with_capacity(1024))
    }

    fn with_buf(buf: Vec<u8>) -> Self {
        TokenBuffer(buf)
    }
}

impl<'a> TokenBufferLock<'a> {
    fn extend(&mut self, chunk: &[u8]) {
        self.0.extend(chunk)
    }

    fn complete(self) -> Result<&'a str, LoadError> {
        from_utf8(self.0).map_err(LoadError::Utf8Error)
    }
}

pub struct TokensReader<R: io::BufRead> {
    reader: R,
    token: TokenBuffer,
}

impl<R: io::BufRead> TokensReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            token: TokenBuffer::new()
        }
    }
}

impl TokensReader<io::BufReader<io::Stdin>> {
    pub fn stdin() -> Self {
        Self::new(io::BufReader::new(io::stdin()))
    }
}

impl<R: io::BufRead> Tokens for TokensReader<R> {
    fn next_raw(&mut self) -> Result<&str, LoadError> {
        use std::io::ErrorKind::Interrupted;

        // TODO: clean this up when NLL is ready
        // Clear leading whitespace
        let final_leading_ws = loop {
            let leading_ws = match self.reader.fill_buf() {
                Err(ref err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(LoadError::Io(err)),
                Ok([]) => return Err(LoadError::OutOfTokens),
                Ok(buf) => match buf.iter().position(|byte| !byte.is_ascii_whitespace()) {
                    Some(i) => break i,
                    None => buf.len(),
                },
            };
            self.reader.consume(leading_ws);
        };
        self.reader.consume(final_leading_ws);

        // If we reach this point, there is definitely a non-empty token ready to be read.
        let mut token_buf = self.token.lock();

        let final_amt = loop {
            let amt = match self.reader.fill_buf() {
                Err(ref err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(LoadError::Io(err)),
                Ok([]) => return token_buf.complete(),
                Ok(buf) => match buf.iter().position(u8::is_ascii_whitespace) {
                    Some(i) => {
                        token_buf.extend(&buf[..i]);
                        break i + 1;
                    }
                    None => {
                        token_buf.extend(buf);
                        buf.len()
                    }
                },
            };
            self.reader.consume(amt);
        };
        self.reader.consume(final_amt);

        token_buf.complete()
    }
}

#[derive(Debug)]
pub struct TokensFromIterator<'a, T: Iterator<Item = &'a str>> {
    iter: T,
}

impl<'a, T: Iterator<Item = &'a str>> Tokens for TokensFromIterator<'a, T> {
    fn next_raw(&mut self) -> Result<&str, LoadError> {
        self.iter.next().ok_or(LoadError::OutOfTokens)
    }
}
