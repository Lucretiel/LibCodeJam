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
#include <mutex>
#include <condition_variable>
#include <vector>
#include <utility>

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

template<class Solution>
class ThreadedCodeJamSolver : public CodeJamSolver<Solution>
{
private:
	std::mutex print_mutex;
	std::condition_variable print_cond;
	unsigned next_print;

	void ordered_print(const Solution& solution, const unsigned index,
		std::ostream& ostr)
	{
		//Lock for printing
		std::unique_lock<std::mutex> print_lock(print_mutex);

		//Wait until our turn to print
		print_cond.wait(print_lock, [this, index]
		{
			return index == next_print;
		});

		//Print result
		this->print_case(solution, index, ostr);

		//Increment print counter and signal next thread
		++next_print;
		print_cond.notify_all();
	}

	void solve_code_jam(std::istream& istr, std::ostream& ostr) override
	{
		// Load up synchronized tokens
		ThreadedTokens tokens(istr);

		//Presolve
		const unsigned num_cases = this->pre_solve(tokens);

		//Reserve memory for threads
		std::vector<std::thread> threads;
		threads.reserve(num_cases);

		// Reset printing
		next_print = 0;

		for(unsigned case_index = 0; case_index < num_cases; ++case_index)
		{
			/*
			 * Lock for a new test case. The test case must call done, allowing
			 * future threads to be spawned. case_index is captured by value,
			 * so that each thread has its own fixed index.
			 */
			tokens.start_case();
			threads.emplace_back([this, case_index, &tokens, &ostr]
			{
				/*
				 * Solve and print the case in this thread. The solver must
				 * call tokens.done() to allow future threads to be spawned,
				 * and the ordered print ensures that output happens in the
				 * right order.
				 */
				ordered_print(this->solve_case(tokens), case_index, ostr);
			});
		}

		for(auto& thread : threads)
			thread.join();
	}
};

#define SOLVER_MULTITHREADED(SOLUTION_TYPE, BODY) _SOLVER(ThreadedCodeJamSolver, SOLUTION_TYPE, BODY)
