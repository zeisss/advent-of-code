package main

import (
	"fmt"
	"iter"
	"strings"
)

type Range struct{ From, To int }

func parseInput(input string) []Range {
	var ranges []Range

	pairs := strings.Split(input, ",")
	ranges = make([]Range, len(pairs))
	for i, pair := range pairs {
		fmt.Sscanf(pair, "%d-%d", &ranges[i].From, &ranges[i].To)
	}
	return ranges
}

func IsInvalidID(id int) bool {
	sid := fmt.Sprintf("%d", id)
	if len(sid)%2 == 1 {
		return false
	}
	mid := len(sid) / 2
	return sid[:mid] == sid[mid:]
}

func emitNumbers(ranges []Range) iter.Seq[int] {
	return func(yield func(int) bool) {
		for _, rg := range ranges {
			for n := rg.From; n <= rg.To; n++ {
				if IsInvalidID(n) {
					if !yield(n) {
						return
					}
				}
			}
		}
	}
}

func sum(it iter.Seq[int]) int {
	total := 0
	for v := range it {
		total += v
	}
	return total
}

func Part1(input string) int {
	return sum(emitNumbers(parseInput(input)))
}

func main() {
	const input = `874324-1096487,6106748-6273465,1751-4283,294380-348021,5217788-5252660,828815656-828846474,66486-157652,477-1035,20185-55252,17-47,375278481-375470130,141-453,33680490-33821359,88845663-88931344,621298-752726,21764551-21780350,58537958-58673847,9983248-10042949,4457-9048,9292891448-9292952618,4382577-4494092,199525-259728,9934981035-9935011120,6738255458-6738272752,8275916-8338174,1-15,68-128,7366340343-7366538971,82803431-82838224,72410788-72501583`
	fmt.Println("Advent of Code 2025 - Day 02")

	fmt.Println("Part 1:", Part1(input))
}
