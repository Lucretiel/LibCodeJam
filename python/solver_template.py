# This code jam solution is powered by Nathan West's LibCodeJam; see
# https://github.com/Lucretiel/LibCodeJam for source code and (ostensibly) some
# documentation.

from code_jam import autosolve, collects_tokens


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
@collects_tokens
def solve_case(tokens):
    '''
    Solve a single test case using the tokens, and return the solution. If
    this code jam problem requires shared state between the test cases (for
    instance, word dictionary of the 2009 Qualification Problem A: Alien
    Language), this function should be a generator which yields each solution.
    Otherwise, it should return a single solution; @autosolve will call it the
    correct number of times.

    A token is simply a whitespace-delimited string, such as via str.split. The
    tokens object makes no distinction between tokens on the same line or
    different lines.

    Running this file will solve a code jam problem, taking the input file from
    stdin and writing the solution to stdout. The tokens object has the
    following methods:

    tokens.next_token(type) -> get the next token, converting it to type
    tokens.next_many_tokens(n, type) -> yield the next n tokens
    tokens.next_counted_tokens(type) -> get the next int token, then yield that
        many tokens

    These methods have the aliases n, m, and c, respectively.

    Optionally, add additional parameters to the solve_case function. One token
    will be extracted for each parameter besides tokens, in order, and passed
    in as additional arguments. Use a type annotation to specify the tokens's
    type; this will be passed into. Remove the tokens parameter if you won't
    need to extract any extra tokens.
    '''