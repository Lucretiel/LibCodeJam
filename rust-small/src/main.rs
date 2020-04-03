/// This function can accept a Group as input. A Group is:
/// - Most primitive types (ints, floats, char, String)
/// - A LengthPrefixed struct, which is a length followed by a collection of Groups
/// - A tuple of up to 12 Groups
/// It can return a Result of any Display-implementing success and failure.
#[inline(always)]
fn solve(input: LengthPrefixed<u32, Vec<u32>>) -> Result<u32, Never> {
    let mut sum = 0;
    for &value in &input {
        sum += value;
    }
    Ok(sum)
}

/// Everything below this line is a pre-written scaffold based on my own
/// LibCodeJam (https://github.com/Lucretiel/LibCodeJam). It's designed to
/// provide a strongly- and implicitly-typed interface for reading data in
/// the code jam style, as well as provide a typical main loop (read number
/// of cases, then call a function for each case, printing the results). Most
/// of this scaffold is dedicated to a composable, type-safe system for reading
/// tokens or collections of tokens, followed by a buffered input reader
/// designed for minimal data copying & allocating as much as possible

// For interactive problems, rewrite this function. For typical problems,
// implement `solve` instead.
#[inline(always)]
fn base_solve(case: usize, tokens: &mut impl Tokens, ostr: &mut impl io::Write) {
    let input = tokens
        .next()
        .unwrap_or_else(|err| panic!("Error loading input for Case #{}: {}", case, err));

    let solution =
        solve(input).unwrap_or_else(|err| panic!("Error solving Case#{}: {}", case, err));

    writeln!(ostr, "Case #{}: {}", case, solution)
        .unwrap_or_else(|err| panic!("Error printing solution for Case#{}: {}", case, err));
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut tokens = TokensReader::new(stdin_lock);

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let num_cases = tokens
        .next()
        .unwrap_or_else(|err| panic!("Error loading number of test cases: {}", err));

    for case in 1..=num_cases {
        base_solve(case, &mut tokens, &mut stdout_lock)
    }
}

use std::{
    error::Error,
    fmt::{self, Display},
    io,
    iter::{FromIterator, FusedIterator},
    marker::PhantomData,
    str::{from_utf8, FromStr, Utf8Error},
};

/// An error reading a raw token
#[derive(Debug)]
enum RawTokenError {
    OutOfTokens,
    Utf8Error(Utf8Error),
    Io(io::Error),
}

impl Display for RawTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawTokenError::OutOfTokens => write!(f, "ran out of tokens."),
            RawTokenError::Utf8Error(err) => write!(f, "error encoding token as utf8: {}", err),
            RawTokenError::Io(err) => write!(f, "i/o error while reading token: {}", err),
        }
    }
}

impl Error for RawTokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RawTokenError::OutOfTokens => None,
            RawTokenError::Utf8Error(err) => Some(err),
            RawTokenError::Io(err) => Some(err),
        }
    }
}

impl From<Utf8Error> for RawTokenError {
    fn from(err: Utf8Error) -> Self {
        RawTokenError::Utf8Error(err)
    }
}

impl From<io::Error> for RawTokenError {
    fn from(err: io::Error) -> Self {
        RawTokenError::Io(err)
    }
}

/// an error occurred reading the nth element of a collection
#[derive(Debug, Clone)]
struct CollectionError<E> {
    index: usize,
    error: E,
}

impl<E: Display> Display for CollectionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error loading collection at index {}: {}",
            self.index, self.error
        )
    }
}

impl<E: Error + 'static> Error for CollectionError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// An error occurred while loading a single token
#[derive(Debug)]
enum TokenError<E> {
    /// An error occurred while loading the &str
    Raw(RawTokenError),
    /// An error occurred while parsing the str
    Parse { err: E, tok: String },
}

impl<E: Display> Display for TokenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::Raw(err) => err.fmt(f),
            TokenError::Parse { err, tok } => write!(f, "error parsing token '{}': {}", tok, err),
        }
    }
}

impl<E: Error + 'static> Error for TokenError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TokenError::Raw(err) => Some(err),
            TokenError::Parse { err, .. } => Some(err),
        }
    }
}

impl<E> From<RawTokenError> for TokenError<E> {
    fn from(err: RawTokenError) -> Self {
        TokenError::Raw(err)
    }
}

type UsizeTokenError = TokenError<<usize as FromStr>::Err>;

