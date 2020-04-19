// These lints need to be disabled for Rust 1.24 compatibility
#![allow(unused_imports)]
#![allow(bare_trait_objects)]

// This is a pre-written single-file scaffold based on my own LibCodeJam
// (https://github.com/Lucretiel/LibCodeJam). It's designed to provide a
// strongly- and implicitly-typed interface for reading data in the code jam
// style, as well as provide a typical main loop (read number of cases, then
// call a function for each case, printing the results). Most of this scaffold
// is dedicated to a composable, type-safe system for reading tokens or
// collections of tokens, followed by a buffered input reader designed for
// minimal data copying & allocating.
//
// The actual solution function is at the bottom of this file.

// This set of imports should comprehensively include everything we might need
// to solve a typical code jam problem. Stuff added here should be added
// permanently.
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::iter::{repeat, FromIterator};
use std::marker::PhantomData;
use std::mem;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};
use std::process::exit;
use std::rc::{Rc, Weak};
use std::str::{from_utf8, FromStr, Utf8Error};

/// Main loop for solving code jam problems. Performs the following steps:
/// - Create a Tokens instance associated with stdin
/// - Read the number of cases, a usize value
/// - Read any global data (data needed for all test cases). Type inferred
///   by the solve function.
/// - Call the solve function once for each test case, sending the results
///   to stdout.
/// - Panic with detailed information in the event of any errors.
fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut tokens = TokensReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();

    // Currently, stdout is line-buffered unconditionally; this is simply
    // a future-proof. We do this to simplify interactive problems, which are
    // line-oriented.
    let mut line_buffered = io::LineWriter::new(stdout_lock);

    let num_cases = tokens.next().unwrap_or_else(|err| {
        panic!("Error loading number of cases: {}", err);
    });

    let global_data = tokens.next().unwrap_or_else(|err| {
        panic!("Error loading global data: {}", err);
    });

    for case in 0..num_cases {
        solve(case + 1, &global_data, &mut tokens, &mut line_buffered).unwrap_or_else(|err| {
            panic!("Error solving Case #{}: {}", case + 1, err);
        });
    }
}

/// A group is single token or group of tokens that knows how to independently
/// read itself from a token stream. For example, a single number, a set of
/// coordinates, or a length-prefixed list of Groups.
trait Group: Sized {
    type Err: Error + 'static;

    fn from_tokens<T: Tokens>(tokens: &mut T) -> Result<Self, Self::Err>;
}

/// Create a Group implementation for any type implementing FromStr. We can't
/// do a blanket implementation for type coherence reasons. Other FromStr types
/// can be loaded with next_token, or have Group implemented with this macro.
macro_rules! token {
    ( $($type:ty)* ) => {$(
        impl Group for $type {
            type Err = TokenError<<$type as std::str::FromStr>::Err>;

            #[inline(always)]
            fn from_tokens<T: Tokens>(tokens: &mut T) -> Result<Self, Self::Err> {
                tokens.next_token()
            }
        }
    )*}
}

token! {
    u8 u16 u32 u64
    i8 i16 i32 i64
    isize usize
    f32 f64
    char String
}

// TODO: copy in OrderedFloat, NotNaN, etc

impl Group for () {
    type Err = Never;

    #[inline(always)]
    fn from_tokens<T: Tokens>(_tokens: &mut T) -> Result<Self, Self::Err> {
        Ok(())
    }
}

macro_rules! count {
    () => (0);
    ($thing:ident $(, $rest:ident)*) => (1 + count!($($rest),*))
}

