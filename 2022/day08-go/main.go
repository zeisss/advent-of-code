package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile("./testdata/input.txt")
	if err != nil {
		log.Fatalf("ERROR(readfile): %v", err)
	}
	g, err := Parse(string(data))
	if err != nil {
		log.Fatalf("ERROR(parse): %v", err)
	}
	log.Printf("Number of visible trees: %d", CountVisibleTrees(g))
}
func Parse(input string) (*Grid, error) {

	trees := make([][]int, 0, 10)

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		var heights []int
		for c := 0; c < len(line); c++ {
			s := line[c : c+1]

			n, err := strconv.ParseInt(s, 10, 32)
			if err != nil {
				return nil, err
			}

			heights = append(heights, int(n))
		}
		trees = append(trees, heights)
	}

	g := &Grid{
		Trees:  trees,
		Width:  len(trees[0]),
		Height: len(trees),
	}
	return g, nil
}

type Grid struct {
	Width  int
	Height int
	Trees  [][]int
}

func (g Grid) HeightAt(x, y int) int {
	return g.Trees[y][x]
}

func CountVisibleTrees(g *Grid) int {
	visible := g.Width + g.Width + g.Height - 2 + g.Height - 2 // the border is always visible

	for y := 1; y < g.Height-1; y++ {
		for x := 1; x < g.Width-1; x++ {
			if !TreeIsInvisible(g, x, y) {
				visible++
			}
		}
	}
	return visible
}

func TreeIsInvisible(g *Grid, x, y int) bool {
	targetHeight := g.HeightAt(x, y)

	anyTreeHeigherOrEqual := func(xMod, yMod int) bool {
		x := x + xMod
		y := y + yMod
		for x >= 0 && x < g.Width && y >= 0 && y < g.Height {
			if g.HeightAt(x, y) >= targetHeight {
				return true
			}
			x += xMod
			y += yMod
		}
		return false
	}

	return anyTreeHeigherOrEqual(-1, 0) && anyTreeHeigherOrEqual(1, 0) &&
		anyTreeHeigherOrEqual(0, -1) && anyTreeHeigherOrEqual(0, +1)
}
