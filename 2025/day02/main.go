package main

import (
	"fmt"
	"iter"
	"strings"

	helpers "github.com/zeisss/advent-of-code/2025"
)

type Range struct{ From, To int64 }

func parseInput(input string) RangeList {
	var ranges []Range

	pairs := strings.Split(input, ",")
	ranges = make([]Range, len(pairs))
	for i, pair := range pairs {
		fmt.Sscanf(pair, "%d-%d", &ranges[i].From, &ranges[i].To)
	}
	return ranges
}

func (r Range) All() iter.Seq[int64] {
	return func(yield func(int64) bool) {
		for n := r.From; n <= r.To; n++ {
			if !yield(n) {
				return
			}
		}
	}
}

func (r Range) String() string {
	return fmt.Sprintf("%d-%d", r.From, r.To)
}

type RangeList []Range

func (rs RangeList) All() iter.Seq[int64] {
	return func(yield func(int64) bool) {
		for _, rg := range rs {
			for n := range rg.All() {
				if !yield(n) {
					return
				}
			}
		}
	}
}

func filterInvalidIDs(it iter.Seq[int64], filter func(int64) bool) iter.Seq[int64] {
	return func(yield func(int64) bool) {
		for id := range it {
			if filter(id) {
				if !yield(id) {
					return
				}
			}
		}
	}
}

func Part1InvalidIDPolicy(id int64) bool {
	sid := fmt.Sprintf("%d", id)
	mid := len(sid) / 2
	return sid[:mid] == sid[mid:]
}

func Part2InvalidIDPolicy(id int64) bool {
	sid := fmt.Sprintf("%d", id)

	if len(sid) <= 1 {
		return false
	}

	for i := 1; i < len(sid); i++ {
		if isRepeatedStringMatching(sid[:i], sid) {
			return true
		}
	}
	return false
}

func isRepeatedStringMatching(pattern, full string) bool {
	multiplier := int(len(full) / len(pattern))
	// length doesn't exactly match
	if multiplier*len(pattern) != len(full) {
		return false
	}
	return strings.Repeat(pattern, multiplier) == full
}

func Part1(input string) int64 {
	return helpers.Sum(filterInvalidIDs(
		parseInput(input).All(),
		Part1InvalidIDPolicy,
	))
}

func Part2(input string) int64 {
	return helpers.Sum(filterInvalidIDs(
		parseInput(input).All(),
		Part2InvalidIDPolicy,
	))
}

func main() {
	const input = `874324-1096487,6106748-6273465,1751-4283,294380-348021,5217788-5252660,828815656-828846474,66486-157652,477-1035,20185-55252,17-47,375278481-375470130,141-453,33680490-33821359,88845663-88931344,621298-752726,21764551-21780350,58537958-58673847,9983248-10042949,4457-9048,9292891448-9292952618,4382577-4494092,199525-259728,9934981035-9935011120,6738255458-6738272752,8275916-8338174,1-15,68-128,7366340343-7366538971,82803431-82838224,72410788-72501583`
	fmt.Println("Advent of Code 2025 - Day 02")

	fmt.Println("Part 1:", Part1(input))
	fmt.Println("Part 2:", Part2(input))
}
