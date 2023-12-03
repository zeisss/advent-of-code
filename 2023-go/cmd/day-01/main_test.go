package main

import (
	"fmt"
	"strings"
	"testing"

	"github.com/zeisss/advent-of-code/2023-go/internal"

	"github.com/stretchr/testify/assert"
)

var INPUT = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`

var EXAMPLETWO = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`

func ExampleINPUT() {
	n := Reduce(strings.Split(INPUT, "\n"), DigitCandidates)
	fmt.Println("Test Input 1:", n)

	n = Reduce(strings.Split(EXAMPLETWO, "\n"), WordAndDigitCandidates)
	fmt.Println("Test Input 2:", n)

	// Output:
	// Test Input 1: 142
	// Test Input 2: 281
}

func ExampleDigitCandidates() {
	lines := internal.MustReadFile("../../testdata/day-01.txt")
	fmt.Println("Part 1:", Reduce(lines, DigitCandidates))
	fmt.Println("Part 2:", Reduce(lines, WordAndDigitCandidates))

	// Output:
	// Part 1: 53334
	// Part 2: 52834
}

func ExampleWordAndDigitCandidates() {
	for _, i := range strings.Split(EXAMPLETWO, "\n") {
		a, b := WordAndDigitCandidates(i)
		fmt.Println(a, b)
	}

	// Output:
	// 2 9
	// 8 3
	// 1 3
	// 2 4
	// 4 2
	// 1 4
	// 7 6
}

func TestWordAndDigitCandidates_More(t *testing.T) {
	s := "ljnff279"
	a, b := WordAndDigitCandidates(s)
	fmt.Printf("%s => %d%d\n", s, a, b)
	assert.Equal(t, 6, a)
	assert.Equal(t, 6, b)
}
