package main

import (
	"log"
	"os"
)

func Checksum(s string) int {
	numbers := StringToIntSlice(s)
	bytes := Sum(numbers)

	data := make([]byte, 0, bytes)
	for i, n := range numbers {
	}
	return 0
}

func main() {
	d, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}

	log.Printf("Part 1: %d", Checksum(string(d)))
	// log.Printf("Part 2: %d", p2)
}
