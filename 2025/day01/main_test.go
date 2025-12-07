package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseRow(t *testing.T) {
	assert.Equal(t, Row{false, 1}, parseRow("R01"))
	assert.Equal(t, Row{true, 850}, parseRow("L850"))
}

func TestParseInput(t *testing.T) {
	input := `R01
L850
R123`
	rows := parseInput(input)
	expected := []Row{
		{false, 1},
		{true, 850},
		{false, 123},
	}
	assert.Equal(t, expected, rows)
}

func TestRunInput(t *testing.T) {
	t.Run("simple", func(t *testing.T) {
		input := []Row{
			{true, 25},   // 25
			{true, 30},   // 95
			{false, 105}, // 0
			{false, 100}, // 0
		}
		part1, part2, lastPosition := RunInput(50, input)
		assert.Equal(t, 2, part1, "zero hit")
		assert.Equal(t, 4, part2, "Part 2")
		assert.EqualValues(t, 0, lastPosition, "last position")
	})

	t.Run("1000x right", func(t *testing.T) {
		dial := Dial(50)
		_, part2, lastPosition := RunInput(dial, []Row{
			{false, 1000},
		})

		assert.Equal(t, 10, part2, "Part 2")
		assert.EqualValues(t, 50, lastPosition, "Dial end position")
	})
}

func ExampleRunInput_firstExample() {
	var input = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`
	rows := parseInput(input)
	part1, part2, lastPosition := RunInput(50, rows)
	fmt.Println("Part 1:", part1)
	fmt.Println("Part 2:", part2)
	fmt.Println("Last position:", lastPosition)
	// Output:
	// Part 1: 3
	// Part 2: 6
	// Last position: 32
}

func ExampleDial_Right() {
	dial := Dial(50)
	dial, n := dial.Right(30)
	fmt.Println("a", dial, n)
	dial, n = dial.Right(10)
	fmt.Println("b", dial, n)
	dial, n = dial.Right(20)
	fmt.Println("c", dial, n)

	// Output:
	// a 80 0
	// b 90 0
	// c 10 1
}

func ExampleDial_Right_two() {
	dial := Dial(95)
	dial, n := dial.Right(10)
	fmt.Println(dial, n)

	// Output: 5 1
}

func ExampleDial_Right_tozero() {
	fmt.Println(Dial(95).Right(5))

	// Output: 0 1
}

func TestDial_Left(t *testing.T) {
	t.Run("5 L10", func(t *testing.T) {
		dial, n := Dial(5).Left(10)
		assert.EqualValues(t, 95, dial)
		assert.Equal(t, 1, n)
	})
	t.Run("0 L5", func(t *testing.T) {
		dial, n := Dial(0).Left(5)
		assert.EqualValues(t, 95, dial)
		assert.Equal(t, 0, n)
	})
	t.Run("50 L10", func(t *testing.T) {
		dial, n := Dial(50).Left(10)
		assert.EqualValues(t, 40, dial)
		assert.Equal(t, 0, n)
	})
	t.Run("50 L250", func(t *testing.T) {
		dial, n := Dial(50).Left(250)
		assert.EqualValues(t, 0, dial)
		assert.Equal(t, 3, n)
	})
	t.Run("50 L100", func(t *testing.T) {
		dial, n := Dial(50).Left(100)
		assert.EqualValues(t, 50, dial)
		assert.Equal(t, 1, n)
	})
	t.Run("0 L200", func(t *testing.T) {
		dial, n := Dial(0).Left(200)
		assert.EqualValues(t, 0, dial)
		assert.Equal(t, 2, n)
	})
}
