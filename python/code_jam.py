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

from signal import signal, SIGPIPE, SIG_DFL
from contextlib import contextmanager

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
        Read a single token of type `t`
        '''
        return t(next(self.tokens))

    def next_many_tokens(self, n, t):
        '''
        Yield the next `n` tokens of type `t`
        '''
        for _ in range(n):
            yield self.next_token(t)

    def next_counted_tokens(self, t):
        '''
        Read a token n, then yield n tokens of type `t`.
        '''
        return self.next_many_tokens(self.next_token(int), t)


def print_cases(solutions, ostr):
    '''
    Format and print the solutions of a code jam to the file object `ostr`.
    `solutions` should be an ordered iterable of solutions. Prints using the
    standard "Case #1: X" formatting. If code_jam.INSERT_NEWLINE is True, a
    newline is printed before each solution. The solution is printed with the
    standard print functions, so any stringable type is fine.

    This function silently stops and returns in the event of a BrokenPipeError
    from either the input or output file.

    Output is unconditionally flushed on every solution. This is to enable
    real-time output even when in a pipeline, such as with head or tee.
    '''
    sep = '\n' if INSERT_NEWLINE else ' '
    format_case = "Case #{}:".format

    try:
        for case, solution in enumerate(solutions, 1):
            print(format_case(case), solution, sep=sep, file=ostr, flush=True)
    except BrokenPipeError:
        signal(SIGPIPE, SIG_DFL)


def generator_solve_code_jam(solver, istr, ostr):
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


def solve_code_jam(solver, istr, ostr):
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
    def solve(tokens):
        for _ in range(tokens.next_token(int)):
            yield solver(tokens)

    return generator_solve_code_jam(solve, istr, ostr)

@contextmanager
def smart_open(filename, *args, **kwargs):
    '''
    Context manager to open and close a file, or default to a file object. If
    the filename argument isn't a string (for instance, if you pass stdin or
    stdout), it yields the object directly.
    '''
    if isinstance(filename, (str, bytes, int)):
        with open(filename, *args, **kwargs) as f:
            yield f
    else:
        yield filename

def autosolve(solver):
    '''
    Decorator to immediatly solve a code jam with a function when the file is
    run as a script. It should decorate a function which, when called with a
    Tokens object, returns a solution to a single test case. The code jam is
    then immediatly solved by assuming the first token is the number of test
    cases, and repeatedly calling the decorated function to retreive solutions.
    Doesn't respect __name__ == '__main__'.

    If the decorated function is a generator, the behavior is slightly diffent.
    The generator is called with the Tokens object, and each yielded solution
    is printed. The generator is responsible for yielding the correct number of
    solutions.

    autosolve also collects filenames from sys.argv. The first command line
    argument, if given, is the input file, and the second, if given, is the
    output file. These default to stdin and stdout, respectively.

    The decorated function is returned unchanged.
    '''

    from sys import stdin, stdout
    from argparse import ArgumentParser
    from inspect import isgeneratorfunction

    parser = ArgumentParser()
    parser.add_argument('in_file', nargs='?', default=stdin,
        help="The input file to use. Defaults to stdin")
    parser.add_argument('out_file', nargs='?', default=stdout,
        help="The file to write the solutions to. Defaults to stdout.")
    args = parser.parse_args()

    with smart_open(args.in_file, 'r', encoding='ascii') as istr:
        with smart_open(args.out_file, 'w', encoding='ascii') as ostr:
            if isgeneratorfunction(solver):
                generator_solve_code_jam(solver, istr, ostr)
            else:
                solve_code_jam(solver, istr, ostr)

    return solver

# TODO: Windows (sometimes) defaults to UTF-16 or some other ascii-incompatible
# format on stdout. Force autosolve to make output to be ascii when >redirecting
# to a file.
