package main

import "fmt"

const exampleInput = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`

func ExamplePart1() {
	fmt.Println(Part1(exampleInput))
	// Output: 13
}

func ExamplePart2() {
	fmt.Println(Part2(exampleInput))
	// Output: 43
}
