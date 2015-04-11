#
# Copyright 2014 Nathan West
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#

'''
Utility library for solving code jams. Handles input tokenization and output
formatting.
'''

from sys import stdin, stdout, stderr
from argparse import ArgumentParser
from contextlib import contextmanager, suppress
from inspect import isgeneratorfunction
from functools import wraps


__all__ = []
def export(thing):
    __all__.append(thing.__name__)
    return thing


# Set this variable to true in your code to force printing newlines between
# "Case #" and the solution itself
INSERT_NEWLINE = False


class Tokens:
    '''
    Helper class to read in tokens, either individually or in groups. A token
    is simply a whitespace-delimited group of characters.
    '''
    @staticmethod
    def tokenize(istr):
        '''
        Break a stream into whitespace-separated tokens
        '''
        for line in istr:
            for token in line.split():
                yield token

    def __init__(self, stream):
        self.tokens = self.tokenize(stream)

    def next_token(self, t):
        '''
        Read and return single token of type `t`
        '''
        return t(next(self.tokens))

    def next_group(self, *types):
        '''
        Return tuple of tokens of each of the given types, in order:

        name, age = tokens.next_group(str, int)
        '''

        return tuple(self.next_token(t) for t in types)

    def next_many(self, n, t):
        '''
        Yield the next `n` tokens of type `t`.
        '''
        for _ in range(n):
            yield self.next_token(t)

    t = next_token
    m = next_many
    g = next_group


@export
def collects(func):
    '''
    This decorator allows a function to collect tokens. The function's
    signature is changed to accept a single Tokens instance. For each of the
    function's parameters with a type annotation, a token is extracted from the
    tokens instance and passed as an argument, with a type matching the
    annotation. Any other parameter is simply passed the Tokens instance
    instead of a new token.

    Example:

        @collects
        def solve(a: int, b: int, s: str, tokens):
            return a + b

        # This is the same as:
        def solve(_tokens):
            a = _tokens.next_token(int)
            b = _tokens.next_token(int)
            s = _tokens.next_token(str)
            tokens = _tokens
            return a + b

    It is designed to be used with the autosolve decorator, like so:

        @autosolve
        @collects
        def solve(...):
            ....
    '''
    from inspect import signature
    params = tuple(signature(func).parameters.values())

    def collect_token_args(tokens):
        for param in params:
            if param.annotation is not param.empty:
                yield tokens.next_token(param.annotation)
            else:
                yield tokens

    def collect_wrapper(tokens):
        return func(*collect_token_args(tokens))

    # solve_code_jam uses this flag to determine if a solver is a generator or
    # a per-case function.
    collect_wrapper._gen = isgeneratorfunction(func)
    return collect_wrapper


def progress(i, n):
    if progress.enabled:
        print(
            '\r' + ('@' * i).ljust(n, '-'),
            end='' if i != n else '\n',
            file=stderr)


progress.enabled = False


@export
def cases(n):
    '''
    This decorator helps with writing generator solvers. When applied to a
    function, it wraps the function in a generator which calls the underlying
    function with the arguments n times, yielding the return values. The intent
    is to make it possible to use @collects within a generator solver.

    Example:

        @autosolve
        @collects
        def solve_problem(n: int, x: int, tokens):
            """for each case, add x to the int for that case"""
            @cases(n)
            @collects
            def solve_case(a: int):
                return a + x

            yield from solve_case(tokens)

    It also prints a progress bar to stderr, which can be enabled with the -p
    option at the command line or by setting code_jam.progress.enabled to True
    '''
    def decorator(func):
        def cases_wrapper(*args, **kwargs):
            for i in range(n):
                progress(i, n)
                yield func(*args, **kwargs)
            progress(n, n)
        return cases_wrapper
    return decorator


def print_cases(solutions, ostr):
    '''
    Format and print the solutions of a code jam to the file object `ostr`.
    `solutions` should be an ordered iterable of solutions. Prints using the
    standard "Case #1: X" formatting. If code_jam.INSERT_NEWLINE is True, a
    newline is printed before each solution.

    This function silently stops and returns in the event of a BrokenPipeError
    from either the input or output file.

    Output is unconditionally flushed on every solution. This is to enable
    real-time output even when in a pipeline, such as with head or tee.
    '''
    sep = '\n' if INSERT_NEWLINE else ' '
    format_case = "Case #{}:".format

    with suppress(BrokenPipeError):
        for case, solution in enumerate(solutions, 1):
            print(format_case(case), solution, sep=sep, file=ostr, flush=True)


