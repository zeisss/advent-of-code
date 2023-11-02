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
	log.Printf("Number of visible trees: %d", CountTotalVisibleTreesInGrid(g))
	log.Printf("Highest Scenic Score: %d", FindHeighestScenicScore(g))
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

func CountTotalVisibleTreesInGrid(g *Grid) int {
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

func FindHeighestScenicScore(g *Grid) int {
	score := 0
	for x := 0; x < g.Width; x++ {
		for y := 0; y < g.Height; y++ {
			if s := ScenicScore(g, x, y); s > score {
				score = s
			} else if score == s && s > 0 {
				log.Printf("equal scenic score: %d, %d => %d", x, y, s)
			}
		}
	}
	return score
}

func TreeIsInvisible(g *Grid, x, y int) bool {
	return countTreesUntilEqualOrHigher(g, x, y, -1, 0) == x &&
		countTreesUntilEqualOrHigher(g, x, y, 1, 0)-x == 0 &&
		countTreesUntilEqualOrHigher(g, x, y, 0, -1) == y &&
		countTreesUntilEqualOrHigher(g, x, y, 0, +1)-y == 0
}

func ScenicScore(g *Grid, x, y int) int {
	return countTreesUntilEqualOrHigher(g, x, y, -1, 0) *
		countTreesUntilEqualOrHigher(g, x, y, 1, 0) *
		countTreesUntilEqualOrHigher(g, x, y, 0, -1) *
		countTreesUntilEqualOrHigher(g, x, y, 0, +1)
}

func countTreesUntilEqualOrHigher(g *Grid, srcX, srcY, xMod, yMod int) int {
	targetHeight := g.HeightAt(srcX, srcY)

	x := srcX + xMod
	y := srcY + yMod
	trees := 0
	for x >= 0 && x < g.Width && y >= 0 && y < g.Height {
		trees++
		if g.HeightAt(x, y) >= targetHeight {
			break
		}
		x += xMod
		y += yMod
	}
	return trees
}
