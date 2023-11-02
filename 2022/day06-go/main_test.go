package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindMarker(t *testing.T) {
	assert.Equal(t, 4, FindMarker("abcd"))
	assert.Equal(t, 4, FindMarker("abcde"))
	assert.Equal(t, 5, FindMarker("aabcd"))
	assert.Equal(t, 5, FindMarker("abcad"))
	assert.Equal(t, 7, FindMarker("abccdef"))

	// Examples
	assert.Equal(t, 7, FindMarker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))
	assert.Equal(t, 5, FindMarker("bvwbjplbgvbhsrlpgdmjqwftvncz"))
	assert.Equal(t, 6, FindMarker("nppdvjthqldpwncqszvftbrmjlhg"))
	assert.Equal(t, 10, FindMarker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
	assert.Equal(t, 11, FindMarker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
}
