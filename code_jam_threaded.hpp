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

struct TaskState
{
private:
	std::mutex input_mutex;
	unsigned next_task_id = 1;

	std::mutex print_mutex;
	std::condition_variable print_cond;
	unsigned next_print = 1;

	std::ostream* ostr;

public:
	TaskState(std::ostream& ostr):
		ostr(&ostr)
	{}

	template<class Solver>
	void run_thread(Solver&& solver)
	{
		//Lock input and find task ID
		token_mutex.lock();
		unsigned task_id = next_task_id++;

		//Solve case. solver MUST unlock the token_mutex when it is done reading
		auto result = solver(token_mutex);

		//Lock for printing
		std::unique_lock<std::mutex> lock(print_mutex);

		//Wait until our turn to print
		print_cond.wait(lock, [this, task_id]()
		{
			return task_id == next_print;
		});

		//Print result
		(*ostr) << "Case #" << task_id << ": " << result << '\n';

		//Increment print counter and signal next thread
		++next_print;
		print_cond.notify_all();
	}
};

//Generic solve. Use this when the number of test cases is separatly determined
template<class Solver>
void solve_code_jam_multithreaded(unsigned num_tasks, std::ostream& ostr,
	Solver&& solver)
{
	TaskState task_state(ostr);
	std::vector<std::thread> threads;
	threads.reserve(num_tasks);

	for(unsigned i = 0; i < num_tasks; ++i)
		threads.emplace_back([&task_state, &solver]()
		{
			task_state.run_thread(solver);
		});

	for(auto& thread : threads)
		thread.join();
}

//Standard solve. Use this when the first token is the number of test cases
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

/*
 * Use this instead of a normal function declaration to create a program that
 * automatically solves, using the function. Example:
 *
 *	AUTOSOLVE_MULTITHREAD(int, tokens, mutex)
 *	{
 *		int x = tokens.next_token<int>();
 *		mutex.unlock();
 *		//Solve!
 *		return solution;
 *	}
 */
#define AUTOSOLVE_MULTITHREAD(RETURN_TYPE, TOKENS, MUTEX) \
RETURN_TYPE autosolve(Tokens& TOKENS, std::mutex& MUTEX); \
MAIN_MULTITHREAD(autosolve) \
RETURN_TYPE autosolve(Tokens& TOKENS, std::mutex& MUTEX)


