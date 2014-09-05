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

class Tokens
{
private:
	std::istream* istr;
	std::istream& stream() { return *istr; }

public:
	explicit Tokens(std::istream& istr=std::cin):
		istr(&istr)
	{}

	//istream-style read off of stream
	template<class T>
	Tokens& operator >>(T& token)
	{
		stream() >> token;
		return *this;
	}

	//Get and return a single token
	template<class T>
	T next_token()
	{
		T token;
		stream() >> token;
		return token;
	}

	//Get a fixed set of tokens
	template<class T, class... Args>
	void load_tokens(T& token, Args&... rest)
	{
		stream() >> token;
		load_tokens(rest...);
	}

	void load_tokens()
	{}

	//Fill the range with the next tokens
	template<class Iterator>
	void next_tokens(Iterator begin, Iterator end)
	{
		for(; begin != end; ++begin)
			stream() >> *begin;
	}
	
	//Fill a container with the next tokens
	template<class Container>
	void next_tokens(Container& container)
	{
		for(auto& e : container)
			stream() >> e;
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
	void next_n_tokens(Iterator begin)
	{
		next_n_tokens(begin, next_token<unsigned>());
	}

	//Push back the next n tokens to a container
	template<class Container, class T=typename Container::value_type>
	void push_back_tokens(Container& container, const unsigned n)
	{
		for(unsigned i = 0; i < n; ++i)
			container.push_back(next_token<T>());
	}

	//Read a token n, then push back the next n tokens
	template<class Container, class T=typename Container::value_type>
	void push_back_tokens(Container& container)
	{
		push_back_tokens(container, next_token<unsigned>());
	}
};

template<class Solver>
void solve_code_jam(unsigned num_cases, std::ostream& ostr, Solver&& solver)
{
	for(unsigned c = 1; c <= num_cases; ++c)
		ostr << "Case #" << c << ": " << solver() << endl;
}

template<class Solver>
void solve_code_jam(std::istream& istr, std::ostream& ostr, Solver&& solver)
{
	Tokens tokens(istr);

	solve_code_jam(tokens.next_token<unsigned>(), ostr,
		[&solver, &tokens]() { return solver(tokens); });
}

#define MAIN(FUNCTION) \
int main(int argc, char const *argv[]) \
{ solve_code_jam(std::cin, std::cout, (&FUNCTION)); }
