package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

const exampleInput = `3-5
10-14
16-20
12-18

1
5
8
11
17
32`

func ExamplePart1() {
	fmt.Println(Part1(exampleInput))
	// Output: 3
}

func TestParseDatabase(t *testing.T) {
	db := ParseDatabase(exampleInput)
	assert.Len(t, db.Fresh, 4)
	assert.Len(t, db.Ingredients, 6)
}
