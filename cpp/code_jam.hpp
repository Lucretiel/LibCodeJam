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
#include <cstdint>

// Not used by LibCodeJam, but are commonly used in many solutions
#include <vector>
#include <string>
#include <algorithm>
#include <utility>

using namespace std;

// Convenient typedef
typedef intmax_t Int;
typedef uintmax_t UInt;

class Tokens
{
private:
	istream* istr;

	// Via http://stackoverflow.com/q/7943525
	template<class T>
	struct function_traits : public function_traits<decltype(&T::operator())>
	{};

	template<class Ret, class Arg>
	struct function_traits<Ret(Arg)>
	{
		typedef Arg arg_type;
	};

	template<class Ret, class Arg>
	struct function_traits<Ret(*)(Arg)>
	{
		typedef Arg arg_type;
	};

	template<class T, class Ret, class Arg>
	struct function_traits<Ret(T::*)(Arg)>
	{
		typedef Arg arg_type;
	};

	template<class T, class Ret, class Arg>
	struct function_traits<Ret(T::*)(Arg) const>
	{
		typedef Arg arg_type;
	};

public:
	explicit Tokens(istream& istr):
		istr(&istr)
	{}

	/*
	Get and return a single token. Useful for storing const data. The return
	type removes const-qualifiers so that the return value can be used to
	move construct a type.
	*/
	template<class T>
	typename remove_cv<T>::type next_token()
	{
		typedef typename remove_cv<T>::type MutableT;
		static_assert(!is_reference<MutableT>::value,
			"next_token cannot get a reference type");

		MutableT token;
		stream() >> token;
		return token;
	}

	/*
	Fill 1 or more variables of arbitrary type with tokens, in order.
	*/
	template<class T, class... Rest>
	void load_tokens(T& t, Rest&... rest)
	{
		stream() >> t;
		load_tokens(rest...);
	}

	void load_tokens()
	{}

	// Fill a container with tokens
	template<class Container>
	void fill(Container& container)
	{
		for(auto& i : container)
			stream() >> i;
	}

	// Call some lambda on the next n tokens of type T. Returns n.
	template<class Func>
	UInt next_many_tokens(const UInt n, Func&& func)
	{
		typedef typename decay<typename function_traits<Func>::arg_type>::type T;
		for(UInt i = 0; i < n; ++i)
			func(next_token<T>());
		return n;
	}

	// Read a token n, then call the lambda on the next n tokens of type T.
	// Returns the number of read tokens
	template<class Func>
	UInt next_counted_tokens(Func&& func)
	{
		return next_many_tokens(next_token<UInt>(), func);
	}

	/*
	Access to the underlying stream, for other input operations the user may
	want.
	*/
	istream& stream() { return *istr; }

	/*
	For the threaded version, this function should be called to signal that the
	next thread may begin reading tokens
	*/

	virtual void done() {};

	// Fancy macro interface
	#define NEXT(TYPE) tokens.next_token<TYPE>()
	#define LOAD(...) tokens.load_tokens(__VA_ARGS__)
	#define FILL(CONTAINER) tokens.fill(CONTAINER)
	#define DONE() tokens.done()

	#define MUT_TOKEN(TYPE, NAME) TYPE NAME{ NEXT(TYPE) }
	#define TOKEN(TYPE, NAME) MUT_TOKEN(TYPE const, NAME)

	#define TOK_INT(NAME) TOKEN(Int, NAME)
	#define TOK_STR(NAME) TOKEN(string, NAME)

	#define TOK_CONTAINER(TYPE, NAME, SIZE) TYPE NAME{ SIZE }; FILL(NAME)
	#define TOK_VEC(TYPE, NAME, SIZE) TOK_CONTAINER(vector<TYPE>, NAME, SIZE)
	#define TOK_INTVEC(NAME, SIZE) TOK_VEC(Int, NAME, SIZE)
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

template<class Solution>
void print_case(
	const Solution& solution,
	const unsigned case_id,
	ostream& ostr)
{
#ifndef INSERT_NEWLINE
	ostr << "Case #" << case_id + 1 << ": " << solution << endl;
#else
	ostr << "Case #" << case_id + 1 << ":\n" << solution << endl;
#endif
}

template<class Solver>
void solve_code_jam(istream& istr, ostream& ostr)
{
	Tokens tokens(istr);
	Solver solver;
	const unsigned num_cases = solver.pre_solve(tokens);

	for(unsigned case_id = 0; case_id < num_cases; ++case_id)
		print_case(solver.solve_case(tokens), case_id, ostr);
}

#define SOLVER class Solver : public SolverBase

#define PRE_SOLVE public: unsigned pre_solve(Tokens& tokens)

#define SOLVE_CASE public: auto solve_case(Tokens& tokens) const

#define BASIC_SOLVE SOLVER { auto solve_case(Tokens&) const; }; \
	auto Solver::solve_case(Tokens& tokens) const

#define AUTOSOLVE int main() { solve_code_jam<Solver>(cin, cout); }
