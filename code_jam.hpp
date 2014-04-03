/*
 * Copyright 2014 Nathan West
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Th program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

 //C++ implementation of the code jam library
 
 #pragma once

 #include <iostream>
 #include <iterator>

class Tokens
{
private:
	std::istream* stream;

public:
	Tokens(std::istream& str):
		stream(&str)
	{}

	std::istream& istr()
	{
		return *stream;
	}

	template<class T>
	T next_token()
	{
		T result;
		istr() >> result;
		return result;
	}

	template<class It>
	void next_many_tokens(It begin, It end)
	{
		for(; begin != end; ++begin)
			istr() >> *begin;
	}
	
	template<class T>
	void next_n_tokens(unsigned n, It begin)
	{
		for(unsigned i = 0; i < n; ++i, ++begin)
			istr() >> *begin
	}
};

template<class Solver>
void generic_solve_code_jam(unsigned num_cases, Solver&& solver,
	std::ostream& ostr=std::cout)
{
	for(unsigned c = 1; c <= num_cases; ++c)
		ostr << "Case #" << c << ": " << solver() << "\n";
}

template<class Solver>
void solve_code_jam(Solver&& solver, std::istream& istr=std::cin,
	std::ostream& ostr=std::cout)
{
	Tokens tokens(istr);
	generic_solve_code_jam(
		tokens.next_token<unsigned>(),
		[&tokens, &solver] () {return solver(tokens)},
		ostr);
}
