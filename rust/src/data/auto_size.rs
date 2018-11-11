use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;

use derive_more::From;

use crate::data::{Group, UsizeTokenError};
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

pub trait Collection: IntoIterator + FromIterator<<Self as IntoIterator>::Item> {}

impl<C> Collection for C
where
    C: IntoIterator,
    C: FromIterator<<C as IntoIterator>::Item>,
{
}

#[derive(Debug, Clone)]
pub struct AutoSize<C: Collection>
where
    C::Item: Group,
{
    pub count: usize,
    pub collection: C,
}

impl<C: Collection> Deref for AutoSize<C>
where
    C::Item: Group,
{
    type Target = C;

    fn deref(&self) -> &C {
        &self.collection
    }
}

impl<C: Collection> DerefMut for AutoSize<C>
where
    C::Item: Group,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<C: Collection> AsRef<C> for AutoSize<C>
where
    C::Item: Group,
{
    fn as_ref(&self) -> &C {
        &self.collection
    }
}

impl<C: Collection> AsMut<C> for AutoSize<C>
where
    C::Item: Group,
{
    fn as_mut(&mut self) -> &mut C {
        &mut self.collection
    }
}

impl<C: Collection> IntoIterator for AutoSize<C>
where
    C::Item: Group,
{
    type Item = C::Item;
    type IntoIter = C::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<'a, C: Collection> IntoIterator for &'a AutoSize<C>
where
    C::Item: Group,
    &'a C: IntoIterator,
{
    type Item = <&'a C as IntoIterator>::Item;
    type IntoIter = <&'a C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.collection).into_iter()
    }
}

impl<'a, C: Collection> IntoIterator for &'a mut AutoSize<C>
where
    C::Item: Group,
    &'a mut C: IntoIterator,
{
    type Item = <&'a mut C as IntoIterator>::Item;
    type IntoIter = <&'a mut C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.collection).into_iter()
    }
}

impl<C: Collection> Group for AutoSize<C>
where
    C::Item: Group,
{
    type Err = AutoSizeError<<C::Item as Group>::Err>;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<Self, Self::Err> {
        let count = tokens.next()?;
        let collection: C = tokens.collect(count)?;
        Ok(Self { collection, count })
    }
}
