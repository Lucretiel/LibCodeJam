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
formatting. Source-compatible with python 2 and 3.
'''

from __future__ import print_function
from sys import stdin, stdout


class Tokens:
    '''
    Helper class to read in tokens, either individually or in groups.
    '''
    @staticmethod
    def tokenize(stream):
        '''
        Break a stream into whitespace-separated tokens
        '''
        for line in stream:
            for token in line.split():
                yield token

    def __init__(self, stream=stdin):
        self.tokens = self.tokenize(stream)

    def __iter__(self):
        return self

    def __next__(self):
        return next(self.tokens)

    def next_token(self, type):
        '''Read a single token of type `type`'''
        return type(next(self))

    def gen_many_tokens(self, n, type):
        '''
        Get a generator for the next n tokens
        '''
        for _ in range(n):
            yield self.next_token(type)

    def next_many_tokens(self, n, type, collection):
        '''
        Read a group of tokens, and store them in a collection.
        '''
        return collection(self.gen_many_tokens(n, type))


def generic_solve_code_jam(solver, num_cases, ostr=stdout):
    '''
    Output the solution of a code jam to `ostr`. The jam consists of `num_cases`
    cases, and each case is solved by a call to solver with no arguments. This
    function handles formatting the output correctly, using the standard code
    jam "Case #1: x" formatting.
    '''
    case_line = "Case #{}".format
    for case in range(num_cases):
        print(case_line(case + 1), solver(), file=ostr)


def solve_code_jam(solver, istr=stdin, ostr=stdout):
    '''
    For a code jam where the first token is the number of cases, this function
    outputs the solution, as with generic_solve_code_jam. In this variant, the
    solver is called with the created Tokens object each time.
    '''
    tokens = Tokens(istr)
    generic_solve_code_jam(
        (lambda: solver(tokens)),
        tokens.next_token(int),
        ostr)

