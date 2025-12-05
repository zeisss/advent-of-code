package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStringToIntSlices(t *testing.T) {
	assert.Equal(t, []int{1, 2, 3, 4}, StringToIntSlice("1234"))
	assert.Equal(t, []int{0, 5, 1, 2, 3, 4}, StringToIntSlice("051234"))
}

func TestAcceptanceExamples(t *testing.T) {
	assert.Equal(t, 1928, Checksum("2333133121414131402"))
}
