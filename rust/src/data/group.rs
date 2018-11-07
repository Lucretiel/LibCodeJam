use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::tokens::{LoadError, Tokens};

pub trait Group: Sized {
    type Err: Error + 'static;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err>;
}

// TOKEN TYPES
#[derive(Debug)]
pub enum TokenError<E: Error> {
    LoadError(LoadError),
    ParseError { err: E, tok: String },
}

impl<E: Error> Display for TokenError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenError::LoadError(err) => err.fmt(f),
            TokenError::ParseError { err, tok } => {
                write!(f, "error parsing token \"{}\": {}", tok, err)
            }
        }
    }
}

impl<E: Error> Error for TokenError<E> {
    fn cause(&self) -> Option<&Error> {
        match self {
            TokenError::LoadError(err) => Some(err),
            TokenError::ParseError { err, .. } => Some(err),
        }
    }
}

impl<E: Error> From<LoadError> for TokenError<E> {
    fn from(err: LoadError) -> TokenError<E> {
        TokenError::LoadError(err)
    }
}

macro_rules! token_via_fromstr {
    ( $( $type:ty )+ ) => {$(
        impl Group for $type {
            type Err = TokenError<<$type as std::str::FromStr>::Err>;

            fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
                let raw = tokens.next_raw()?;
                raw.parse().map_err(move |err| TokenError::ParseError{err, tok: raw.into()})
            }
        }
    )*}
}

token_via_fromstr!{
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
    char String
}

pub type UsizeTokenError = <usize as Group>::Err;

impl Group for () {
    type Err = !;

    fn from_tokens(_tokens: &mut impl Tokens) -> Result<(), !> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TupleGroupError {
    index: usize,
    error: Box<Error + Send>,
}

impl TupleGroupError {
    pub fn new<E: Error + Send + 'static>(index: usize, error: E) -> Self {
        TupleGroupError {
            index,
            error: Box::new(error),
        }
    }
}

impl Display for TupleGroupError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Error loading tuple field at index {}: {}",
            self.index, self.error
        )
    }
}

impl Error for TupleGroupError {
    fn cause(&self) -> Option<&Error> {
        Some(self.error.as_ref())
    }
}

impl From<!> for TupleGroupError {
    #[allow(unreachable_code)]
    fn from(err: !) -> Self {
        TupleGroupError::new(0, err)
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
            where $field::Err: Error + Send,
            $( $tail::Err: Error + Send, )*
        {
            type Err = TupleGroupError;

            fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
                let ($($tail,)*) = tokens.next()?;
                let last = tokens.next().map_err(|err| TupleGroupError::new(count!($($tail),*), err))?;

                Ok(($($tail,)*  last))
            }
        }
    }
}

tuple_group!{A, B, C, D, E, F, G, H, I, J, K, L}

#[derive(Debug)]
pub struct StructGroupError {
    field: String,
    error: Box<Error + Send>,
}

impl StructGroupError {
    pub fn new<E: Error + Send + 'static>(field: &'static str, error: E) -> Self {
        StructGroupError {
            field: field.to_string(),
            error: Box::new(error),
        }
    }
}

impl Display for StructGroupError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "error loading struct field {}: {}",
            self.field, self.error
        )
    }
}

impl Error for StructGroupError {
    fn cause(&self) -> Option<&Error> {
        Some(self.error.as_ref())
    }
}

#[macro_export]
macro_rules! make_struct_field {
    ($tokens:ident) => {
        $tokens.next()
    };
    ($tokens:ident @ $size:expr) => {
        $tokens.collect($size)
    };
}

#[macro_export]
macro_rules! struct_group {
    (struct $Name:ident {
        $($field:ident : $type:ty $(=> $size:expr )* ,)*
    }) => (
        #[derive(Debug)]
        pub struct $Name {
            $(pub $field: $type,)*
        }

        impl Group for $Name {
            type Err = StructGroupError;

            fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
                $(
                    let $field = make_struct_field!(tokens $(@ $size)*)
                        .map_err(|err| StructGroupError::new(stringify!($field), err))?;
                )*

                Ok(Self {$(
                    $field,
                )*})
            }
        }
    )
}
