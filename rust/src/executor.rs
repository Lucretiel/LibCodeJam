use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::process::exit;

use derive_more::From;

use crossbeam::{self, channel};

use crate::case_index::CaseIndex;
use crate::printer::Printer;
use crate::solver::Solver;
use crate::tokens::Tokens;

use crate::data::global::{GlobalDataError, LoadGlobalData};
use crate::data::group::Group;

#[derive(Debug)]
pub enum CaseErrorKind<E: Error> {
    Load(E),
    Print(io::Error),
}

#[derive(Debug)]
pub struct CaseError<E: Error> {
    case: CaseIndex,
    error: CaseErrorKind<E>,
}

impl<E: Error> CaseError<E> {
    #[inline(always)]
    pub fn new(case: CaseIndex, error: CaseErrorKind<E>) -> Self {
        CaseError { case, error }
    }

    #[inline(always)]
    pub fn load_error(case: CaseIndex, err: E) -> Self {
        CaseError::new(case, CaseErrorKind::Load(err))
    }

    #[inline(always)]
    pub fn print_error(case: CaseIndex, err: io::Error) -> Self {
        CaseError::new(case, CaseErrorKind::Print(err))
    }
}

impl<E: Error> Display for CaseError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.error {
            CaseErrorKind::Load(ref err) => {
                write!(f, "error loading data for {}: {}", self.case, err)
            }
            CaseErrorKind::Print(ref err) => {
                write!(f, "error writing solution to {}: {}", self.case, err)
            }
        }
    }
}

impl<E: Error> Error for CaseError<E> {
    fn cause(&self) -> Option<&Error> {
        match self.error {
            CaseErrorKind::Load(ref err) => Some(err),
            CaseErrorKind::Print(ref err) => Some(err),
        }
    }
}

#[derive(Debug, From)]
pub enum ExecutionError<E1: Error, E2: Error> {
    Global(GlobalDataError<E1>),
    Case(CaseError<E2>),
}

impl<E1: Error, E2: Error> ExecutionError<E1, E2> {
    #[inline(always)]
    pub fn global_error(err: GlobalDataError<E1>) -> Self {
        ExecutionError::Global(err)
    }

    #[inline(always)]
    pub fn load_error(case: CaseIndex, err: E2) -> Self {
        ExecutionError::Case(CaseError::load_error(case, err))
    }

    #[inline(always)]
    pub fn print_error(case: CaseIndex, err: io::Error) -> Self {
        ExecutionError::Case(CaseError::print_error(case, err))
    }
}

impl<E1: Error, E2: Error> Display for ExecutionError<E1, E2> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExecutionError::Global(err) => err.fmt(f),
            ExecutionError::Case(err) => err.fmt(f),
        }
    }
}

impl<E1: Error, E2: Error> Error for ExecutionError<E1, E2> {
    fn cause(&self) -> Option<&Error> {
        match self {
            ExecutionError::Global(err) => Some(err),
            ExecutionError::Case(err) => Some(err),
        }
    }
}

pub trait Executor {
    fn execute<T: Tokens + Send, P: Printer + Send, S: Solver + Sync>(
        tokens: T,
        printer: P,
        solver: S,
    ) -> Result<
        (),
        ExecutionError<<S::GlobalData as LoadGlobalData>::Err, <S::CaseData as Group>::Err>,
    >
    where
        S::CaseData: Group + Send,
        S::GlobalData: LoadGlobalData + Sync,
        S::Solution: Display + Send,
        <S::CaseData as Group>::Err: Send,
        <S::GlobalData as LoadGlobalData>::Err: Send;

    fn run<T: Tokens + Send, P: Printer + Send, S: Solver + Sync>(tokens: T, printer: P, solver: S)
    where
        S::CaseData: Group + Send,
        S::GlobalData: LoadGlobalData + Sync,
        S::Solution: Display + Send,
        <S::CaseData as Group>::Err: Send,
        <S::GlobalData as LoadGlobalData>::Err: Send,
    {
        Self::execute(tokens, printer, solver).unwrap_or_else(|err| {
            eprintln!("Error solving code jam: {}", err);
            exit(1);
        })
    }
}

pub struct SequentialExecutor;

impl Executor for SequentialExecutor {
    fn execute<T: Tokens + Send, P: Printer + Send, S: Solver + Sync>(
        mut tokens: T,
        mut printer: P,
        solver: S,
    ) -> Result<
        (),
        ExecutionError<<S::GlobalData as LoadGlobalData>::Err, <S::CaseData as Group>::Err>,
    >
    where
        S::CaseData: Group + Send,
        S::GlobalData: LoadGlobalData + Sync,
        S::Solution: Display + Send,
        <S::CaseData as Group>::Err: Send,
        <S::GlobalData as LoadGlobalData>::Err: Send,
    {
        tokens
            .start_problem()?
            .for_each_case(move |case, global_data| {
                let case_data = tokens
                    .next()
                    .map_err(|err| ExecutionError::load_error(case, err))?;
                let solution = solver.solve_case(global_data, case_data);
                printer
                    .print_solution(case, solution)
                    .map_err(|err| ExecutionError::print_error(case, err))
            })
    }
}

pub struct ThreadExecutor;

impl Executor for ThreadExecutor {
    fn execute<T: Tokens + Send, P: Printer + Send, S: Solver + Sync>(
        mut tokens: T,
        mut printer: P,
        solver: S,
    ) -> Result<
        (),
        ExecutionError<<S::GlobalData as LoadGlobalData>::Err, <S::CaseData as Group>::Err>,
    >
    where
        S::CaseData: Group + Send,
        S::GlobalData: LoadGlobalData + Sync,
        S::Solution: Display + Send,
        <S::CaseData as Group>::Err: Send,
        <S::GlobalData as LoadGlobalData>::Err: Send,
    {
        let global_data = tokens.start_problem()?;
        let solver = &solver;

        crossbeam::scope(|scope| {
            let (sender, receiver) = channel::bounded(global_data.num_cases);

            // Spawn a print thread which will do all the printing, bailing on an error.
            let print_thread = scope.spawn(move || {
                // Solutions may arrive in any order; collect them into a hash table
                let mut solutions = HashMap::new();
                let mut next_case = CaseIndex::default();

                for (case, solution) in receiver {
                    if case == next_case {
                        next_case = printer
                            .print_advance(case, solution)
                            .map_err(move |err| (case, err))?;

                        while let Some(solution) = solutions.remove(&next_case) {
                            next_case = printer
                                .print_advance(next_case, solution)
                                .map_err(move |err| (case, err))?;
                        }
                    } else {
                        solutions.insert(case, solution);
                    }
                }
                Ok(())
            });

            // Start spawning test cases
            global_data.for_each_case(move |case, global_data| {
                // Can't use ? here, because the chain of ? confuses the type inferrer.
                let case_data = match tokens.next() {
                    Err(err) => return Err(CaseError::load_error(case, err)),
                    Ok(case_data) => case_data,
                };

                let local_sender = sender.clone();

                scope.spawn(move || {
                    let solution = solver.solve_case(global_data, case_data);
                    local_sender.send((case, solution));
                });

                Ok(())
            })?;

            print_thread
                .join()
                .expect("Print thread panicked!")
                .map_err(|(case, err)| CaseError::print_error(case, err))?;
            Ok(())
        })
    }
}