/// Recursively implement Group for Tuples of length 1 through the number of
/// provided identifiers.
macro_rules! tuple_group {
    () => ();
    ($head:ident $(, $tail:ident)*) => {
        tuple_group!{$($tail),*}

        #[allow(non_snake_case)]
        impl< $head : Group , $( $tail : Group, )* > Group for ($head, $($tail,)*)
            where $head::Err: Error + 'static,
            $( $tail::Err: Error + 'static, )*
        {
            type Err = TupleGroupError;

            #[inline(always)]
            fn from_tokens<T: Tokens>(tokens: &mut T) -> Result<Self, Self::Err> {
                let ($($tail,)*) = tokens.next()?;

                // Need to load head last because the reported error index is
                // the length of the tail
                let $head = tokens.next().map_err(|err|
                    TupleGroupError::new(count!($($tail),*), err)
                )?;

                Ok(($($tail,)* $head,))
            }
        }
    }
}

tuple_group! {A, B, C, D, E, F, G, H, I, J, K, L}

/// A length prefixed list of something. Because many collections support
/// FromIterable with more than 1 type, we have to specify what type is being
/// collected as the second generic parameter.
#[derive(Debug, Clone)]
struct LengthPrefixed<C, G> {
    collection: C,
    phantom: PhantomData<G>,
}

impl<G, C> LengthPrefixed<C, G> {
    #[allow(dead_code)]
    fn into_inner(self) -> C {
        self.collection
    }
}

impl<G: Group, C: FromIterator<G>> Group for LengthPrefixed<C, G> {
    type Err = LengthPrefixedError<G::Err>;

    #[inline(always)]
    fn from_tokens<T: Tokens>(tokens: &mut T) -> Result<Self, Self::Err> {
        let len = tokens.next()?;
        Ok(LengthPrefixed {
            collection: tokens.collect(len)?,
            phantom: PhantomData,
        })
    }
}

impl<G, C: IntoIterator> IntoIterator for LengthPrefixed<C, G> {
    type Item = C::Item;
    type IntoIter = C::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<'a, G, C> IntoIterator for &'a LengthPrefixed<C, G>
where
    &'a C: IntoIterator,
{
    type Item = <&'a C as IntoIterator>::Item;
    type IntoIter = <&'a C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<C, G> Deref for LengthPrefixed<C, G> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl<C, G> DerefMut for LengthPrefixed<C, G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.collection
    }
}

trait Tokens: Sized {
    /// Load the next whitespace separated token as a str
    fn next_raw(&mut self) -> Result<&str, RawTokenError>;

    /// Shortcut method for loading FromStr types
    fn next_token<T: FromStr>(&mut self) -> Result<T, TokenError<T::Err>>
    where
        T::Err: Error + 'static,
    {
        let raw = self.next_raw()?;
        raw.parse().map_err(|err| TokenError::Parse {
            err,
            tok: raw.into(),
        })
    }

    /// Load any Group type
    #[inline(always)]
    fn next<T: Group>(&mut self) -> Result<T, T::Err> {
        T::from_tokens(self)
    }

    /// Load N Groups into a collection
    #[inline(always)]
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
struct TokensIter<'a, T: Tokens + 'a, G: Group> {
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

// Unfortunately, it's not possible to implement `Tokens` directly on a BufRead,
// because we can't control the size of its buffer. We therefore have this
// TokenBuffer / TokensReader construct to wrap a BufRead (like stdin)

#[derive(Debug)]
struct TokenBuffer(Vec<u8>);

impl TokenBuffer {
    fn new() -> Self {
        Self::with_capacity(1024)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_buf(Vec::with_capacity(capacity))
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
    // a reference to it
    fn complete(self) -> Result<&'a str, RawTokenError> {
        from_utf8(self.0).map_err(RawTokenError::Utf8Error)
    }
}

#[derive(Debug)]
pub struct TokensReader<R: io::BufRead> {
    reader: R,
    token: TokenBuffer,
}

impl<R: io::BufRead> TokensReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            token: TokenBuffer::new(),
        }
    }
}

