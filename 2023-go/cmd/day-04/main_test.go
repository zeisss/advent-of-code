package main

import (
	"fmt"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = strings.Split(`Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`, "\n")

func ExampleINPUT() {
	fmt.Println("Part 1: Total Points:", MustParse(INPUT).TotalPoints())
	// Output:
	// Part 1: Total Points: 13
}
func ExampleDay04Input() {
	fmt.Println("Part 1: Total Points:", MustParse(internal.MustReadFile("../../testdata/day-04.txt")).TotalPoints())
	// Output:
	// Part 1: Total Points: 20829
}

func ExampleParseCard() {
	card := MustParseCard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
	fmt.Println("card:", card)
	fmt.Println("points:", card.Points())

	// Output:
	// card: {1 [41 48 83 86 17] [83 86 6 31 17 9 48 53]}
	// points: 8
}
