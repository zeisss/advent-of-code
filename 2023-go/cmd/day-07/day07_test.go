package day07

import (
	"fmt"
	"sort"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = `32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483`

func ExampleParse() {
	hands := internal.Must(Parse(INPUT))
	PrintHands(hands)
	// Output:
	// {32T3K 765} => type=one-pair rank=1
	// {T55J5 684} => type=three-of-a-kind rank=2
	// {KK677 28} => type=two-pair rank=3
	// {KTJJT 220} => type=two-pair rank=4
	// {QQQJA 483} => type=three-of-a-kind rank=5
}

func ExamplePart1() {
	fmt.Println("Example Part 1:", Part1(internal.Must(Parse(INPUT))))
	fmt.Println("Player Part 1:", Part1(internal.Must(Parse(internal.MustReadFileNoSplit("../../testdata/day-07.txt")))))

	// Output:
	// Example Part 1: 6440
	// Player Part 1: 250946742
}

func ExampleSort() {
	hands := internal.Must(Parse(INPUT))
	sort.Sort(SortByType(hands))

	PrintHands(hands)
	// Output:
	// {32T3K 765} => type=one-pair rank=1
	// {KTJJT 220} => type=two-pair rank=2
	// {KK677 28} => type=two-pair rank=3
	// {T55J5 684} => type=three-of-a-kind rank=4
	// {QQQJA 483} => type=three-of-a-kind rank=5
}

func ExampleHand_Type() {
	inputs := []string{
		"AAAAA",
		"AA8AA",
		"23332",
		"TTT98",
		"23432",
		"A23A4",
		"23456",
	}
	for _, s := range inputs {
		fmt.Println(s, "=", Hand{Labels: s}.Type())
	}

	// Output:
	// AAAAA = five-of-a-kind
	// AA8AA = four-of-a-kind
	// 23332 = full-house
	// TTT98 = three-of-a-kind
	// 23432 = two-pair
	// A23A4 = one-pair
	// 23456 = high-card
}

func ExampleSortBy() {
	hands := []Hand{
		{Labels: "33332"},
		{Labels: "2AAAA"},

		{Labels: "77788"},
		{Labels: "77888"},

		{Labels: "KK677"},
		{Labels: "KTJJT"},
	}
	sort.Sort(SortByType(hands))
	PrintHands(hands)

	// Output:
	// {KTJJT 0} => type=two-pair rank=1
	// {KK677 0} => type=two-pair rank=2
	// {77788 0} => type=full-house rank=3
	// {77888 0} => type=full-house rank=4
	// {2AAAA 0} => type=four-of-a-kind rank=5
	// {33332 0} => type=four-of-a-kind rank=6
}

func PrintHands(hands []Hand) {
	for i, hand := range hands {
		fmt.Printf("%v => type=%v rank=%d\n", hand, hand.Type(), i+1)
	}
}
