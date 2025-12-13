package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func ExamplePart1() {
	const input = `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124`
	fmt.Println(Part1(input))
	// Output: 1227775554
}

func ExampleIsInvalidID() {
	// Output:
	// 11 true
	// 6464 true
	// 1010 true
	// 123123 true
	// 15 false
	// 999 false
	fmt.Println("11", IsInvalidID(11))
	fmt.Println("6464", IsInvalidID(6464))
	fmt.Println("1010", IsInvalidID(1010))
	fmt.Println("123123", IsInvalidID(123123))

	fmt.Println("15", IsInvalidID(15))
	fmt.Println("999", IsInvalidID(999))
}

func TestParseInput(t *testing.T) {
	const input = `11-22,95-115,998-1012`
	expected := []Range{
		{11, 22},
		{95, 115},
		{998, 1012},
	}
	result := parseInput(input)
	assert.Equal(t, expected, result)
}
