package main

import (
	"strings"
	"unicode"
)

func MustParseGrid(lines []string) Grid {
	return Grid{lines}
}

type Grid struct {
	lines []string
}

const ignore = "0123456789."

func (g Grid) SumPartNumbers() int64 {
	var sum int64
	for y, line := range g.lines {
		var (
			hasPart bool
			n       int64
		)
		checkToAdd := func() {
			if n != 0 && hasPart {
				// fmt.Println("Adding", n)
				sum += n
			} else if n > 0 {
				// fmt.Println("dropping", n)
			}
			n = 0
			hasPart = false
		}

		for x, r := range line {
			if !unicode.IsDigit(r) {
				checkToAdd()
				continue
			}

			// we found a digit: add to n and check for parts
			n = n*10 + runeToInt(r)
			hasPart = hasPart || g.IsPartAround(x, y)
		}
		checkToAdd()
	}
	return sum
}

func (g Grid) IsPartAround(x, y int) bool {
	return g.IsPartAt(x-1, y-1) ||
		g.IsPartAt(x-1, y) ||
		g.IsPartAt(x-1, y+1) ||
		g.IsPartAt(x, y-1) ||
		g.IsPartAt(x, y+1) ||
		g.IsPartAt(x+1, y-1) ||
		g.IsPartAt(x+1, y) ||
		g.IsPartAt(x+1, y+1)
}

func (g Grid) IsPartAt(x, y int) bool {
	if x < 0 || y < 0 || y >= len(g.lines) {
		return false
	}
	line := g.lines[y]
	if x >= len(line) {
		return false
	}
	return !strings.ContainsRune(ignore, rune(line[x]))
}

func runeToInt(r rune) int64 {
	return int64(r - '0')
}
