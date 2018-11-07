use std::fmt::Display;
use std::io::{self, Write};

use crate::case_index::CaseIndex;

pub trait Printer {
    fn print_solution(&mut self, case: CaseIndex, solution: impl Display) -> io::Result<()>;

    fn print_advance(&mut self, case: CaseIndex, solution: impl Display) -> io::Result<CaseIndex> {
        self.print_solution(case, solution)?;
        Ok(case.next())
    }
}

macro_rules! printer_pattern {
	($($printer:ident : $pattern:expr ;)+) => ($(
        #[derive(Debug)]
        pub struct $printer<W: Write>(pub io::BufWriter<W>);

        impl<W: Write> $printer<W> {
            pub fn new(writer: W) -> Self {
                $printer(io::BufWriter::new(writer))
            }
        }

        impl<W: Write> Printer for $printer<W> {
            fn print_solution(&mut self, case: CaseIndex, solution: impl Display) -> io::Result<()> {
                write!(self.0, $pattern, case=case, solution=solution)?;
                self.0.flush()
            }
        }
    )*)
}

printer_pattern! {
    StandardPrinter: "{case}: {solution}\n";
    NewlinePrinter: "{case}:\n{solution}\n";
}
