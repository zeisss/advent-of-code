package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var example = [][]int{
	{3, 4, 2, 1, 3, 3},
	{4, 3, 5, 3, 9, 3},
}

func TestParse(t *testing.T) {
	s := `3   4
4   3
2   5
1   3
3   9
3   3`
	n := Parse(s)

	assert.Equal(t, example, n)
}

func TestEval(t *testing.T) {
	n := EvalPart1(example)
	assert.Equal(t, 11, n)
}
