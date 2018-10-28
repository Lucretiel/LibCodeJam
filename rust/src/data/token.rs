use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::tokens::LoadError;

#[derive(Debug)]
pub enum TokenError<E: Error> {
    LoadError(LoadError),
    ParseError { err: E, tok: String },
}

impl<E: Error> Display for TokenError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenError::LoadError(err) => write!(f, "error reading token: {}", err),
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

pub trait Token: Sized {
    type Error: Error;

    fn from_raw(tok: &str) -> Result<Self, Self::Error>;
}

macro_rules! token_via_fromstr {
	( $( $type:ty )+ ) => {$(
		impl Token for $type {
			type Error = <$type as ::std::str::FromStr>::Err;

			fn from_raw(tok: &str) -> Result<Self, Self::Error> {
				tok.parse()
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
