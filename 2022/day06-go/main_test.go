package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindUniqueMarkerPosition_Length4(t *testing.T) {
	assert.Equal(t, 4, FindUniqueMarkerPosition("abcd", 4))
	assert.Equal(t, 4, FindUniqueMarkerPosition("abcde", 4))
	assert.Equal(t, 5, FindUniqueMarkerPosition("aabcd", 4))
	assert.Equal(t, 5, FindUniqueMarkerPosition("abcad", 4))
	assert.Equal(t, 7, FindUniqueMarkerPosition("abccdef", 4))

	// Examples
	assert.Equal(t, 7, FindUniqueMarkerPosition("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4))
	assert.Equal(t, 5, FindUniqueMarkerPosition("bvwbjplbgvbhsrlpgdmjqwftvncz", 4))
	assert.Equal(t, 6, FindUniqueMarkerPosition("nppdvjthqldpwncqszvftbrmjlhg", 4))
	assert.Equal(t, 10, FindUniqueMarkerPosition("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4))
	assert.Equal(t, 11, FindUniqueMarkerPosition("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4))
}

func TestFindUniqueMarkerPosition_Length14(t *testing.T) {
	// Examples
	assert.Equal(t, 19, FindUniqueMarkerPosition("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14))
	assert.Equal(t, 23, FindUniqueMarkerPosition("bvwbjplbgvbhsrlpgdmjqwftvncz", 14))
	assert.Equal(t, 23, FindUniqueMarkerPosition("nppdvjthqldpwncqszvftbrmjlhg", 14))
	assert.Equal(t, 29, FindUniqueMarkerPosition("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14))
	assert.Equal(t, 26, FindUniqueMarkerPosition("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14))
}

func TestHasDuplicateCharacters(t *testing.T) {
	assert.False(t, HasDuplicateCharacters("ab"), "ab should be false")
	assert.False(t, HasDuplicateCharacters("abcd"), "abcd should be false")
	assert.True(t, HasDuplicateCharacters("aa"), "aa should be true")
	assert.True(t, HasDuplicateCharacters("aba"), "aba should be true")
}
