#[macro_use]
extern crate libcodejam;

use libcodejam::data::group::*;
use libcodejam::executor::*;
use libcodejam::printer::*;
use libcodejam::solver::*;
use libcodejam::tokens::*;

struct_group!{struct Data {
    num_columns: usize,
    ball_counts: Vec<usize> => num_columns,
}}

fn main() {
    ThreadExecutor::run(
        TokensReader::new_buf(std::io::stdin()),
        StandardPrinter::new(std::io::stdout()),
        solver(|data: u64| -> &'static str {
            std::thread::sleep(std::time::Duration::from_secs(data));
            "Done"
        }),
    );
}
