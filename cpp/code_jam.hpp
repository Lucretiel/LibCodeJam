/*
Copyright 2014 Nathan West

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#pragma once

#include <iostream>
#include <fstream>
#include <type_traits>

/*
In your .cpp file, override this with true before #including this file if
you want to insert newlines between the Case# and the actual solutions
*/
#ifndef INSERT_NEWLINE
#define INSERT_NEWLINE false
#endif

class Tokens
{
private:
	std::istream* istr;

public:
	explicit Tokens(std::istream& istr):
		istr(&istr)
	{}

	/*
	Get and return a single token. Useful for storing const data. The return
	type removes const-qualifiers so that the return value can be used to
	move construct a type
	*/
	template<class T>
	typename std::remove_cv<T>::type next_token()
	{
		typename std::remove_cv<T>::type token{};
		stream() >> token;
		// Return a non-const to allow move construction from the return value.
		return token;
	}

	/*
	Quick macro to create an object of type TYPE and initialize it with
	next_token
	 */
	#define TOKEN(TYPE, NAME) TYPE NAME{tokens.next_token<TYPE>()}

	/*
	Fill 1 or more variables of arbitrary type with tokens, in order.
	*/
	template<class T, class... Rest>
	inline void load_tokens(T& t, Rest&... rest)
	{
		stream() >> t;
		load_tokens(rest...);
	}

	inline void load_tokens()
	{}

	// Fill a container with tokens
	template<class Container>
	void next_many_tokens(Container& container)
	{
		for(auto& i : container)
			stream() >> i;
	}

	// Insert n tokens into an input iterator
	template<class Iterator>
	void next_many_tokens(Iterator it, unsigned n)
	{
		for(unsigned i = 0; i < n; ++i, ++it)
			/*
			Use assignment instead of "stream() >>"" to allow for back_inserter
			and other funky iterators
			*/
			*it = next_token<
				typename std::iterator_traits<Iterator>::value_type>();
	}

	// Read a token n, then insert n tokens into an input iterator
	template<class Iterator>
	void next_counted_tokens(Iterator it)
	{
		next_many_tokens(it, next_token<unsigned>());
	}

	/*
	Access to the underlying stream, for other input operations the user may
	want.
	*/
	std::istream& stream() { return *istr; }

	/*
	For the threaded version, this function should be called to signal that the
	next thread may begin reading tokens
	*/

	virtual void done() {};
};

/*
This class solves a whole code jam. Users should implement solve_case, which is
called for each test case. They may also implement pre_solve, which is called
once, before the individual cases. If pre_solve is implemented, it should
return the number of test cases.
*/
template<class Solution>
class CodeJamSolver
{
protected:
	//Format and print a single solution to a code jam.
	bool print_case(const Solution& solution, unsigned index,
		std::ostream& ostr)
	{
		if(ostr.good())
			ostr << "Case #" << index + 1 <<
			(INSERT_NEWLINE ? ":\n" : ": ") <<
			solution << std::endl;

		return !ostr.good();
	}

	virtual unsigned pre_solve(Tokens& tokens)
	{
		return tokens.next_token<unsigned>();
	}

	virtual Solution solve_case(Tokens& tokens) const =0;

public:
	/*
	Solve a code jam from an input stream, writing the results to an output
	stream.
	*/
	virtual void solve_code_jam(std::istream& istr, std::ostream& ostr)
	{
		Tokens tokens(istr);

		const unsigned num_cases = pre_solve(tokens);

		for(unsigned case_index = 0; case_index < num_cases; ++case_index)
			if(print_case(solve_case(tokens), case_index, ostr))
				return;
	}


	/*
	Solve a code jam using filenames. If either or both of the arguments are
	null, they default to cin and cout, respectively.
	*/
	void solve_files(const char* ifile, const char* ofile)
	{
		std::ifstream istr;
		if(ifile)
			istr.open(ifile);

		std::ofstream ostr;
		if(ofile)
			ostr.open(ofile);

		solve_code_jam(
			istr.is_open() ? istr : std::cin,
			ostr.is_open() ? ostr : std::cout);
	}
};

#define _SOLVER(BASE, SOLUTION_TYPE, BODY) \
class Solver : public BASE<SOLUTION_TYPE> { SOLUTION_TYPE solve_case(Tokens& tokens) const override BODY }

// Create a class called Solver with solve_case being the given body
#define SOLVER(SOLUTION_TYPE, BODY) _SOLVER(CodeJamSolver, SOLUTION_TYPE, BODY)

/*
Add this macro at the bottom of your source file, with the name of your
solver class. This macro creates a main function which solves the code jam
using solve_files, with this class. Input is taken from first file command
line argument, or stdin, and output is written to the second file command
line argument, or stdout.
*/
#define MAIN(CLASS) \
int main(int argc, char const *argv[]) \
{ CLASS solver; solver.solve_files(argc > 1 ? argv[1] : nullptr, argc > 2 ? argv[2] : nullptr); }
