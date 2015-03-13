# This code jam solution is powered by Nathan West's LibCodeJam; see
# https://github.com/Lucretiel/LibCodeJam for source code and (ostensibly) some
# documentation.

from code_jam import autosolve, collects, cases


# Uncomment if you want a newline between the "Case #" and the actual solution
#code_jam.INSERT_NEWLINE = True

# Example:
#
#    Case #1: solution
#    Case #2: solution
#
# vs
#
#    Case #1:
#    solution
#    Case #2:
#    solution
#

@autosolve
@collects
def solve(tokens):
    '''
    Solve a single test case using the tokens, and return the solution. If
    this code jam problem requires shared state between the test cases (for
    instance, word dictionary of the 2009 Qualification Problem A: Alien
    Language), this function should be a generator which yields each solution.
    Otherwise, it should return a single solution; @autosolve will call it the
    correct number of times.

    Running this file will solve a code jam problem, taking the input file from
    stdin and writing the solution to stdout. The input is split into tokens
    using the Tokens class; a token is simply a whitespace-delimited string,
    such as via str.split. The tokens object makes no distinction between tokens
    on the same line or different lines. It has the following methods:

    tokens.next_token(type) -> get the next token, converting it to type
    tokens.next_many(n, type) -> yield the next n tokens
    tokens.next_counted(type) -> get the next int, then yield that many tokens

    These methods have the aliases t, m, and c, respectively.

    Optionally, add additional parameters to the solve_case function. One token
    will be extracted for each parameter besides tokens, in order, and passed
    in as additional arguments. Use a type annotation to specify the tokens's
    type; this will be passed into tokens.next_token (defaulting to str). Remove
    the tokens parameter if you won't need to extract any extra tokens.
    '''
