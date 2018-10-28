use std::error::Error;

use crate::data::token::{Token, TokenError};
use crate::tokens::Tokens;

pub trait Group: Sized {
    type Error: Error;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Error>;
}

impl<T> Group for T
where
    T: Token,
{
    type Error = TokenError<T::Error>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Error> {
        tokens.next_token()
    }
}

/*
macro_rules! struct_error_type {
    (($type:ty)) =>
        (<$type as $crate::data::Group>::Error);
    (($type:ident < $subtype:ty > @ $length:expr)) =>
        ($crate::data::CollectionError<<$subtype as $crate::data>)
}

#[macro_export]
macro_rules! group {
    (struct $Struct:ident, err $StructErr:ident {$(
        $name:ident : $spec:tt
    )*}) => {
        #[derive(Debug)]
        enum $StructErr {$(
            #[allow(non_camel_case_types)]
            $name (struct_error_type!($spec)),
        )*}

        #[derive(Debug, Clone)]
        struct $Struct {}
    };
}

group!{struct Point, err PointError {
    x: (usize)
    y: (usize)
}}

group!{struct Balls, err BallError {
    columns: (usize)
    counts: (Vec<usize> @ columns)
}}
*/

pub type UsizeTokenError = <usize as Group>::Error;
