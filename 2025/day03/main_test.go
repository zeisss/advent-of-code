package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func ExamplePart1() {
	input := `987654321111111
811111111111119
234234234234278
818181911112111`
	result := Part1(input)
	fmt.Println(result)

	// Output: 357
}

func ExamplePart2() {
	input := `987654321111111
811111111111119
234234234234278
818181911112111`
	result := Part2(input)
	fmt.Println(result)
	// Output: 3121910778619
}

func TestBank_BiggestJoltage(t *testing.T) {
	var bank Bank
	var result int
	bank = ParseLine("1234")
	result = bank.BiggestJoltage()
	assert.Equal(t, 34, result)

	bank = ParseLine("987654321111111")
	result = bank.BiggestJoltage()
	assert.Equal(t, 98, result)

	bank = ParseLine("811111111111119")
	result = bank.BiggestJoltage()
	assert.Equal(t, 89, result)

	bank = ParseLine("234234234234278")
	result = bank.BiggestJoltage()
	assert.Equal(t, 78, result)

	bank = ParseLine("818181911112111")
	result = bank.BiggestJoltage()
	assert.Equal(t, 92, result)
}

func TestBank_BatteryCombiJoltage(t *testing.T) {
	const batteries = 12
	var bank Bank
	var result int64

	bank = ParseLine("1234321")
	result = bank.BatteryCombiJoltage(4)
	assert.EqualValues(t, 4321, result)

	bank = ParseLine("987654321111111")
	result = bank.BatteryCombiJoltage(batteries)
	assert.EqualValues(t, 987654321111, result)

	bank = ParseLine("811111111111119")
	result = bank.BatteryCombiJoltage(batteries)
	assert.EqualValues(t, 811111111119, result)

	bank = ParseLine("234234234234278")
	result = bank.BatteryCombiJoltage(batteries)
	assert.EqualValues(t, 434234234278, result)

	bank = ParseLine("818181911112111")
	result = bank.BatteryCombiJoltage(batteries)
	assert.EqualValues(t, 888911112111, result)
}

func TestParseInput(t *testing.T) {
	example := `987654321111111
811111111111119
234234234234278
818181911112111`
	puzzleInput := ParsePuzzleInput(example)

	expected := [][]Rating{
		{9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1},
		{8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9},
		{2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8},
		{8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1},
	}

	for i := range expected {
		for j := range expected[i] {
			if puzzleInput[i].Ratings[j] != expected[i][j] {
				t.Errorf("Expected puzzleInput[%d][%d] to be %d, got %d", i, j, expected[i][j], puzzleInput[i].Ratings[j])
			}
		}
	}
}
