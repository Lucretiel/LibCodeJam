LibCodeJam
==========

Helper library for Google Code Jam, implemented in various languages.

Preview
-------

Let's say there's a sample problem. The input file is **T**, the number of test cases, followed by **T** lines. Each line in **N**, the number of values, followed by **N** ints. The output should be **T** lines, formatted "Case #*x*: *y*" where *x* is the test case, starting from 1, and *y* is the sum of the ints.

In plain python, this looks like:

```python
import sys

def tokens():
    for line in sys.stdin:
        for token in line.split():
            yield token
tokens = tokens()

num_cases = int(next(tokens))
for i in range(num_cases):
    num_values = next(tokens)
    values = [int(next(token)) for _ in range(num_values)]
    print("Case #{}: {}".format(i+1, sum(values)))
```

Quick, but the solution algorithm is mixed with input parsing and output formatting in an ugly way. LibCodeJam handles all that in a neat way:

```python
from code_jam import *

@autosolve
@collects
def solve(N: int,
          values: ('N', int)):
    return sum(values)
```

The `@autosolve` decorator sets up the solution function. It automatically sets up file parsing and output formatting. It reads the first token, **T**, and then calls your solver function **T** times, writing the return values. The `@collects` decorator sets up some magic token inputting– it examines your function signature, and supplies your function with tokens or lists of tokens based on the annotations. This allows you to focus on just writing a function to solve the problem. The input file is read from stdin and the solution is written to stdout.

Some code jams have some global data, shared between test cases. `@autosolve` handles that as well, using the `@cases` nested solver helper. For instance, let's say there's a problem where the input is a line with the values **T** and **X**. On the following **T** lines is a single int **N**. The soltion to test case *n* is **N<sub>n</sub>** + **X**. Here's the LibCodeJam solution:

```python
from code_jam import *

@autosolve
@collects
def solve(T: int, X: int, tokens):
    @cases(T)
    @collects
    def solve_case(N: int):
        return N + X

    # The `yield from` indicates to @autosolve that this function solves a whole problem, not a single test case.
    yield from solve_case(tokens)
```