/// An error occurred reading a tuple field
#[derive(Debug)]
struct TupleGroupError {
    index: usize,
    error: Box<dyn Error + 'static>,
}

impl TupleGroupError {
    fn new(index: usize, error: impl Error + 'static) -> Self {
        Self {
            index,
            error: Box::new(error),
        }
    }
}

impl Display for TupleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error loading tuple field at index {}: {}",
            self.index, self.error
        )
    }
}

impl Error for TupleGroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}

impl From<Never> for TupleGroupError {
    fn from(_err: Never) -> Self {
        unreachable!()
    }
}

/// Error reading a length-prefixed collection
#[derive(Debug)]
enum LengthPrefixedError<E> {
    Length(UsizeTokenError),
    Collection(CollectionError<E>),
}

impl<E: Display> Display for LengthPrefixedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthPrefixedError::Length(err) => {
                write!(f, "error loading length of collection: {}", err)
            }
            LengthPrefixedError::Collection(err) => err.fmt(f),
        }
    }
}

impl<E: Error + 'static> Error for LengthPrefixedError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LengthPrefixedError::Length(err) => Some(err),
            LengthPrefixedError::Collection(err) => Some(err),
        }
    }
}

impl<E> From<UsizeTokenError> for LengthPrefixedError<E> {
    fn from(err: UsizeTokenError) -> Self {
        LengthPrefixedError::Length(err)
    }
}

impl<E> From<CollectionError<E>> for LengthPrefixedError<E> {
    fn from(err: CollectionError<E>) -> Self {
        LengthPrefixedError::Collection(err)
    }
}

/// A group is single token or group of tokens that knows how to independently
/// read itself from a token stream. For example, a single number, a set of
/// coordinates, or a length-prefixed list of Group.
trait Group: Sized {
    type Err: Error + 'static;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ViaFromStr<T>(T);

impl<T: FromStr> FromStr for ViaFromStr<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(ViaFromStr)
    }
}

impl<T: FromStr> Group for ViaFromStr<T>
where
    T::Err: Error + 'static,
{
    type Err = TokenError<T::Err>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
        let raw = tokens.next_raw()?;
        raw.parse().map_err(|err| TokenError::Parse {
            err,
            tok: raw.into(),
        })
    }
}

macro_rules! token_via_fromstr {
    ( $($type:ty)* ) => {$(
        impl Group for $type {
            type Err = TokenError<<$type as std::str::FromStr>::Err>;

            fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
                ViaFromStr::from_tokens(tokens).map(|value| value.0)
            }
        }
    )*}
}

token_via_fromstr! {
    u8 u16 u32 u64
    i8 i16 i32 i64
    isize usize
    f32 f64
    char String
}

// TODO: copy in OrderedFloat, NotNaN, etc

#[derive(Debug, Copy, Clone)]
enum Never {}

impl Display for Never {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl Error for Never {}

impl Group for () {
    type Err = Never;
    fn from_tokens(_tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
        Ok(())
    }
}

macro_rules! count {
    () => (0);
    ($thing:ident $(, $rest:ident)*) => (1 + count!($($rest),*))
}

macro_rules! tuple_group {
    () => ();
    ($field:ident $(, $tail:ident)*) => {
        tuple_group!{$($tail),*}

        #[allow(non_snake_case)]
        impl< $field : Group $(, $tail : Group)* > Group for ($field, $($tail,)*)
            where $field::Err: Error + 'static,
            $( $tail::Err: Error + 'static, )*
        {
            type Err = TupleGroupError;

            fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
                let ($($tail,)*) = tokens.next()?;
                let last = tokens.next().map_err(|err| TupleGroupError::new(count!($($tail),*), err))?;

                Ok(($($tail,)* last))
            }
        }
    }
}

tuple_group! {A, B, C, D, E, F, G, H, I, J, K, L}

#[derive(Debug)]
struct LengthPrefixed<G, C> {
    collection: C,
    phantom: PhantomData<G>,
}

impl<G, C> LengthPrefixed<G, C> {
    fn into_inner(self) -> C {
        self.collection
    }
}

impl<G: Group, C: FromIterator<G>> Group for LengthPrefixed<G, C> {
    type Err = LengthPrefixedError<G::Err>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
        let len = tokens.next()?;
        let collection = tokens.collect(len)?;
        Ok(LengthPrefixed {
            collection,
            phantom: PhantomData,
        })
    }
}

