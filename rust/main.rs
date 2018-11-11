#[macro_use]
extern crate libcodejam;

use libcodejam::data::*;
use libcodejam::executor::*;
use libcodejam::printer::*;
use libcodejam::solver::*;
use libcodejam::tokens::*;

struct_group!{
    struct Data {
    num_columns: usize,
    ball_counts: Vec<usize> => num_columns,
}}

fn main() {
    ThreadExecutor::run(
        TokensReader::stdin(),
        StandardPrinter::stdout(),
        solver(|_data: Data| {
            "Done"
        }),
    );
}
