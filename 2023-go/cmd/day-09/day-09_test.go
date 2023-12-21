package main

import (
	"fmt"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = `0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45`

func ExampleINPUT() {
	seq := Parse(INPUT)

	fmt.Println(seq)

	// Output: [[0 3 6 9 12 15] [1 3 6 10 15 21] [10 13 16 21 30 45]]
}

func ExamplePrintSequenceTree() {
	seq := Parse(INPUT)

	s := seq[0]
	d1 := Derive(s)
	d2 := Derive(d1)
	printSequenceTree(s, d1, d2)

	// Output:
	// 0   3   6   9  12  15
	//    3   3   3   3   3
	//      0   0   0   0
}

func ExampleDeriveAndExtend() {
	seqs := Parse(INPUT)

	for _, seq := range seqs {
		tree := DeriveAndExtend(seq)
		printSequenceTree(tree...)
	}

	// Output:
	// 0   3   6   9  12  15  18
	//    3   3   3   3   3   3
	//      0   0   0   0   0
	//  1   3   6  10  15  21  28
	//    2   3   4   5   6   7
	//      1   1   1   1   1
	//        0   0   0   0
	// 10  13  16  21  30  45  68
	//    3   3   5   9  15  23
	//      0   2   4   6   8
	//        2   2   2   2
	//          0   0   0
}

func ExampleDeriveAndExtendFront() {
	seqs := Parse(INPUT)
	tree := DeriveAndExtendFront(seqs[2])
	printSequenceTree(tree...)

	// Output:
	// 5  10  13  16  21  30  45
	//    5   3   3   5   9  15
	//     -2   0   2   4   6
	//        2   2   2   2
	//          0   0   0
}

func printSequenceTree(seqs ...Sequence) {
	for i, seq := range seqs {
		fmt.Print(strings.Repeat("  ", i))
		for index, j := range seq {
			if index > 0 {
				fmt.Print("  ")
			}
			fmt.Printf("%2d", j)
		}
		fmt.Println()
	}
}

func ExamplePart1() {
	seqs := Parse(INPUT)
	fmt.Println("Example:", Part1(seqs))

	fmt.Printf("Player Input: %d", Part1(Parse(internal.MustReadFileNoSplit("../../testdata/day-09.txt"))))

	// Output:
	// Example: 114
	// Player Input: 1731106378
}

func ExamplePart2() {
	seqs := Parse(INPUT)
	fmt.Println("Example:", Part2(seqs))

	fmt.Printf("Player Input: %d", Part2(Parse(internal.MustReadFileNoSplit("../../testdata/day-09.txt"))))

	// Output:
	// Example: 2
	// Player Input: 1087
}
