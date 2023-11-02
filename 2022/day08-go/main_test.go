package main

import (
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const TEST_INPUT = `30373
25512
65332
33549
35390`

func TestParse(t *testing.T) {
	data, err := Parse(TEST_INPUT)
	require.NoError(t, err)
	require.NotNil(t, data)

	approvals.VerifyJSONStruct(t, data)

	assert.EqualValues(t, 0, data.HeightAt(1, 0))
	assert.EqualValues(t, 6, data.HeightAt(0, 2))
}

func TestCountVisibleTrees(t *testing.T) {
	data, err := Parse(TEST_INPUT)
	require.NoError(t, err)
	require.NotNil(t, data)

	assert.EqualValues(t, 21, CountVisibleTrees(data))
}
