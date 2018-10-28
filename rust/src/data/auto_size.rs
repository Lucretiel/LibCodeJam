use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::data::collection::Collection;
use crate::data::collection::CollectionError;
use crate::data::group::{Group, UsizeTokenError};
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
pub struct AutoSize<C, T> {
    pub collection: C,
    phantom: PhantomData<T>,
}

impl<C, T> Deref for AutoSize<C, T> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.collection
    }
}

impl<C, T> DerefMut for AutoSize<C, T> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<C, T> AsRef<C> for AutoSize<C, T> {
    fn as_ref(&self) -> &C {
        &self.collection
    }
}

impl<C, T> AsMut<C> for AutoSize<C, T> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<C, T> Group for AutoSize<C, T>
where
    C: Collection<Item = T>,
    T: Group,
{
    type Error = AutoSizeError<T::Error>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Error> {
        let size = tokens.next()?;
        Ok(Self {
            collection: tokens.collect(size)?,
            phantom: PhantomData,
        })
    }
}
