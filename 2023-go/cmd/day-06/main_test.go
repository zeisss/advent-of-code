package main

import (
	"fmt"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = `Time:      7  15   30
Distance:  9  40  200`

func ExamplePart1() {
	fmt.Println("Example Part 1:", Part1(internal.Must(Parse(INPUT))))
	fmt.Println("Player Part 1:", Part1(internal.Must(Parse(internal.MustReadFileNoSplit("../../testdata/day-06.txt")))))

	// Output:
	// Example Part 1: 288
	// Player Part 1: 500346
}

func ExamplePart2() {
	fmt.Println("Player Part 2: ", CalculateOptions(Race{
		Milliseconds:   51926890,
		DistanceRecord: 222203111261225,
	}))

	// Output: Playert Part 2: 42515755
}

func ExampleParse() {
	races := internal.Must(Parse(INPUT))
	for i, race := range races {
		options := CalculateOptions(race)
		fmt.Printf("Race #%d: time=%d distance=%d <=> options=%d\n", i+1, race.Milliseconds, race.DistanceRecord, options)
	}

	// Output:
	// Race #1: time=7 distance=9 <=> options=4
	// Race #2: time=15 distance=40 <=> options=8
	// Race #3: time=30 distance=200 <=> options=9
}

func ExampleCalcRange() {
	raceTime := int64(7)
	for i := int64(0); i <= raceTime; i++ {
		fmt.Printf("press=%d => range=%d\n", i, CalcRange(i, raceTime))
	}

	// Output:
	// press=0 => range=0
	// press=1 => range=6
	// press=2 => range=10
	// press=3 => range=12
	// press=4 => range=12
	// press=5 => range=10
	// press=6 => range=6
	// press=7 => range=0
}
