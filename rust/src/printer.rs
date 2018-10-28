use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub struct CasePrintError {
    case: u32,
    error: io::Error,
}

impl Display for CasePrintError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Error writing solution to Case #{}: {}",
            self.case, self.error
        )
    }
}

impl Error for CasePrintError {
    fn cause(&self) -> Option<&Error> {
        self.error.cause()
    }
}

pub trait Printer: Sized {
    fn print_solution(&mut self, case: u32, solution: impl Display) -> io::Result<()>;

    fn print_solutions<I>(&mut self, solutions: I) -> Result<(), CasePrintError>
    where
        I: IntoIterator,
        I::Item: Display,
    {
        solutions
            .into_iter()
            .zip(1..)
            .try_for_each(move |(solution, case)| {
                self.print_solution(case, solution)
                    .map_err(|error| CasePrintError { case, error })
            })
    }
}

macro_rules! printer_pattern {
	($($printer:ident : $pattern:expr ;)+) => ($(
        #[derive(Debug)]
        pub struct $printer<W: io::Write>(pub W);

        impl<W: io::Write> Printer for $printer<W> {
            fn print_solution(&mut self, case: u32, solution: impl Display) -> io::Result<()> {
                writeln!(self.0, $pattern, case=case, solution=solution)?;
                self.0.flush()
            }
        }
    )*)
}

printer_pattern! {
    StandardPrinter: "Case #{case}: {solution}";
    NewlinePrinter: "Case #{case}:\n{solution}";
}