impl<R: io::BufRead> Tokens for TokensReader<R> {
    fn next_raw(&mut self) -> Result<&str, RawTokenError> {
        use std::io::ErrorKind::Interrupted;

        // TODO: clean this up when NLL is ready
        // Clear leading whitespace
        let final_leading_ws = loop {
            let leading_ws = match self.reader.fill_buf() {
                Err(ref err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(RawTokenError::Io(err)),
                Ok(buf) if buf.is_empty() => return Err(RawTokenError::OutOfTokens),
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
                Err(err) => return Err(RawTokenError::Io(err)),
                Ok(buf) if buf.is_empty() => return token_buf.complete(),
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

/// An error reading a raw token
#[derive(Debug)]
enum RawTokenError {
    OutOfTokens,
    Utf8Error(Utf8Error),
    Io(io::Error),
}

impl Display for RawTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RawTokenError::OutOfTokens => write!(f, "ran out of tokens."),
            &RawTokenError::Utf8Error(ref err) => {
                write!(f, "error encoding token as utf8: {}", err)
            }
            &RawTokenError::Io(ref err) => write!(f, "i/o error while reading token: {}", err),
        }
    }
}

impl Error for RawTokenError {
    fn description(&self) -> &str {
        "Error getting raw token"
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "error loading collection at index {}: {}",
            self.index, self.error
        )
    }
}

impl<E: Error + 'static> Error for CollectionError<E> {
    fn description(&self) -> &str {
        "Error loading collection"
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TokenError::Raw(ref err) => Display::fmt(err, f),
            &TokenError::Parse { ref err, ref tok } => {
                write!(f, "error parsing token '{}': {}", tok, err)
            }
        }
    }
}

impl<E: Error + 'static> Error for TokenError<E> {
    fn description(&self) -> &str {
        "Error loading token"
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
    error: Box<Error + 'static>,
}

impl TupleGroupError {
    fn new<E: Error + 'static>(index: usize, error: E) -> Self {
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
    fn description(&self) -> &str {
        "Error loading element of tuple"
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LengthPrefixedError::Length(ref err) => {
                write!(f, "error loading length of collection: {}", err)
            }
            &LengthPrefixedError::Collection(ref err) => err.fmt(f),
        }
    }
}

impl<E: Error + 'static> Error for LengthPrefixedError<E> {
    fn description(&self) -> &str {
        "Error loading length-prefixed collection"
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

#[derive(Debug, Copy, Clone)]
enum Never {}

impl Display for Never {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl Error for Never {
    fn description(&self) -> &str {
        unreachable!()
    }
}

/// Simple macro for creating typed enums with a FromStr and Group
/// implementation. Example:
///
/// input_enum! {Direction {
///     "U" => Up,
///     "D" => Down,
///     "L" => Left,
///     "R" => Right,
/// }}
#[allow(unused_macros)]
macro_rules! input_enum {
    ($Name:ident {$(
        $pattern:expr => $Variant:ident,
    )*} err $Error:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum $Name {$(
            $Variant,
        )*}

        impl FromStr for $Name {
            type Err = $Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if false { unreachable!() }
                $(else if s.eq_ignore_ascii_case($pattern) {
                    Ok($Name::$Variant)
                })*
                else {Err($Error)}
            }
        }

        token! { $Name }

        #[derive(Debug, Copy, Clone)]
        struct $Error;

        impl Display for $Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!("Invalid token for ", stringify!($Name)))
            }
        }

        impl Error for $Error {
            fn description(&self) -> &str {
                concat!("Invalid token for ", stringify!($Name))
            }
        }
    };

    ($Name:ident {$(
        $pattern:expr => $Variant:ident,
    )*}) => {
        input_enum!{
            $Name {$($pattern => $Variant,)*} err EnumError
        }
    };
}

/// Generic function to solve a single test case. This is the part that
/// solution authors should edit. This function is called in a loop in main.
///
/// case: always a usize, it is the 1-indexed case number.
/// global_data: any Group-implementing function (including ()); this data is
/// loaded once (after num_cases) and passed by reference to each test case
/// tokens: a Tokens struct, for reading tokens and Groups from stdin
/// ostr: A line-buffered stdout target for writing your solution.
///
/// A Group is a token or collection of tokens that can be loaded from an input
/// stream. Specifically, it is:
///
/// - An int or float
/// - a char or String
/// - any tuple of Groups
/// - LengthPrefixed<Collection, Group>, which is an integer N followed by
/// N groups. The collection is constructed via FromIterator<Group>
#[inline(always)]
fn solve<T: Tokens, W: io::Write>(
    case: usize,
    _global: &(),
    tokens: &mut T,
    ostr: &mut W,
) -> Result<(), Box<Error>> {
    let value: String = tokens.next()?;

    writeln!(ostr, "Case #{}: {}", case, value)?;
    Ok(())
}
