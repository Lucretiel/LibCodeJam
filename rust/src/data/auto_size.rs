use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use derive_more::*;

use crate::data::group::{Group, UsizeTokenError};
use crate::tokens::CollectionError;
use crate::tokens::Tokens;

#[derive(Debug, From)]
pub enum AutoSizeError<E: Error> {
    CountError(UsizeTokenError),
    CollectionError(CollectionError<E>),
}

impl<E: Error> Display for AutoSizeError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AutoSizeError::CountError(err) => {
                write!(f, "error reading number of elements in collection: {}", err)
            }
            AutoSizeError::CollectionError(err) => err.fmt(f),
        }
    }
}

impl<E: Error> Error for AutoSizeError<E> {
    fn cause(&self) -> Option<&Error> {
        Some(match self {
            AutoSizeError::CountError(err) => err,
            AutoSizeError::CollectionError(err) => err,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AutoSize<T: Group, C: FromIterator<T>> {
    pub collection: C,
    phantom: PhantomData<T>,
}

impl<T: Group, C: FromIterator<T>> Deref for AutoSize<T, C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.collection
    }
}

impl<T: Group, C: FromIterator<T>> DerefMut for AutoSize<T, C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<T: Group, C: FromIterator<T>> AsRef<C> for AutoSize<T, C> {
    fn as_ref(&self) -> &C {
        &self.collection
    }
}

impl<T: Group, C: FromIterator<T>> AsMut<C> for AutoSize<T, C> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<C: FromIterator<T>, T: Group> Group for AutoSize<T, C> {
    type Error = AutoSizeError<T::Error>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Error> {
        let size = tokens.next()?;
        Ok(AutoSize {
            collection: tokens.collect(size)?,
            phantom: PhantomData,
        })
    }
}
