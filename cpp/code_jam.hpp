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
#include <type_traits>

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
		typename std::remove_cv<T>::type token;
		stream() >> token;
		return token;
	}

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

	// Fancy macro interface
	#define NEXT(TYPE) tokens.next_token<TYPE>()
	#define LOAD(...) tokens.load_tokens(__VA_ARGS__)
	#define FILL(CONTAINER) tokens.next_many_tokens(CONTAINER)
	#define FILL_IT(ITERATOR, N) tokens.next_many_tokens(ITERATOR, N)
	#define COUNTED(ITERATOR) tokens.next_many_tokens(ITERATOR)
	#define DONE() tokens.done()

	#define TOKEN(TYPE, NAME) TYPE NAME{ NEXT(TYPE) }
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
		return NEXT(unsigned);
	}
};

#ifndef INSERT_NEWLINE
#define SEP ": "
#else
#define SEP ":\n"
#endif

template<class Solution>
inline void print_case(
	const Solution& solution,
	const unsigned case_id,
	std::ostream& ostr)
{
	ostr << "Case #" << case_id + 1 << SEP << solution << std::endl;
}

#undef SEP

template<class Solver>
inline void solve_code_jam(std::istream& istr, std::ostream& ostr)
{
	Tokens tokens(istr);
	Solver solver;
	const Solver& c_solver = solver;
	const unsigned num_cases = solver.pre_solve(tokens);

	for(unsigned case_id = 0; case_id < num_cases; ++case_id)
		print_case(c_solver.solve_case(tokens), case_id, ostr);
}

#define SOLVER class Solver : public SolverBase

#define SOLVE_CASE public: inline auto solve_case(Tokens& tokens) const

#define PRE_SOLVE public: inline unsigned pre_solve(Tokens& tokens)

#define BASIC_SOLVE SOLVER { SOLVE_CASE; }; \
	inline auto Solver::solve_case(Tokens& tokens) const

#define AUTOSOLVE int main() { solve_code_jam<Solver>(std::cin, std::cout); }
