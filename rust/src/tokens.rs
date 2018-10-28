use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::str::{from_utf8, Utf8Error};

use crate::data::collection::{Collection, CollectionError};
use crate::data::group::Group;
use crate::data::token::{Token, TokenError};

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Utf8Error(Utf8Error),
    OutOfTokens,
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::Io(err)
    }
}

impl From<Utf8Error> for LoadError {
    fn from(err: Utf8Error) -> Self {
        LoadError::Utf8Error(err)
    }
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

pub trait Tokens: Sized {
    fn next_raw(&mut self) -> Result<&str, LoadError>;

    fn next_token<T: Token>(&mut self) -> Result<T, TokenError<T::Error>> {
        self.next_raw()
            .map_err(TokenError::LoadError)
            .and_then(|tok| {
                T::from_raw(tok).map_err(|err| TokenError::ParseError {
                    err,
                    tok: tok.to_string(),
                })
            })
    }

    fn next<T: Group>(&mut self) -> Result<T, T::Error> {
        T::from_tokens(self)
    }

    fn collect<C, T>(&mut self, count: usize) -> Result<C, CollectionError<T::Error>>
    where
        C: Collection<Item = T>,
        T: Group,
    {
        C::from_tokens(self, count)
    }
}

#[derive(Debug)]
struct TokenBuffer(Vec<u8>);

#[derive(Debug)]
struct TokenBufferLock<'a>(&'a mut Vec<u8>);

impl TokenBuffer {
    fn lock(&mut self) -> TokenBufferLock {
        self.0.clear();
        TokenBufferLock(&mut self.0)
    }
}

impl TokenBuffer {
    #[inline(always)]
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
        from_utf8(self.0).map_err(LoadError::from)
    }
}

pub struct TokensReader<R: io::BufRead> {
    reader: R,
    token: TokenBuffer,
}

impl<R: io::BufRead> TokensReader<R> {
    pub fn new(reader: R) -> Self {
        TokensReader {
            reader: reader,
            token: TokenBuffer::new(),
        }
    }
}

impl<R: io::BufRead> TokensReader<R> {
    pub fn new_with_buf(reader: R, buffer: Vec<u8>) -> Self {
        TokensReader {
            reader,
            token: TokenBuffer::with_buf(buffer),
        }
    }
}

impl<R: io::BufRead> Tokens for TokensReader<R> {
    fn next_raw(&mut self) -> Result<&str, LoadError> {
        use std::io::ErrorKind::Interrupted;

        // Clear leading whitespace
        let final_leading_ws = loop {
            let leading_ws = match self.reader.fill_buf() {
                Err(ref e) if e.kind() == Interrupted => continue,
                Err(e) => return Err(e.into()),
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
                Err(ref e) if e.kind() == Interrupted => continue,
                Err(e) => return Err(e.into()),
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
