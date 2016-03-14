LibCodeJam
==========

Helper library for Google Code Jam, implemented in various languages.

Preview
-------

Let's say there's a sample problem. The input file is **T**, the number of test cases, followed by **T** lines. Each line in **N**, the number of values, followed by **N** ints. The output should be **T** lines, formatted "Case #*x*: *y*" where *x* is the test case, starting from 1, and *y* is the sum of the ints.

In plain python, this looks like:

```python
import sys

istr = sys.stdin

num_cases = int(next(istr))
for i in range(num_cases):
    line = next(istr)
    values = [int(x) for x in line.split()[1:]]
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

The `@autosolve` decorator sets up the solution function. It automatically sets up file parsing and output formatting. It reads the first token, **T**, and then calls your solver function **T** times, writing the return values. The `@collects` decorator sets up some magic token inputting– it examines your function signature, and supplies your function with tokens or lists of tokens based on the annotations. This allows you to focus on just writing a function to solve the problem.

`@autosolve` also sets up optional file writing, via the first and second positional parameters (`infile` and `outfile`), as well as an optional progress bar with `-p`.
