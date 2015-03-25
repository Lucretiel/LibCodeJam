/*
This code jam solution is powered by Nathan West's LibCodeJam; see
https://github.com/Lucretiel/LibCodeJam for source code and (ostensibly) some
documentation.
*/

/*
Uncomment if you want a newline between the "Case #" and the actual solution:

    Case #1: solution
vs
	Case #1:
	solution

*/
//#define INSERT_NEWLINE

#include "LibCodeJam/cpp/code_jam.hpp"

/*
SOLVER creates a class which stores any data that needs to be shared between
cases. PRE_SOLVE initializes the data, and SOLVE_CASE solves the individual
cases. AUTOSOLVE uses the class to solve the code jam.
*/
SOLVER
{
	/*
	This function is called once, before any cases. It should initialize any
	data in the class that is global to every case, then return the number of
	test cases. If no such data needs to be shared, consider using BASIC_SOLVE
	(see basic_template.cpp) instead. It is passed a Tokens object called
	tokens. It does not need to (and should not) call tokens.done() after it is
	finished, even when running multithreaded.
	*/
	PRE_SOLVE
	{
		return NEXT(unsigned);
	}

	/*
	Your solution goes here. SOLVE_CASE creates a membrer function which solves
	a single code jam case. The return type is deduced automatically. The
	function is passed a Tokens object called Tokens; see also the Tokens macro
	interface for quick use of this object. This function is declared
	const; it cannot modify any class members while it is running.

	Tokens macro interface:

	NEXT(TYPE), GROUP(...), FILL(CONTAINER), MANY(N) [](){}
	TOKEN(TYPE, NAME), MUT_TOKEN(TYPE, NAME), TOK_INT(NAME), TOK_STR(NAME)
	TOK_CONTAINER(T, NAME, SIZE), TOK_VEC(T, NAME, SIZE), TOK_INTVEC(NAME, SIZE)
	*/
	SOLVE_CASE
	{

	}
};

AUTOSOLVE
