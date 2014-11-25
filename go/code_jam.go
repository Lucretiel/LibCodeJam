import (
	"fmt"
	"io"
	"sync"
)

var insert_newline = false

type Tokens struct {

}

type STokens struct {
	istr io.Reader
}

type CodeJamSolver interface {
	// Load any global data and return the number of test cases
	pre_solve(Tokens) uint

	// Solve a single code jam case. This function should load all data needed,
	// then spawn a goroutine to actually solve the case. It sould return a
	// channel that will contain the solution, once found.
	solve_code_jam(Tokens) <-chan interface{}
}

func print_case(solution interface{}, case_index uint, ostr io.Writer) bool {
	sep := ' '
	if insert_newline {
		sep = '\n'
	}

	_, err := fmt.Fprintf(ostr, "Case #%d:%c%v", case_index + 1, sep, solution)

	return err
}

func solve_code_jam(CodeJamSolver solver, istr io.Reader, ostr io.Writer) {
	tokens := Tokens{istr}

	num_cases := solver.pre_solve(tokens)

	solutions := make(chan (<- chan interface{}), num_cases)

	// Spawn out solver goroutines in a separate goroutine, so that printing can
	// happen in this one
	go func() {
		for i := 0; i < num_cases; ++i {
			solutions <- solver.solve_code_jam(tokens)
		}
	}

	// Loop over the result channels, printing the solutions they contain
	for i, c := range solutions {
		print_case(<-c, i, ostr)
	}
}
