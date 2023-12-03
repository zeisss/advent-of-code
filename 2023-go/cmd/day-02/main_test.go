package main

import (
	"fmt"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var EXAMPLE = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

func ExampleUser() {
	input := internal.MustReadFile("../../testdata/day-02.txt")
	fmt.Println("Part 1:", SumGameIDsWithCondition(input, Part1))
	fmt.Println("Part 1:", SumGameIDsWithCondition(input, Part2))
	// Output:
	// Part 1: 2285
	// Part 2: 77021
}
func ExamplePart1() {
	fmt.Println("Part 1:", SumGameIDsWithCondition(strings.Split(EXAMPLE, "\n"), Part1))
	// Output: Part 1: 8
}

func ExamplePart2() {
	fmt.Println(Part2(ParseLine("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")))
	fmt.Println("Sum of Power:", SumGameIDsWithCondition(strings.Split(EXAMPLE, "\n"), Part2))
	// Output:
	// 48
	// Sum of Power: 2286
}

func ExampleParseLine() {
	g := ParseLine("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
	fmt.Println(g)
	// Output:
	// {1 [{4 0 3} {1 2 6} {0 2 0}]}
}
