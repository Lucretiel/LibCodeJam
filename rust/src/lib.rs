#![feature(try_trait)]
#![feature(trusted_len)]
#![feature(never_type)]

#[macro_use]
extern crate derive_more;
extern crate crossbeam;

pub mod data;
pub mod printer;
pub mod solver;
pub mod tokens;
