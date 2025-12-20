package main

import (
	"fmt"
	"slices"
)

const exampleInput = `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `

func ExampleOperand() {
	input := []Value{1, 2, 3}
	fmt.Println("multiply:", MultiplyOperand.Reduce(slices.Values(input)))
	fmt.Println("sum:", AdditionOperand.Reduce(slices.Values(input)))
	// Output:
	// multiply: 6
	// sum: 6
}

func ExamplePart1() {
	fmt.Println(Part1(exampleInput))
	// Output: 4277556
}
