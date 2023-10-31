package main

import (
	"bufio"
	"bytes"
	"flag"
	"fmt"
	"log"
	"os"
)

func main() {
	flag.Parse()
	file := flag.Arg(0)
	if file == "" {
		file = "testdata/input.txt"
	}
	if err := processFile(file); err != nil {
		log.Printf("ERROR: %v", err)
	}
}

func processFile(filename string) error {
	pairs, err := ParseFile(filename)
	if err != nil {
		return err
	}

	log.Printf("Pairs: %v", len(pairs))
	task1, task2 := check(pairs)
	log.Printf("Complete overlaps: %d", task1)
	log.Printf("Partial overlap: %d", task2)
	return nil
}

func check(pairs []Pair) (int, int) {
	var task1 int
	var task2 int
	for _, pair := range pairs {
		if CompleteOverlap(pair) {
			task1++
		}
		if Overlap(pair) {
			task2++
		}
	}
	return task1, task2
}

type Range struct {
	Start int
	End   int
}

type Pair struct {
	First  Range
	Second Range
}

func Overlap(p Pair) bool {
	if p.First.End < p.Second.Start || p.Second.End < p.First.Start {
		return false
	}
	return true
}

func CompleteOverlap(p Pair) bool {
	return Inside(p.First, p.Second) || Inside(p.Second, p.First)
}

func Inside(small, big Range) bool {
	return big.Start <= small.Start && small.End <= big.End
}

func ParseFile(filename string) ([]Pair, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, fmt.Errorf("read %s: %w", filename, err)
	}

	return ParseData(data)
}

func ParseData(data []byte) ([]Pair, error) {
	var result []Pair
	scanner := bufio.NewScanner(bytes.NewReader(data))
	for scanner.Scan() {
		line := scanner.Text() // format: a-b,c-d
		if line == "" {
			break
		}

		var pair Pair
		if err := ParseLine(line, &pair); err != nil {
			return nil, err
		}
		result = append(result, pair)
	}
	return result, nil
}

func ParseLine(line string, pair *Pair) error {
	_, err := fmt.Sscanf(line, "%d-%d,%d-%d", &pair.First.Start, &pair.First.End, &pair.Second.Start, &pair.Second.End)
	if err != nil {
		return err
	}
	return nil
}
