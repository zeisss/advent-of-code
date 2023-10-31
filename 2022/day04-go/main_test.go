package main

import (
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/require"
)

func TestParseData_Approval(t *testing.T) {
	input := `5-7,7-9
	2-8,3-7`

	pairs, err := ParseData([]byte(input))
	require.NoError(t, err)
	approvals.VerifyArray(t, pairs)
}

func TestExampe_Part1(t *testing.T) {
	input := `5-7,7-9
	2-8,3-7
	6-6,4-6
	2-6,4-8`

	pairs, err := ParseData([]byte(input))
	require.NoError(t, err)

	task1, _ := check(pairs)

	require.Equal(t, 2, task1, "Task 1")

}

func TestExampe_Part2_Examples(t *testing.T) {
	check := func(input string, overlap bool) {
		t.Run(input, func(t *testing.T) {
			var pair Pair
			err := ParseLine(input, &pair)
			require.NoError(t, err)

			out := Overlap(pair)

			require.Equal(t, overlap, out, "Expected overlaps")
		})
	}

	check("2-4,6-8", false)
	check("2-3,4-5", false)
	check("5-7,7-9", true)
	check("2-8,3-7", true)
	check("6-6,4-6", true)
	check("2-6,4-8", true)
}

func TestExampe_Part2(t *testing.T) {
	input := `5-7,7-9
	2-8,3-7
	6-6,4-6
	2-6,4-8`

	pairs, err := ParseData([]byte(input))
	require.NoError(t, err)

	_, task2 := check(pairs)

	require.Equal(t, 4, task2, "Task 2")
}

func TestInput_txt(t *testing.T) {
	pairs, err := ParseFile("testdata/input.txt")
	require.NoError(t, err)

	t1, t2 := check(pairs)
	t.Logf("check: %d, %d", t1, t2)
	require.EqualValues(t, 503, t1)
}
