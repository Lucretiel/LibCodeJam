#![feature(try_trait)]
#![feature(trusted_len)]
#![feature(never_type)]

pub mod case_index;
pub mod data;
pub mod executor;
pub mod printer;
pub mod solver;
pub mod tokens;
pub mod helpers;

pub use ordered_float::{OrderedFloat, NotNan};
