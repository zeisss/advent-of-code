package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const exampleInput = `3-5
10-14
16-20
12-18

1
5
8
11
17
32`

const longerInput = `415615768268371-416146851443768
191441934518457-197157487694725
283039808754572-287535623375734
112431539028123-113253150554096
459000186532687-459000186532687`

func ExamplePart1() {
	fmt.Println(Part1(exampleInput))
	// Output: 3
}

func ExamplePart2() {
	fmt.Println(Part2(exampleInput))
	// Output: 14
}

func TestRange_Count(t *testing.T) {
	subject := Range{From: 10, To: 12}
	require.EqualValues(t, 3, subject.Count())
}

func TestParseDatabase(t *testing.T) {
	db := ParseDatabase(exampleInput)
	assert.Len(t, db.FreshRanges, 4)
	assert.Len(t, db.Inventory, 6)
}

func BenchmarkDatabase_FreshByRanges_shortExample(b *testing.B) {
	db := ParseDatabase(exampleInput)
	b.ResetTimer()
	for b.Loop() {
		_ = db.CountFreshByRanges()
	}
}

func BenchmarkDatabase_FreshByRanges_longerInput(b *testing.B) {
	db := ParseDatabase(longerInput)
	b.ResetTimer()
	for b.Loop() {
		_ = db.CountFreshByRanges()
	}
}
