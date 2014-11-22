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

#include <iostream>
#include <fstream>
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
	 * Get and return a single token. Useful for storing const data.
	 */
	template<class T>
	typename std::remove_cv<T>::type next_token()
	{
		typename std::remove_cv<T>::type token;
		stream() >> token;
		return token;
	}

	/*
	 * Quick macro to create an object of type TYPE and initialize it with
	 * next_token
	 */
	#define TOKEN(TYPE, NAME, TOKENS) TYPE NAME{(TOKENS).next_token<TYPE>()}

	/*
	 * Fill 1 or more variables of arbitrary type with tokens, in order.
	 */
	template<class T, class... Rest>
	void load_tokens(T& t, Rest&... rest)
	{
		stream() >> t;
		load_tokens(rest...);
	}

	void load_tokens()
	{}

	/*
	 * Fill a container with tokens
	 */
	template<class Container>
	void next_many_tokens(Container& container)
	{
		for(auto& i : container)
			stream() >> i;
	}

	/*
	 * Insert n tokens into an input iterator
	 */
	template<class Iterator>
	void next_many_tokens(Iterator it, unsigned n)
	{
		for(unsigned i = 0; i < n; ++i, ++it)
			// Use assignment instead of "stream() >>"" to allow for
			// back_inserter and other funky iterators
			*it = next_token<
				typename std::iterator_traits<Iterator>::value_type>();
	}

	/*
	 * Read a token n, then insert n tokens into an input iterator
	 */
	template<class Iterator>
	void next_counted_tokens(Iterator it)
	{
		next_many_tokens(it, next_token<unsigned>());
	}

	/*
	 * Access to the underlying stream, for other input operations the use may
	 * want.
	 */
	std::istream& stream() { return *istr; }

	/*
	 * Signal that this particular case is done reading tokens. No-op here, but
	 * multi-threaded versions use it to signal the next thread to begin
	 * reading tokens.
	 */
	virtual void done() {}
};

/*
 * Format and print a single code jam test case solution. case_index is
 * 0-indexed. If insert_newline is true, insert a newline before the actual
 * solution. Returns true if the output failed somehow (for instance, a broken
 * pipe)
 */
template<class Solution>
inline bool print_case(Solution&& solution, unsigned case_index,
	std::ostream& ostr, bool insert_newline=false)
{
	if(ostr.good())
		ostr << "Case #" << case_index + 1 <<
		(insert_newline ? ":\n" : ": ") <<
		solution << std::endl;

	return !ostr.good();
}

/*
 * Solve a generic code jam. For each case in num_cases, solver is called, and
 * the return value is formatted and printed as a code jam solution to ostr.
 * If insert_newline is true, a newline is printed before each solution.
 */
template<class Solver>
void generic_solve_code_jam(Solver&& solver, unsigned num_cases,
	std::ostream& ostr, bool insert_newline=false)
{
	for(unsigned case_index = 0; case_index < num_cases; ++case_index)
		if(print_case(solver(), case_index, ostr, insert_newline))
			return;
}

/*
 * Solve a standard code jam. The first token is read off the input stream,
 * and used as the number of test cases. For each case, solver is called with
 * the created Tokens object, and the result is formatted and printed to ostr.
 * If insert_newline is true, a newline is printed before each solution.
 */
template<class Solver>
void solve_code_jam(Solver&& solver, std::istream& istr, std::ostream& ostr,
	bool insert_newline=false)
{
	Tokens tokens(istr);

	generic_solve_code_jam([&solver, &tokens] { return solver(tokens); },
		tokens.next_token<unsigned>(), ostr, insert_newline);
}

/*
 * Helper function for the AUTOMAIN wrapper. If ifile and/or ofile are given,
 * they are opened as files and used instead of cin and cout.
 */
template<class Solver>
void automain(Solver&& solver, const char* ifile, const char* ofile,
	bool insert_newline=false)
{
	std::ifstream istr;
	if(ifile)
		istr.open(ifile);

	std::ofstream ostr;
	if(ofile)
		istr.open(ofile);

	solve_code_jam(
		solver,
		istr.is_open() ? istr : std::cin,
		ostr.is_open() ? ostr : std::cout,
		insert_newline);
}

#define _AUTOMAIN(SOLVER, INSERT_NEWLINE) \
int main(int argc, char const *argv[]) \
{ automain((SOLVER), argc > 1 ? argv[1] : nullptr, argc > 2 ? argv[2] : nullptr, (INSERT_NEWLINE)); }

/*
 * Add this macro at the bottom of your source file, with the name of your
 * solver function or object. The code jam is solved as a standard code jam,
 * using solve_code_jam, with this object. Input is taken from the first file
 * argument, or stdin, and Output is written to the second file argument, or
 * stdout. Use AUTOMAIN_NEWLINE to print a newline before each solution.
 */
#define AUTOMAIN(SOLVER) _AUTOMAIN((SOLVER), false)

#define AUTOMAIN_NEWLINE(SOLVER) _AUTOMAIN((SOLVER), true)
