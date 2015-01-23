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

/*
This is a multithreaded C++ implementation of the code jam helper. Provides
the ThreadedCodeJamSolver class, which functions identically to the
CodeJamSolver class, except that it solves each case in a separate thread. It
automatically handles the ordering of token input to each case and solution
output. The solve_case function must call tokens.done() to signal the next
thread to begin reading tokens.
*/

#pragma once

#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <iostream>

#include "code_jam.hpp"

class ThreadedTokens : public Tokens
{
private:
	std::mutex input_mutex;

public:
	ThreadedTokens(std::istream& istr):
		Tokens(istr)
	{}

	void start_case()
	{
		input_mutex.lock();
	}

	void done() override
	{
		input_mutex.unlock();
	}
};

class ThreadedPrinter
{
	std::mutex print_mutex;
	std::condition_variable print_cond;
	unsigned next_print = 0;

	std::ostream* ostr;

public:
	ThreadedPrinter(std::ostream& ostr):
		ostr(&ostr)
	{}

	template<class Solution>
	void ordered_print(const Solution& solution, const unsigned case_id)
	{
		{
			// Lock for printing
			std::unique_lock<std::mutex> print_lock(print_mutex);

			// Wait until our turn to print
			print_cond.wait(print_lock, [this, case_id]
			{
				return case_id == next_print;
			});

			// Print result
			print_case(solution, case_id, *ostr);

			// Increment print counter and signal next thread
			++next_print;
		}
		print_cond.notify_all();
	}
};

template<class Solver>
inline void threaded_solve_code_jam(std::istream& istr, std::ostream& ostr)
{
	ThreadedTokens tokens(istr);
	ThreadedPrinter printer(ostr);

	Solver solver;
	const Solver& c_solver = solver;

	std::vector<std::thread> threads;
	threads.reserve(num_cases);

	const unsigned num_cases = solver.pre_solve(tokens);

	for(unsigned case_id = 0; case_id < num_cases; ++case_id)
	{
		tokens.start_case();
		threads.emplace_back([&c_solver, &tokens, &printer, case_id]
		{
			printer.ordered_print(c_solver.solve_case(tokens), case_id);
		});
	}

	for(auto& thread : threads)
		thread.join();
}

#define THREADED_AUTOSOLVE int main() { threaded_solve_code_jam<Solver>(std::cin, std::cout); }
