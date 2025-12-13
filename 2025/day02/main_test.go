package main

import (
	"fmt"
	"slices"
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

func ExamplePart2() {
	const input = `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124`
	fmt.Println(Part2(input))
	// Output: 4174379265
}

func TestPart2_detailed(t *testing.T) {
	example := func(s string, numbers ...int64) {
		t.Helper()
		t.Run("full/"+s, func(t *testing.T) {
			n := Part2(s)
			var expectedSum int64 = 0
			for _, v := range numbers {
				expectedSum += v
			}
			assert.EqualValuesf(t, expectedSum, n, "Part2(%q) = %d; expected %d",
				s,
				n,
				expectedSum)
		})

		t.Run("sequence/"+s, func(t *testing.T) {
			it := filterInvalidIDs(parseInput(s).All(), Part2InvalidIDPolicy)
			n := slices.Collect(it)
			assert.EqualValues(t, numbers, n)
		})
	}

	example("11-22", 11, 22)
	example("95-115", 99, 111)
	example("998-1012", 999, 1010)
	example("1188511880-1188511890", 1188511885)
	example("222220-222224", 222222)
	example("1698522-1698528")
	example("446443-446449", 446446)
	example("38593856-38593862", 38_593_859)
	example("565653-565659", 565_656)
	example("824824821-824824827", 824_824_824)
	example("2121212118-2121212124", 2_121_212_121)
}

func ExamplePart1InvalidIDPolicy() {
	// Output:
	// 11 true
	// 6464 true
	// 1010 true
	// 123123 true
	// 15 false
	// 999 false
	fmt.Println("11", Part1InvalidIDPolicy(11))
	fmt.Println("6464", Part1InvalidIDPolicy(6464))
	fmt.Println("1010", Part1InvalidIDPolicy(1010))
	fmt.Println("123123", Part1InvalidIDPolicy(123123))

	fmt.Println("15", Part1InvalidIDPolicy(15))
	fmt.Println("999", Part1InvalidIDPolicy(999))
}

func ExamplePart2InvalidIDPolicy() {
	// Output:
	// 12341234 true
	// 123123123 true
	// 1212121212 true
	// 1111111 true
	// 11 true
	// 22 true
	// 999 true
	// 1188511885 true
	// 222222 true
	// 824824824 true

	inputs := []int64{
		12341234,
		123123123,
		1212121212,
		1111111,
		11,
		22,
		999,
		1188511885,
		222222,
		824824824,
	}
	for _, n := range inputs {
		fmt.Println(n, Part2InvalidIDPolicy(n))
	}
}

func TestParseInput(t *testing.T) {
	const input = `11-22,95-115,998-1012`
	expected := RangeList{
		{11, 22},
		{95, 115},
		{998, 1012},
	}
	result := parseInput(input)
	assert.Equal(t, expected, result)
}
