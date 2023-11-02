package main

import (
	"log"
	"os"
)

func main() {
	s, err := ParseFile("testdata/input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}
	log.Println(FindMarker(s))
}

func ParseFile(s string) (string, error) {
	data, err := os.ReadFile(s)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func FindMarker(input string) int {
	for i := 0; i <= len(input)-4; i++ {
		if input[i] == input[i+1] ||
			input[i] == input[i+2] ||
			input[i] == input[i+3] ||
			input[i+1] == input[i+2] ||
			input[i+1] == input[i+3] ||
			input[i+2] == input[i+3] {
			continue
		}
		return i + 4
	}

	return -1
}
