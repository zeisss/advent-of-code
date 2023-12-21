package main

import (
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

type Sequence []int

func (seq Sequence) AllZero() bool {
	for _, i := range seq {
		if i != 0 {
			return false
		}
	}
	return true
}

func Parse(input string) []Sequence {
	var result []Sequence

	for _, line := range strings.Split(input, "\n") {
		var seq Sequence
		for _, field := range strings.Fields(line) {
			seq = append(seq, internal.MustAtoi(field))
		}
		result = append(result, seq)
	}

	return result
}

func Derive(seq Sequence) Sequence {
	if seq.AllZero() {
		panic("derive called on all-zero")
	}

	var out Sequence
	first := seq[0]

	for _, i := range seq[1:] {
		out = append(out, i-first)
		first = i
	}

	return out
}

func DeriveAndExtend(start Sequence) []Sequence {
	tree := []Sequence{start}
	last := start

	// Build tree until last line is all zero
	for !last.AllZero() {
		next := Derive(last)
		tree = append(tree, next)
		last = next
	}

	// Add a zero to the last line
	tree[len(tree)-1] = append(tree[len(tree)-1], 0)

	// Now walk up the tree and derive the next number from the previous line
	for row := len(tree) - 2; row >= 0; row-- {
		prev := tree[row+1]
		line := tree[row]

		// n := firstElement(prev) + firstElement(line)

		n := lastElement(prev) + lastElement(line)
		tree[row] = append(line, n)
		// tree[row] = append(Sequence{n}, line...)
	}

	return tree
}

func DeriveAndExtendFront(start Sequence) []Sequence {
	tree := []Sequence{start}
	last := start

	// Build tree until last line is all zero
	for !last.AllZero() {
		next := Derive(last)
		tree = append(tree, next)
		last = next
	}

	// Add a zero to the last line
	tree[len(tree)-1] = append(tree[len(tree)-1], 0)

	// Now walk up the tree and derive the next number from the previous line
	for row := len(tree) - 2; row >= 0; row-- {
		prev := tree[row+1]
		line := tree[row]

		n := firstElement(line) - firstElement(prev)

		// tree[row] = append(line, lastElement(line)+lastElement(prev))
		tree[row] = append(Sequence{n}, line...)
	}

	return tree
}

func firstElement[E any](e []E) E {
	return e[0]
}

func lastElement[E any](e []E) E {
	return e[len(e)-1]
}

func Part1(sequences []Sequence) int {
	var sum int

	for _, seq := range sequences {
		tree := DeriveAndExtend(seq)
		sum += lastElement(tree[0])
	}

	return sum
}

func Part2(sequences []Sequence) int {
	var sum int

	for _, seq := range sequences {
		tree := DeriveAndExtendFront(seq)
		sum += firstElement(tree[0])
	}

	return sum
}
