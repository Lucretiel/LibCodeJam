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

from __future__ import print_function
from sys import stdin, stdout
from signal import signal, SIGPIPE, SIG_DFL


class Tokens:
    '''
    Helper class to read in tokens, either individually or in groups. A token
    is simply a whitespace-delimited group of characters.
    '''
    @staticmethod
    def tokenize(stream):
        '''
        Break a stream into whitespace-separated tokens
        '''
        for line in stream:
            for token in line.split():
                yield token

    def __init__(self, stream):
        self.tokens = self.tokenize(stream)

    def __iter__(self):
        return self

    def __next__(self):
        return next(self.tokens)

    # next method for python 2 compatibility
    next = __next__

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


def generic_solve_code_jam(solver, istr, ostr, insert_newline=False):
    '''
    Print the solution of a code jam to the file object `ostr`, given an input
    file object `istr`. `solver` is a generator or function that takes a Tokens
    object and yields solutions or returns a list of solutions. Handles
    formatting the output correctly, using the standard code jam "Case #1: X"
    formatting. If `insert_newline` is True, a newline is printed before the
    solution ("Case #1:\nX"). The solution is outputted via a normal print, so
    returning strings, ints, or other printable types is fine.

    This function also silently returns in the event of a BrokenPipeError from
    either the input or output file.
    '''

    format_case = "Case #{}:".format
    sep = '\n' if insert_newline else ' '

    try:
        for case, solution in enumerate(solver(Tokens(istr)), 1):
            print(format_case(case), solution, sep=sep, file=ostr, flush=True)
    except BrokenPipeError:
        signal(SIGPIPE, SIG_DFL)


def solve_code_jam(solver, istr, ostr, insert_newline=False):
    '''
    For a code jam where the first token is the number of cases, this function
    prints the solution to the file object `ostr`, given an input file object
    `istr`. In this variant, the solver is a function which is called with a
    Tokens object and returns a single solution. This is the most typical use
    case, as most code jams don't need to share any data.
    '''
    def solve(tokens):
        for _ in range(tokens.next_token(int)):
            yield solver(tokens)

    generic_solve_code_jam(solve, istr, ostr, insert_newline)


def autosolve(func=None, *, insert_newline=False, generic=False):
    '''
    Decorator to immediatly solve a code jam with a function, from stdin and
    stdout. Doesn't respect __name__ == '__main__'. Can be used with or without
    arguments:

        @autosolve
        def solver(tokens):
            code code code

        @autosolve(insert_newline=True, ...)
        def solver(tokens):
            code code code
    '''
    solve = generic_solve_code_jam if generic else solve_code_jam
    def decorator(solver):
        solve(solver, stdin, stdout, insert_newline)
        return solver

    return decorator(func) if func else decorator

# TODO: Windows (sometimes) defaults to UTF-16 or some other ascii-incompatible
# format when redirecting to a file. Force autosolve to make output to be UTF-8
# or ascii on windows.
