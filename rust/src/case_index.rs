use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseIndex(usize);

impl CaseIndex {
    pub fn next(self) -> CaseIndex {
        CaseIndex(self.0 + 1)
    }

    pub fn start() -> CaseIndex {
        CaseIndex(1)
    }
}

impl Default for CaseIndex {
    fn default() -> CaseIndex {
        CaseIndex::start()
    }
}

impl Display for CaseIndex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Case #{}", self.0)
    }
}

pub fn case_range(num_cases: usize) -> impl Iterator<Item = CaseIndex> {
    (1..=num_cases).map(CaseIndex)
}

pub fn cases() -> impl Iterator<Item = CaseIndex> {
    (1..).map(CaseIndex)
}
