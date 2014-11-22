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
This is a multithreaded C++ implementation of the code jam helper. Provides the
solve_code_jam_multithreaded function, which takes an istream, ostream, and
solver function. One thread is created per test case, and the solver function
is called once per thread with a Tokens object and a locked std::mutex. The
solver function should unlock the mutex once it is done reading tokens, to allow
other threads to proceed. Correct order of output is handled automatically.
*/

#include <thread>
#include <vector>
#include <mutex>
#include <condition_variable>
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



/*
 * Generic solve. Use this when the number of test cases is separatly
 * determined. The solver object is a callable that takes the tokens objcet as
 * an argument. In the course of solving the puzzle, it MUST call tokens.done()
 * when it has finished reading all the tokens it needs, so that other threads
 * can begin.
 */
template<class Solver>
void solve_code_jam_multithreaded(Solver&& solver, ThreadedTokens& tokens,
	unsigned num_cases, std::ostream& ostr, bool insert_newline=false)
{
	// These variables control print ordering
	std::mutex print_mutex;
	std::condition_variable print_cond;
	unsigned next_print = 0;

	//Our threads!
	std::vector<std::thread> threads;
	threads.reserve(num_cases);

	for(unsigned case_id = 0; case_id < num_cases; ++case_id)
	{
		tokens.start_case();
		threads.emplace_back([case_id, &solver, &tokens, &print_mutex,
			&print_cond, &next_print, &ostr, insert_newline]
		{
			//solve the case. solver must call tokens.done()
			auto solution = solver(tokens);

			//Lock for printing
			std::unique_lock<std::mutex> print_lock(print_mutex);

			//Wait until our turn to print
			print_cond.wait(print_lock, [case_id, &next_print]
			{
				return case_id == next_print;
			});

			//Print result
			print_case(solution, case_id, ostr, insert_newline);

			//Increment print counter and signal next thread
			++next_print;
			print_cond.notify_all();
		});
	}

	for(auto& thread : threads)
		thread.join();
}

/*
 * Standard solve. Use this when the first token is the number of test cases.
 * The solver is a function that takes a Tokens object and a mutex as the
 * arguments and returns the solution.
 */
template<class Solver>
void solve_code_jam_multithreaded(Solver&& solver, std::istream& istr,
	std::ostream& ostr, bool insert_newline=false)
{
	ThreadedTokens tokens(istr);

	solve_code_jam_multithreaded(
		[&solver](Tokens& tokens) { return solver(tokens); },
		tokens, tokens.next_token<unsigned>(), ostr, insert_newline);
}

/*
 * Create a main function that calls solve_code_jam_multithreaded with cin,
 * cout, and a function pointer to the provided function
 */
#define AUTOMAIN_MULTITHREAD(FUNCTION) \
int main(int argc, char const *argv[]) \
{ solve_code_jam_multithreaded((FUNCTION), std::cin, std::cout, false); }
