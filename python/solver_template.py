# This code jam solution is powered by Nathan West's LibCodeJam; see
# https://github.com/Lucretiel/LibCodeJam for source code and (ostensibly) some
# documentation.

from code_jam import autosolve

# Uncomment if you want a newline between the "Case #" and the actual solution:
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

#code_jam.INSERT_NEWLINE = True

@autosolve
def solve_case(tokens):
    '''
    Solve a single test case using the tokens, and return the solution. If
    this code jam problem requires shared state between the test cases (for
    instance, word dictionary of the 2009 Qualification Problem A: Alien
    Language), this function should be a generator which yields each solution.
    Otherwise, it should return a single solution; @autosolve will call it the
    correct number of times.
    
    This will solve a code jam, taking the input file from stdin and writing the
    solution to stdout. The tokens object has the follwing methods:
    
    tokens.next_token(type) -> get the next token, converting it to type
    tokens.next_many_tokens(n, type) -> yield the next n tokens
    tokens.next_counted_tokens(type) -> get the next int token, then yield that
        many tokens
    
    A token is simply a whitespace-delimited string, such as via str.split.
    '''
