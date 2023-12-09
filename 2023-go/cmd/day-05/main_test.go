package main

import (
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var INPUT = strings.Split(`seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4`, "\n")

func ExampleAlmanac_Resolve() {
	almanac := MustParseAlmanac(INPUT)
	// fmt.Println(almanac)
	for _, seed := range almanac.Seeds {
		m := almanac.Resolve(seed)
		// fmt.Println(m)
		PrintResolvedPath(os.Stdout, m)
	}
	fmt.Println("Lowest location:", almanac.FindLowestSeedLocation())
	// Output:
	// Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
	// Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
	// Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
	// Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
	// Lowest location: 35
}

func ExampleDay04Input() {
	a := MustParseAlmanac(internal.MustReadFile("../../testdata/day-05.txt"))
	fmt.Println("Part 1:", a.FindLowestSeedLocation())

	// Output:
	// Part 1: 240320250
}

func PrintResolvedPath(w io.Writer, m map[string]int) {
	fmt.Fprintf(w,
		"Seed %d, soil %d, fertilizer %d, water %d, light %d, temperature %d, humidity %d, location %d.\n",
		m["seed"],
		m["soil"],
		m["fertilizer"],
		m["water"],
		m["light"],
		m["temperature"],
		m["humidity"],
		m["location"],
	)
}
