package main

import (
	"fmt"
	"os"
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/require"
)

func TestApplyAction(t *testing.T) {
	for i := 1; i <= 3; i++ {
		t.Run(fmt.Sprintf("amount=%d", i), func(t *testing.T) {
			out := StackMover9000.applyAction(
				[][]string{
					{"a", "b", "c"},
					{},
				},
				Action{Src: 1, Dst: 2, Amount: i},
			)

			require.Len(t, out[0], 3-i)
			require.Len(t, out[1], i)
		})
	}
}

func TestGetTopContainerNames(t *testing.T) {
	out := [][]string{
		{"a", "b", "c"},
		{"d"},
	}
	s := GetTopContainerNames(out)
	require.Equal(t, "cd", s)
}

func TestParseData(t *testing.T) {
	input := `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`
	plan, err := ParseData([]byte(input))
	require.NoError(t, err)

	approvals.VerifyJSONStruct(t, plan)
}

func TestParseData_Actions(t *testing.T) {
	for actions := 1; actions <= 4; actions++ {
		t.Run(fmt.Sprintf("processNextAction() %dx", actions), func(t *testing.T) {
			input := `    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`
			plan, err := ParseData([]byte(input))
			require.NoError(t, err)

			for i := 0; i < actions; i++ {
				StackMover9000.processNextAction(&plan)
			}
			approvals.VerifyJSONStruct(t, plan)

		})
	}
}

func TestMain(m *testing.M) {
	approvals.UseFolder("testdata")

	os.Exit(m.Run())
}
