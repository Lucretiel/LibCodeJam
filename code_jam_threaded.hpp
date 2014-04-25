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

/*
 * Generic solve. Use this when the number of test cases is separatly
 * determined. The solver object is a callable that takes a locked mutex as the
 * argument and returns the solution. It should unlock the mutex after it
 * reads all the tokens.
 */
template<class Solver>
void solve_code_jam_multithreaded(unsigned num_cases, std::ostream& ostr,
	Solver&& solver)
{
	std::mutex input_mutex;

	std::mutex print_mutex;
	std::condition_variable print_cond;
	unsigned next_print = 0;

	std::vector<std::thread> threads;
	threads.reserve(num_cases);

	for(unsigned case_id = 0; case_id < num_cases; ++case_id)
	{
		input_mutex.lock();
		threads.emplace_back([case_id, &solver, &input_mutex, &print_mutex,
			&print_cond, &next_print, &ostr]
		{
			//Solve case. solver MUST unlock the input_mutex when it is done reading
			auto result = solver(input_mutex);

			//Lock for printing
			std::unique_lock<std::mutex> print_lock(print_mutex);

			//Wait until our turn to print
			print_cond.wait(print_lock, [case_id, &next_print]
			{
				return case_id == next_print;
			});

			//Print result
			ostr << "Case #" << case_id+1 << ": " << result << '\n';

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
void solve_code_jam_multithreaded(std::istream& istr, std::ostream& ostr,
	Solver&& solver)
{
	Tokens tokens(istr);

	solve_code_jam_multithreaded(
		tokens.next_token<unsigned>(), ostr,
		[&tokens, &solver](std::mutex& mutex)
		{
			return solver(tokens, mutex);
		});
}

/*
 * Create a main function that calls solve_code_jam_multithreaded with cin,
 * cout, and a function pointer to the provided function
 */
#define MAIN_MULTITHREAD(FUNCTION) \
int main(int argc, char const *argv[]) \
{ solve_code_jam_multithreaded(std::cin, std::cout, (&FUNCTION)); }
