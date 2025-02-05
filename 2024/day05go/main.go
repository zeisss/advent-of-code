package main

import (
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

/*
--- Part Two ---
While the Elves get to work printing the correctly-ordered updates, you have a little time to fix the rest of them.

For each of the incorrectly-ordered updates, use the page ordering rules to put the page numbers in the right order. For the above example, here are the three incorrectly-ordered updates and their correct orderings:

75,97,47,61,53 becomes 97,75,47,61,53.
61,13,29 becomes 61,29,13.
97,13,75,29,47 becomes 97,75,47,29,13.
After taking only the incorrectly-ordered updates and ordering them correctly, their middle page numbers are 47, 29, and 47. Adding these together produces 123.

Find the updates which are not in the correct order. What do you get if you add up the middle page numbers after correctly ordering just those updates?
*/

type Rules = map[int64][]int64
type Update = []int64
type Updates = []Update

func main() {
	d, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}

	rules, updates := Parse(string(d))
	p1 := Part1(rules, updates)
	p2 := Part2(rules, updates)

	log.Printf("Part 1: %d", p1)
	log.Printf("Part 2: %d", p2)
}

func Part1(rules Rules, updates Updates) int64 {
	var output int64
	for _, update := range updates {
		if IsCorrectlyOrdered(rules, update) {
			output += MiddlePageNumber(update)
		}
	}
	return output
}

func Part2(rules Rules, updates Updates) int64 {
	var output int64
	for _, update := range updates {
		if !IsCorrectlyOrdered(rules, update) {
			output += MiddlePageNumber(FixUpdate(rules, update))
		}
	}
	return output
}

func MiddlePageNumber(update Update) int64 {
	return update[len(update)/2]
}

func IsCorrectlyOrdered(rules Rules, update Update) bool {
	for index, pageNumber := range update {
		mayNotBeBeforeN, ok := rules[pageNumber]
		if !ok {
			// valid
			continue
		}

		onlyLeftNumbers := update[0:index]

		// check each rule individually
		for _, mustBeAfter := range mayNotBeBeforeN {
			if slices.Contains(onlyLeftNumbers, mustBeAfter) {
				return false
			}
		}

	}

	return true
}

func FixUpdate(rules Rules, update Update) Update {
	var fixedUpdate Update = update

	slices.SortFunc(fixedUpdate, func(a, b int64) int {
		nextPages := rules[a]
		if slices.Contains(nextPages, b) {
			return -1
		} else {
			return 0
		}
	})

	return fixedUpdate
}

func Parse(input string) (Rules, Updates) {
	parts := strings.Split(input, "\n\n")
	return ParseRules(parts[0]), ParseUpdates(parts[1])
}

func ParseRules(input string) Rules {
	rules := Rules{}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		ss := strings.Split(line, "|")

		i, err := strconv.ParseInt(ss[0], 10, 64)
		if err != nil {
			panic(err)
		}
		j, err := strconv.ParseInt(ss[1], 10, 64)
		if err != nil {
			panic(err)
		}

		if _, ok := rules[i]; !ok {
			rules[i] = []int64{j}
		} else {
			rules[i] = append(rules[i], j)
		}
	}

	return rules
}

func ParseUpdates(input string) Updates {
	updates := Updates{}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		ss := strings.Split(line, ",")
		var update Update

		for _, page := range ss {
			n, err := strconv.ParseInt(page, 10, 64)
			if err != nil {
				panic(err)
			}
			update = append(update, n)
		}

		updates = append(updates, update)
	}
	return updates
}