def generator_solve(solver, istr, ostr):
    '''
    Print the solution of a code jam to the file object `ostr`, given an input
    file object `istr`. `solver` is a generator that takes a Tokens object and
    yields solutions to each case. Handles formatting the output correctly,
    using the standard code jam "Case #1: X" formatting. The solution is
    outputted via a normal print, so returning strings, ints, or other
    printable types is fine.

    This function silently returns in the event of a BrokenPipeError from
    either the input or output file.

    Output is unconditionally flushed on every solution. This is to enable
    real-time output even when in a pipeline, such as with head or tee.
    '''

    return print_cases(solver(Tokens(istr)), ostr)


def function_solve(solver, istr, ostr):
    '''
    For a typical, code jam where the first token is the number of cases, this
    function prints the solution to the code to the file object `ostr`, given
    an input file object `istr`. `solver` should be a function that takes a
    Tokens object and returns the solution to a single test case.
    `solve_code_jam` reads the first token to determine the number of cases,
    then calls solver repeatedly to get each solution. This is the most common
    use case, as most code jams don't share any data between test cases.

    Like the other wrappers, this function silently returns in the event of a
    BrokenPipeError from either the input or output file, and flushes the
    output for each case.
    '''
    def func_solve_wrapper(tokens):
        return cases(tokens.next_token(int))(solver)(tokens)

    generator_solve(func_solve_wrapper, istr, ostr)


def solve_code_jam(solver, istr, ostr):
    '''
    Solve a code jam using either a function or a generator, based on solver's
    type. See function_solve and generator_solve. If the function has the _gen
    attribute and it is True, it is assumed to be a wrapper around a generator.
    '''

    return (generator_solve
        if getattr(solver, '_gen', False) or isgeneratorfunction(solver)
        else function_solve)(solver, istr, ostr)


@contextmanager
def smart_open(filename, *args, **kwargs):
    '''
    Context manager to open and close a file, or default to a file object. If
    the filename argument isn't a string (for instance, if you pass stdin or
    stdout), it yields the object directly.
    '''
    if isinstance(filename, (str, bytes, int)):
        with open(filename, *args, **kwargs) as file:
            yield file
    else:
        yield filename


@export
def autosolve(solver):
    '''
    Decorator to immediately solve a code jam with a function when the file is
    run as a script. It should decorate a function which, when called with a
    Tokens object, returns a solution to a single test case. The code jam is
    then immediately solved by assuming the first token is the number of test
    cases, and repeatedly calling the decorated function to retrieve solutions.
    Doesn't respect __name__ == '__main__'.

    If the decorated function is a generator, the behaviour is slightly
    different. The generator is called with the Tokens object, and each yielded
    solution is printed. The generator is responsible for yielding the correct
    number of solutions.

    autosolve also collects filenames from the command line arguments. The
    first argument, if given, is the input file, and the second, if given, is
    the output file. These default to stdin and stdout, respectively.

    The decorated function is returned unchanged.

    Designed to be combined with the collects decorator:

        @autosolve
        @collects
        def solve(A: int, B: int, tokens):
            ...
    '''

    parser = ArgumentParser()
    parser.add_argument('in_file', nargs='?', default=stdin,
        help="The input file to use. Defaults to stdin")
    parser.add_argument('out_file', nargs='?', default=stdout,
        help="The file to write the solutions to. Defaults to stdout.")
    parser.add_argument('--progress', '-p', action='store_true',
        help="Enable printing a progress bar to stderr.")
    args = parser.parse_args()

    progress.enabled = args.progress

    with smart_open(args.in_file, 'r', encoding='ascii') as istr:
        with smart_open(args.out_file, 'w', encoding='ascii') as ostr:
            solve_code_jam(solver, istr, ostr)

    return solver
# TODO: Windows (sometimes) defaults to UTF-16 or some other ascii-incompatible
# format on stdout. Force autosolve to make output be ascii when >redirecting
# to a file.


@export
def debug(*args, **kwargs):
    '''print to stderr'''
    return print(*args, file=stderr, **kwargs)


@export
def apply(t):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            return t(func(*args, **kwargs))
        return wrapper
    return decorator
