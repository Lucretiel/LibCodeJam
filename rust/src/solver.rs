use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

pub trait Solver {
    type GlobalData;
    type CaseData;
    type Solution;

    fn solve_case(
        &self,
        global_data: &Self::GlobalData,
        case_data: Self::CaseData,
    ) -> Self::Solution;
}

pub struct FnSolver<F: Fn(CD) -> S, CD, S> {
    solver_fn: F,
    case_phantom: PhantomData<CD>,
    solution_phantom: PhantomData<S>,
}

impl<CD, S, F: Fn(CD) -> S> Solver for FnSolver<F, CD, S> {
    type GlobalData = ();
    type CaseData = CD;
    type Solution = S;

    fn solve_case(&self, _global: &(), case_data: CD) -> S {
        (self.solver_fn)(case_data)
    }
}

#[derive(Debug)]
pub struct GlobalFnSolver<F: Fn(&GD, CD) -> S, GD, CD, S> {
    solver_fn: F,
    global_phantom: PhantomData<GD>,
    case_phantom: PhantomData<CD>,
    solution_phantom: PhantomData<S>,
}

impl<GD, CD, S, F: Fn(&GD, CD) -> S> Solver for GlobalFnSolver<F, GD, CD, S> {
    type GlobalData = GD;
    type CaseData = CD;
    type Solution = S;

    fn solve_case(&self, global_data: &GD, case_data: CD) -> S {
        (self.solver_fn)(global_data, case_data)
    }
}

#[derive(Debug)]
pub enum MaybeImpossibleSolution<T> {
    Success(T),
    Failure(&'static str),
}

impl<T: Display> Display for MaybeImpossibleSolution<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::MaybeImpossibleSolution::*;

        match self {
            Success(ref s) => s.fmt(f),
            Failure(msg) => f.write_str(msg),
        }
    }
}

#[derive(Debug)]
pub struct MaybeImpossibleSolver<S, T: Solver<Solution = Option<S>>> {
    underlying: T,
    message: &'static str,
}

impl<S, T: Solver<Solution = Option<S>>> Solver for MaybeImpossibleSolver<S, T> {
    type GlobalData = T::GlobalData;
    type CaseData = T::CaseData;
    type Solution = MaybeImpossibleSolution<S>;

    fn solve_case(
        &self,
        global_data: &Self::GlobalData,
        case_data: Self::CaseData,
    ) -> Self::Solution {
        use self::MaybeImpossibleSolution::*;

        match self.underlying.solve_case(global_data, case_data) {
            Some(solution) => Success(solution),
            None => Failure(self.message),
        }
    }
}

pub trait IntoMaybeSolver<T>: Solver<Solution = Option<T>> + Sized {
    fn or_else(self, message: &'static str) -> MaybeImpossibleSolver<T, Self> {
        MaybeImpossibleSolver {
            underlying: self,
            message,
        }
    }
}

impl<T, S> IntoMaybeSolver<S> for T where T: Solver<Solution = Option<S>> {}

pub fn solver<CD, S, F: Fn(CD) -> S>(solver_fn: F) -> FnSolver<F, CD, S> {
    FnSolver {
        solver_fn,
        case_phantom: PhantomData,
        solution_phantom: PhantomData,
    }
}

pub fn global_solver<GD, CD, S, F: Fn(&GD, CD) -> S>(solver_fn: F) -> GlobalFnSolver<F, GD, CD, S> {
    GlobalFnSolver {
        solver_fn,
        global_phantom: PhantomData,
        case_phantom: PhantomData,
        solution_phantom: PhantomData,
    }
}
