package main

import (
	"fmt"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
`

func ExampleDay03Txt() {
	grid := MustParseGrid(internal.MustReadFile("../../testdata/day-03.txt"))
	fmt.Println("Sum Part Numbers:", grid.SumPartNumbers())

	// Output: Sum Part Numbers: 530849
}

func ExampleINPUT() {
	grid := MustParseGrid(strings.Split(INPUT, "\n"))
	fmt.Println("Sum Part Numbers:", grid.SumPartNumbers())

	// Output: Sum Part Numbers: 4361
}

func TestGridIsPartAround(t *testing.T) {
	g := MustParseGrid([]string{"01234*"})
	require.False(t, g.IsPartAt(4, 0))
	require.True(t, g.IsPartAt(5, 0))
	require.True(t, g.IsPartAround(4, 0))

	g = MustParseGrid([]string{"*1234"})
	require.True(t, g.IsPartAt(0, 0))
	require.False(t, g.IsPartAt(1, 0))
	require.False(t, g.IsPartAt(4, 0))
	require.False(t, g.IsPartAround(4, 0))
	require.True(t, g.IsPartAround(1, 0))
	require.EqualValues(t, 1234, g.SumPartNumbers())

	g = MustParseGrid([]string{"012", "..$"})
	require.False(t, g.IsPartAround(0, 0))
	require.True(t, g.IsPartAround(1, 0))
	require.True(t, g.IsPartAround(2, 0))
}

func TestSumParts(t *testing.T) {
	g := MustParseGrid([]string{".*1", "2.."})
	require.EqualValues(t, 3, g.SumPartNumbers())
}

func TestRuneToInt(t *testing.T) {
	if runeToInt('1') != 1 {
		t.Fail()
	}
}
