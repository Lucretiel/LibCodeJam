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
		return token;
	}

	/*
<<<<<<< HEAD
=======
	Quick macro to create an object of type TYPE and initialize it with
	next_token. Assumes the Tokens instance is called tokens.
	 */
	#define TOKEN(TYPE, NAME) TYPE NAME{tokens.next_token<TYPE>()}

	/*
>>>>>>> FETCH_HEAD
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

	// Fancy macro interface
	#define NEXT(TYPE) tokens.next_token<TYPE>()
	#define TOKEN(TYPE, NAME) TYPE NAME{ NEXT(TYPE) }
	#define LOAD(...) tokens.load_tokens(__VA_ARGS__)
	#define FILL(CONTAINER) tokens.next_many_tokens(CONTAINER)
	#define STREAM tokens.stream()
	#define DONE() tokens.done()
};

/*
Base class for solvers. The macros defined at the end of the library assist=
with creating the actual solver class. This class exists primarily to provide
a default pre_solve implementation; this implementaton is simply shadowed if
the client provides their own.
*/
class SolverBase
{
public:
	unsigned pre_solve(Tokens& tokens)
	{
		return tokens.next_token<unsigned>();
	}
};

// Print a single solution. Returns !ostr.good after printing.
template<class Solution>
inline bool print_case(
	const Solution& solution,
	const unsigned case_id,
	std::ostream& ostr)
{
	ostr << "Case #" << case_id + 1 << (INSERT_NEWLINE ? ":\n" : ": ") <<
		solution << std::endl;

	return !ostr.good();
}

/*
Solve num_cases cases, using a solver. The solver should already have been
pre_solved
*/

template<class Solver>
inline void solve_code_jam(
	const Solver& solver,
	const unsigned num_cases,
	Tokens& tokens,
	std::ostream& ostr)
{
	for(unsigned c = 0; c < num_cases; ++c)
		if(print_case(solver.solve_case(tokens), c, ostr))
			return;
}

template<class Solver>
inline void solve_code_jam(std::istream& istr, std::ostream& ostr)
{
	Tokens tokens(istr);
	Solver solver;
	solve_code_jam(solver, solver.pre_solve(tokens), tokens, ostr);
}

#define SOLVER class Solver : public SolverBase

#define SOLVE_CASE public: inline auto solve_case(Tokens& tokens) const

#define PRE_SOLVE public: inline auto pre_solve(Tokens& tokens)

#define BASIC_SOLVE SOLVER { SOLVE_CASE; }; \
	inline auto Solver::solve_case(Tokens& tokens) const

#define AUTOSOLVE int main() { solve_code_jam<Solver>(std::cin, std::cout); }
