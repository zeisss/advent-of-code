package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	f, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}
	input := Parse(string(f))
	p1 := EvalPart1(input)
	log.Printf("Part 1: %d\n", p1)
}

func Parse(input string) [][]int {
	ss := strings.Split(input, "\n")
	result := make([][]int, 2)
	for _, s := range ss {
		var a, b int
		_, err := fmt.Sscanf(s, "%d %d", &a, &b)
		if err != nil {
			panic(err)
		}
		result[0] = append(result[0], a)
		result[1] = append(result[1], b)
	}

	return result
}

func EvalPart1(input [][]int) int {
	return -1
}
