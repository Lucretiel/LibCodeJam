use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct SpacePair<A, B> {
    first: A,
    second: B,
}

impl<A, B> SpacePair<A, B> {
    #[inline(always)]
    pub fn new(first: A, second: B) -> Self {
        SpacePair{first, second}
    }
}

impl<A: Display, B: Display> Display for SpacePair<A, B> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.first.fmt(f)?;
        ' '.fmt(f)?;
        self.second.fmt(f)
    }
}

#[macro_export]
macro_rules! space_sep {
    ($head:expr) => ($head);
    ($head:expr $(, $tail:expr)+) => ($crate::helpers::SpacePair::new($head, space_sep!($($tail),*)))
}

