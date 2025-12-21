package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

type Manifold struct {
	Width int
	Start int
	// Lines is sorted
	Lines []Line
}
type Line []bool

func ParseManifold(input string) Manifold {
	lines := strings.Split(input, "\n")
	var result Manifold

	result.Start = strings.IndexRune(lines[0], 'S')
	result.Width = len(lines[0])

	for _, line := range lines[1:] {
		var manifoldLine Line = make([]bool, result.Width)
		for pos, r := range line {
			switch r {
			case '^':
				manifoldLine[pos] = true
			}
		}
		result.Lines = append(result.Lines, manifoldLine)
	}
	return result
}

func CountTachyonSplits(manifold Manifold) int {
	beams := make([]bool, manifold.Width)
	beams[manifold.Start] = true

	var splits int

	for _, line := range manifold.Lines {
		log.Println("Line", line)
		newBeams := make([]bool, manifold.Width)
		for pos, hasBeam := range beams {
			if !hasBeam {
				continue
			}
			isSplitter := line[pos]
			if isSplitter {
				log.Println("Tachyon Beam split at", pos)
				newBeams[pos-1] = true
				newBeams[pos+1] = true
				splits++
			} else {
				newBeams[pos] = true
			}
		}
		beams = newBeams
	}
	return splits
}

func Part1(input string) int {
	manifold := ParseManifold(input)
	log.Println(manifold)
	return CountTachyonSplits(manifold)
}

func main() {
	fmt.Println("Advent of Code 2025 - Day 07")
	input, err := os.ReadFile("./day07/input.txt")
	if err != nil {
		panic(err)
	}
	fmt.Println("Part 1", Part1(string(input)))
}
