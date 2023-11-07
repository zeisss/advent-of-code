package main

import (
	"fmt"
	"strings"
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const INPUT_SMALL = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`

const INPUT_LARGE = `R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`

var START_SMALL = NewRope(0, 4, 2)

func TestParse(t *testing.T) {
	dat := MustParse(INPUT_SMALL)
	require.NotNil(t, dat)
	approvals.VerifyJSONStruct(t, dat)
}

func TestExampleSteps(t *testing.T) {
	n := func(t testing.TB, input string, start Rope, w, h int) {
		var buf strings.Builder
		visitor := Renderer{
			out:    &buf,
			w:      w,
			h:      h,
			startX: start[0].X,
			startY: start[0].Y,
		}
		dat := MustParse(input)
		MustApplyStepsAndVisit(start, dat, visitor)
		approvals.VerifyString(t,
			buf.String(),
		)
	}

	t.Run("small", func(t *testing.T) {
		n(t, INPUT_SMALL, START_SMALL, 6, 5)
	})
	t.Run("part2_small", func(t *testing.T) {
		n(t, `R 4
		U 4
		L 3
		D 1
		R 4
		D 1
		L 5
		R 2`, NewRope(0, 4, 10), 6, 5)
	})
	t.Run("part2_large", func(t *testing.T) {
		n(t, `R 5
		U 8
		L 8
		D 3
		R 17
		D 10
		L 25
		U 20`, NewRope(11, 15, 10), 26, 21)
	})
}

func TestExamplSteps_UniquePlaces(t *testing.T) {
	n := func(t testing.TB, rope Rope, input string, expectedPositions int) {
		visitor := SeenFieldsRecorder{}
		steps := MustParse(input)
		MustApplyStepsAndVisit(rope, steps, visitor)
		assert.Equal(t, expectedPositions, visitor.Unique())
	}
	t.Run("small", func(t *testing.T) {
		n(t, START_SMALL, INPUT_SMALL, 13)
	})
	t.Run("part2_large", func(t *testing.T) {
		n(t, NewRope(0, 0, 10), INPUT_LARGE, 36)
	})

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
		require.Equal(t, Position{3, 1}, world[0], "head")
		require.Equal(t, Position{2, 1}, world[1], "tail")
	})

	t.Run("Vertical", func(t *testing.T) {
		// ...    ...    ...
		// .T.    .T.    ...
		// .H. -> ... -> .T.
		// ...    .H.    .H.
		// ...    ...    ...
		world := Rope([]Position{{X: 1, Y: 2}, {X: 1, Y: 1}})
		world = MustApply(world, MustParseStep("D 1"))
		require.Equal(t, Position{1, 3}, world[0], "head")
		require.Equal(t, Position{1, 2}, world[1], "tail")
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
		require.Equal(t, Position{2, 1}, world[0], "head")
		require.Equal(t, Position{2, 2}, world[1], "tail")
	})

	t.Run("Horizontal", func(t *testing.T) {
		// .....    .....    .....
		// .....    .....    .....
		// ..H.. -> ...H. -> ..TH.
		// .T...    .T...    .....
		// .....    .....    .....
		world := Rope([]Position{{X: 2, Y: 2}, {X: 1, Y: 3}})
		world = MustApply(world, MustParseStep("R 1"))
		require.Equal(t, Position{3, 2}, world[0], "head")
		require.Equal(t, Position{2, 2}, world[1], "tail")
	})
}

func TestRenderRopeWorld(t *testing.T) {
	var buf strings.Builder
	RenderString(&buf, 0, 0, 6, 5, START_SMALL)

	fmt.Fprintln(&buf)
	RenderString(&buf, 0, 0, 6, 5, Rope([]Position{
		{0, 0}, {1, 1}, {2, 2},
	}))

	approvals.VerifyString(t, buf.String())
}
