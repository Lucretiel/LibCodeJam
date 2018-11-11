use std::error::Error;
use std::fmt::{self, Display, Formatter};

use derive_more::*;

use crate::case_index::{case_range, CaseIndex};
use crate::data::{Group, UsizeTokenError};
use crate::tokens::Tokens;

#[derive(Debug)]
pub enum GlobalDataError<E: Error> {
    DataError(E),
    CountError(UsizeTokenError),
}

impl<E: Error> Display for GlobalDataError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::GlobalDataError::*;

        match self {
            DataError(err) => write!(f, "error loading global data: {}", err),
            CountError(err) => write!(f, "error loading number of test cases: {}", err),
        }
    }
}

impl<E: Error> Error for GlobalDataError<E> {
    fn cause(&self) -> Option<&Error> {
        use self::GlobalDataError::*;

        match self {
            DataError(err) => Some(err),
            CountError(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub struct GlobalData<T> {
    pub num_cases: usize,
    pub data: T,
}

impl<T> GlobalData<T> {
    pub fn for_each_case<'a, F, E>(self: &'a Self, mut f: F) -> Result<(), E>
    where
        F: FnMut(CaseIndex, &'a T) -> Result<(), E>,
    {
        case_range(self.num_cases).try_for_each(move |case| f(case, &self.data))
    }
}

pub trait LoadGlobalData: Sized {
    type Err: Error;

    fn from_tokens(
        tokens: &mut impl Tokens,
    ) -> Result<GlobalData<Self>, GlobalDataError<Self::Err>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoGlobalData;

impl LoadGlobalData for NoGlobalData {
    type Err = !;
    fn from_tokens(
        tokens: &mut impl Tokens,
    ) -> Result<GlobalData<Self>, GlobalDataError<Self::Err>> {
        tokens
            .next()
            .map_err(GlobalDataError::CountError)
            .map(|num_cases| GlobalData {
                num_cases,
                data: NoGlobalData,
            })
    }
}

impl LoadGlobalData for () {
    type Err = !;

    fn from_tokens(tokens: &mut impl Tokens) -> Result<GlobalData<()>, GlobalDataError<!>> {
        tokens
            .next()
            .map_err(GlobalDataError::CountError)
            .map(|num_cases| GlobalData {
                num_cases,
                data: (),
            })
    }
}

#[derive(Debug, Clone, Copy, From, Deref, DerefMut, Default)]
pub struct CountPrefix<T>(pub T);

impl<T> AsRef<T> for CountPrefix<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for CountPrefix<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Group> LoadGlobalData for CountPrefix<T> {
    type Err = T::Err;

    fn from_tokens(
        tokens: &mut impl Tokens,
    ) -> Result<GlobalData<Self>, GlobalDataError<Self::Err>> {
        let num_cases = tokens.next().map_err(GlobalDataError::CountError)?;
        let data = CountPrefix(tokens.next().map_err(GlobalDataError::DataError)?);

        Ok(GlobalData { num_cases, data })
    }
}

#[derive(Debug, Clone, Copy, From, Deref, DerefMut, Default)]
pub struct CountSuffix<T>(pub T);

impl<T> AsRef<T> for CountSuffix<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for CountSuffix<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Group> LoadGlobalData for CountSuffix<T> {
    type Err = T::Err;

    fn from_tokens(
        tokens: &mut impl Tokens,
    ) -> Result<GlobalData<Self>, GlobalDataError<Self::Err>> {
        let data = CountSuffix(tokens.next().map_err(GlobalDataError::DataError)?);
        let num_cases = tokens.next().map_err(GlobalDataError::CountError)?;

        Ok(GlobalData { num_cases, data })
    }
}
