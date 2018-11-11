use std::fmt::Display;
use std::io;

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
        pub struct $printer<W: std::io::Write>(pub std::io::BufWriter<W>);

        impl<W: std::io::Write> $printer<W> {
            pub fn new(writer: W) -> Self {
                $printer(std::io::BufWriter::new(writer))
            }
        }

        impl $printer<std::io::Stdout> {
            pub fn stdout() -> Self {
                Self::new(io::stdout())
            }
        }

        impl<W: std::io::Write> Printer for $printer<W> {
            fn print_solution(&mut self, case: CaseIndex, solution: impl Display) -> io::Result<()> {
                use std::io::Write;
                writeln!(self.0, $pattern, case=case, solution=solution)?;
                self.0.flush()
            }
        }
    )*)
}

printer_pattern! {
    StandardPrinter: "{case}: {solution}";
    NewlinePrinter: "{case}:\n{solution}";
}
