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
#include <utility>

class Tokens
{
private:
	std::istream* istr;

public:
	explicit Tokens(std::istream& istr=std::cin):
		istr(&istr)
	{}

	std::istream& stream() { return *istr; }

	//Get the next token
	template<class T>
	T next_token()
	{
		T token;
		stream() >> token;
		return token;
	}

	//Fill the range with the next tokens
	template<class Iterator>
	void next_tokens(Iterator begin, Iterator end)
	{
		for(; begin != end; ++begin)
			stream() >> *begin;
	}

	//Fill the range with the next n tokens
	template<class Iterator>
	void next_n_tokens(Iterator begin, const unsigned n)
	{
		for(unsigned i = 0; i < n; ++i, ++begin)
			stream() >> *begin;
	}

	//Read a token n, then fill the range with the next n tokens
	template<class Iterator>
	void next_counted_tokens(Iterator begin);
	{
		next_n_tokens(begin, next_token<unsigned>())
	}

	//Apply a function to the next n tokens of type T
	template<class T, class Func>
	void apply_n_tokens(const unsigned n, Func&& func)
	{
		for(unsigned i = 0; i < n; ++i)
			func(next_token<T>());
	}

	//Read a token n, then apply a function to the next n tokens of type T
	template<class T, class Func>
	void apply_counted_tokens(Func&& func)
	{
		apply_n_tokens<T>(next_token<unsigned>(),
			std::forward<Func>(func));
	}
};

template<class Solver>
void solve_code_jam(unsigned num_cases, std::ostream& ostr, Solver&& solver)
{
	for(unsigned c = 1; c <= num_cases; ++c)
		ostr << "Case #" << c << ": " << solver() << '\n';
}

template<class Solver>
void solve_code_jam(std::istream& istr, std::ostream& ostr, Solver&& solver)
{
	Tokens token(istr)

	solve_code_jam(tokens.next_token<unsigned>(), ostr,
		[&solver, &tokens]() { return solver(tokens); });
}

#define AUTOSOLVE(FUNCTION) \
int main(int argc, char const *argv[]) \
{ solve_code_jam(std::cin, std::cout, (&FUNCTION)); return 0; }
