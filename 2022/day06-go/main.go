package main

import (
	"log"
	"os"
)

func main() {
	s, err := os.ReadFile("./testdata/input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}
	log.Println(FindUniqueMarkerPosition(string(s), 14))
}

func FindUniqueMarkerPosition(input string, length int) int {
	for i := 0; i <= len(input)-length; i++ {
		if HasDuplicateCharacters(input[i : i+length]) {
			continue
		}

		// we processed the loop without a continue, everything should be good!
		return i + length
	}
	return -1
}

func HasDuplicateCharacters(s string) bool {
	bytes := [256]byte{}

	for _, b := range []byte(s) {
		if bytes[b] > 0 {
			// we got a duplicate, lets start over with the next start position
			return true
		}
		bytes[b]++
	}
	return false
}
