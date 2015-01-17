//#define INSERT_NEWLINE true

#include "LibCodeJam/cpp/code_jam.hpp"

/*
SOLVER creates a class which stores any data that needs to be shared between
cases. PRE_SOLVE initializes the data, and SOLVE_CASE solves the individual
cases.
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
	Your solution goes here. BASIC_SOLVE creates a function which solves a single
	code jam case, which is invoked by AUTOSOLVE. The return type is deduced
	automagially. The function is passed a Tokens object called tokens; see also
	the macro interface for quick use of this object. This function is declared
	const; it cannot modify any class members while it is running.
	*/
	SOLVE_CASE
	{

	}
};

AUTOSOLVE