impl<G, C: IntoIterator> IntoIterator for LengthPrefixed<G, C> {
    type Item = C::Item;
    type IntoIter = C::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<'a, G, C> IntoIterator for &'a LengthPrefixed<G, C>
where
    &'a C: IntoIterator,
{
    type Item = <&'a C as IntoIterator>::Item;
    type IntoIter = <&'a C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

trait Tokens: Sized {
    fn next_raw(&mut self) -> Result<&str, RawTokenError>;

    fn next<T: Group>(&mut self) -> Result<T, T::Err> {
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

impl<'a, T: Tokens, G: Group> TokensIter<'a, T, G> {
    fn new(tokens: &'a mut T) -> Self {
        Self {
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
        (std::usize::MAX, None)
    }
}

impl<'a, T: Tokens, G: Group> ExactSizeIterator for TokensIter<'a, T, G> {
    // Technically a lie, but we rely on .take to constrain it.
    fn len(&self) -> usize {
        ::std::usize::MAX
    }
}

impl<'a, T: Tokens, G: Group> FusedIterator for TokensIter<'a, T, G> {}

// Unfortunately, it's not possible to implement `Tokens` directly on a BufRead,
// because we can't control the size of its buffer. We therefore have this
// TokenBuffer / TokensReader construct to wrap a bufread (like stdin)

#[derive(Debug)]
struct TokenBuffer(Vec<u8>);

impl TokenBuffer {
    fn new() -> Self {
        Self::with_buf(Vec::with_capacity(1024))
    }

    fn with_buf(buf: Vec<u8>) -> Self {
        TokenBuffer(buf)
    }

    // Start a new token
    fn lock(&mut self) -> TokenBufferLock {
        self.0.clear();
        TokenBufferLock(&mut self.0)
    }
}

#[derive(Debug)]
struct TokenBufferLock<'a>(&'a mut Vec<u8>);

impl<'a> TokenBufferLock<'a> {
    // Add data to this token
    fn extend(&mut self, chunk: &[u8]) {
        self.0.extend_from_slice(chunk);
    }

    // Complete the token: try to parse the buffer into a string and return
    // a refernce to it
    fn complete(self) -> Result<&'a str, RawTokenError> {
        from_utf8(self.0).map_err(RawTokenError::Utf8Error)
    }
}

pub struct TokensReader<R: io::BufRead> {
    reader: R,
    token: TokenBuffer,
    need_consume: usize,
}

impl<R: io::BufRead> TokensReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            token: TokenBuffer::new(),
            need_consume: 0,
        }
    }
}

impl<R: io::BufRead> Tokens for TokensReader<R> {
    fn next_raw(&mut self) -> Result<&str, RawTokenError> {
        use std::io::ErrorKind::Interrupted;

        self.reader.consume(self.need_consume);
        self.need_consume = 0;

        // Clear leading whitespace
        loop {
            match self.reader.fill_buf() {
                Err(ref err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(RawTokenError::Io(err)),
                Ok([]) => return Err(RawTokenError::OutOfTokens),
                Ok(buf) => match buf.iter().position(|byte| !byte.is_ascii_whitespace()) {
                    Some(len) => {
                        self.reader.consume(len);
                        break;
                    }
                    None => {
                        let len = buf.len();
                        self.reader.consume(len)
                    }
                },
            }
        }

        // If we reach this point, there is definitely a non-empty token ready to be read.
        let mut token_buf = self.token.lock();

        // TODO: If, on the first iteration through this loop, we immdiately
        // encounter a whitespace-terminated token in the buffer, we should be
        // able to return that token directly, rather than copying it into the
        // buffer. This is currently not possible due to lifetime issues that
        // I *think* are incorrect.

        loop {
            match self.reader.fill_buf() {
                Err(ref err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(RawTokenError::Io(err)),
                Ok([]) => return token_buf.complete(),
                Ok(buf) => match buf.iter().position(|byte| byte.is_ascii_whitespace()) {
                    Some(len) => {
                        token_buf.extend(&buf[..len]);
                        // Consume +1 to eat the space we found
                        self.reader.consume(len + 1);
                        return token_buf.complete();
                    }
                    None => {
                        token_buf.extend(buf);
                        let len = buf.len();
                        self.reader.consume(len);
                    }
                },
            }
        }
    }
}
