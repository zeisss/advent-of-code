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

	assert.EqualValues(t, 21, CountTotalVisibleTreesInGrid(data))
}

func TestCountSmallerTreesUntilEqualOrHigher(t *testing.T) {
	data, err := Parse(TEST_INPUT)
	require.NoError(t, err)
	require.NotNil(t, data)

	x := 2
	y := 1
	assert.EqualValues(t, 1, countTreesUntilEqualOrHigher(data, x, y, 0, -1))
	assert.EqualValues(t, 1, countTreesUntilEqualOrHigher(data, x, y, -1, 0))
	assert.EqualValues(t, 2, countTreesUntilEqualOrHigher(data, x, y, +1, 0))
	assert.EqualValues(t, 2, countTreesUntilEqualOrHigher(data, x, y, 0, +1))

	x = 2
	y = 3
	assert.EqualValues(t, 2, countTreesUntilEqualOrHigher(data, x, y, 0, -1))
	assert.EqualValues(t, 2, countTreesUntilEqualOrHigher(data, x, y, -1, 0))
	assert.EqualValues(t, 1, countTreesUntilEqualOrHigher(data, x, y, 0, +1))
	assert.EqualValues(t, 2, countTreesUntilEqualOrHigher(data, x, y, +1, 0))
}

func TestScenicScore(t *testing.T) {
	data, err := Parse(TEST_INPUT)
	require.NoError(t, err)
	require.NotNil(t, data)

	assert.EqualValues(t, 4, ScenicScore(data, 2, 1))
	assert.EqualValues(t, 8, ScenicScore(data, 2, 3))
}

func TestFindHighestScenicScore(t *testing.T) {
	data, err := Parse(TEST_INPUT)
	require.NoError(t, err)
	require.NotNil(t, data)

	assert.EqualValues(t, 8, FindHeighestScenicScore(data))
}
