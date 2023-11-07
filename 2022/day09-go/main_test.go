package main

import (
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const INPUT = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`

var EXAMPLE_START = NewRope(0, 4, 2)

func TestParse(t *testing.T) {
	dat := MustParse(INPUT)
	require.NotNil(t, dat)
	approvals.VerifyJSONStruct(t, dat)
}

func TestParseAndApply(t *testing.T) {
	dat := MustParse(INPUT)
	world := EXAMPLE_START
	var outputs []struct {
		Input string
		World Rope
	}
	outputs = append(outputs, struct {
		Input string
		World Rope
	}{"init", world})
	for _, step := range dat {
		world = MustApply(world, step)
		outputs = append(outputs, struct {
			Input string
			World Rope
		}{step.String(), world})
	}
	approvals.VerifyJSONStruct(t, outputs)
}

func TestFullExample(t *testing.T) {
	steps := MustParse(INPUT)
	world := EXAMPLE_START

	_, unique := MustApplyStepsAndRecord(world, steps)
	assert.Equal(t, 13, unique)
}

func TestMove_SameStart(t *testing.T) {
	world := Rope{Position{1, 1}, Position{1, 1}}
	out := MustApply(world, MustParseStep("R 1"))

	require.Equal(t, Position{2, 1}, out[0], "head should move")
	require.Equal(t, Position{1, 1}, out[1], "tail shouldn't move")
}

func TestMove_Corner(t *testing.T) {
	init := Rope{Position{1, 0}, Position{2, 0}}

	t.Run("D 1", func(t *testing.T) {
		step := MustParseStep("D 1")
		out := MustApply(init, step)
		require.Equal(t, Position{1, 1}, out[0], "head")
		require.Equal(t, Position{2, 0}, out[1], "tail must not move")
	})

	t.Run("D 2", func(t *testing.T) {
		out := MustApply(init, MustParseStep("D 2"))
		require.Equal(t, Position{1, 2}, out[0], "head")
		require.Equal(t, Position{1, 1}, out[1], "tail")
	})
}

func TestMove_ParallelUp(t *testing.T) {
	// .....    ..H..    ..H..
	// .TH.. -> .T... -> .T...
	// .....    .....    .....
	world := Rope([]Position{{X: 2, Y: 1}, {X: 1, Y: 1}})
	world = MustApply(world, MustParseStep("U 1"))
	require.Equal(t, Position{2, 0}, world[0], "head should move")
	require.Equal(t, Position{1, 1}, world[1], "tail shouldn't move")
}

func TestMove_CornerCorner(t *testing.T) {
	world := Rope{Position{2, 1}, Position{1, 1}}
	out := MustApply(world, MustParseStep("U 1"))

	require.Equal(t, Position{2, 0}, out[0], "head should move")
	require.Equal(t, Position{1, 1}, out[1], "tail shouldn't move")
}

// If the head is ever two steps directly up, down, left, or right from the tail,
// the tail must also move one step in that direction so it remains close enough:
func TestMove_Example1(t *testing.T) {
	t.Run("Horizontal", func(t *testing.T) {
		// .....    .....    .....
		// .TH.. -> .T.H. -> ..TH.
		// .....    .....    .....

		world := Rope([]Position{{X: 2, Y: 1}, {X: 1, Y: 1}})
		world = MustApply(world, MustParseStep("R 1"))
		approvals.VerifyJSONStruct(t, world)
	})

	t.Run("Vertical", func(t *testing.T) {
		// ...    ...    ...
		// .T.    .T.    ...
		// .H. -> ... -> .T.
		// ...    .H.    .H.
		// ...    ...    ...
		world := Rope([]Position{{X: 1, Y: 2}, {X: 1, Y: 1}})
		world = MustApply(world, MustParseStep("D 1"))
		approvals.VerifyJSONStruct(t, world)
	})
}

// Otherwise, if the head and tail aren't touching and aren't in the same row or column,
// the tail always moves one step diagonally to keep up:
func TestMove_Example2(t *testing.T) {
	t.Run("Vertical", func(t *testing.T) {
		// .....    .....    .....
		// .....    ..H..    ..H..
		// ..H.. -> ..... -> ..T..
		// .T...    .T...    .....
		// .....    .....    .....
		world := Rope([]Position{{X: 2, Y: 2}, {X: 1, Y: 3}})
		world = MustApply(world, MustParseStep("U 1"))
		approvals.VerifyJSONStruct(t, world)
	})

	t.Run("Horizontal", func(t *testing.T) {
		// .....    .....    .....
		// .....    .....    .....
		// ..H.. -> ...H. -> ..TH.
		// .T...    .T...    .....
		// .....    .....    .....
		world := Rope([]Position{{X: 2, Y: 2}, {X: 1, Y: 3}})

		world = MustApply(world, MustParseStep("R 1"))

		approvals.VerifyJSONStruct(t, world)
	})
}
