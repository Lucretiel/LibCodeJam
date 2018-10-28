use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::iter::{FromIterator, FusedIterator, TrustedLen};
use std::marker::PhantomData;
use std::ops::Try;

use crate::data::group::Group;
use crate::tokens::Tokens;

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

pub trait Collection: Sized + IntoIterator
where
    Self::Item: Group,
{
    fn from_tokens(
        tokens: &mut impl Tokens,
        count: usize,
    ) -> Result<Self, CollectionError<<Self::Item as Group>::Error>>;
}

struct IterTokens<'a, T: 'a, G> {
    tokens: &'a mut T,
    phantom: PhantomData<G>,
}

impl<'a, T, G> IterTokens<'a, T, G> {
    fn new(tokens: &'a mut T) -> Self {
        IterTokens {
            tokens,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tokens, G: Group> Iterator for IterTokens<'a, T, G> {
    type Item = Result<G, G::Error>;

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

impl<'a, T: Tokens, G: Group> ExactSizeIterator for IterTokens<'a, T, G> {
    // Technically a lie, but we rely on take to constrain it.
    fn len(&self) -> usize {
        ::std::usize::MAX
    }
}

impl<'a, T: Tokens, G: Group> FusedIterator for IterTokens<'a, T, G> {}
unsafe impl<'a, T: Tokens, G: Group> TrustedLen for IterTokens<'a, T, G> {}

impl<C, G> Collection for C
where
    G: Group,
    C: FromIterator<G>,
    C: IntoIterator<Item = G>,
{
    fn from_tokens(
        tokens: &mut impl Tokens,
        count: usize,
    ) -> Result<Self, CollectionError<G::Error>> {
        IterTokens::new(tokens)
            .take(count)
            .enumerate()
            .map(|(index, result)| result.map_err(|error| CollectionError { index, error }))
            .collect()
    }
}
