package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile("./testdata/input.txt")
	if err != nil {
		log.Fatalln(err)
	}
	steps := MustParse(string(data))
	var world Rope // we can just start on the zero value

	_, unique := MustApplyStepsAndRecord(world, steps)

	fmt.Printf("Unique Places visited by Tail: %d", unique)

}

// NewRope returns a new rope of the given length with all pieces at position x,y.
func NewRope(x, y, length int) Rope {
	var p []Position

	for i := 0; i < length; i++ {
		p = append(p, Position{x, y})
	}
	return Rope(p)
}

type Position struct{ X, Y int }

func (p Position) String() string {
	return fmt.Sprintf("<%d,%d>", p.X, p.Y)
}

type Rope [2]Position

func (r Rope) String() string {
	var b strings.Builder
	b.WriteString("head:")
	for i, k := range r {
		if i > 0 {
			b.WriteString(" <- ")
		}
		b.WriteString(k.String())
	}
	return b.String()
}

type Step struct {
	Direction string
	Amount    int
}

func (s Step) String() string {
	return fmt.Sprintf("%s %d", s.Direction, s.Amount)
}

func MustParseStep(line string) Step {
	fields := strings.SplitN(line, " ", 2)

	n, err := strconv.ParseInt(fields[1], 10, 32)
	if err != nil {
		panic(err)
	}

	return Step{
		Direction: fields[0],
		Amount:    int(n),
	}
}

func MustParse(input string) []Step {
	lines := strings.Split(input, "\n")
	var output []Step

	for _, line := range lines {
		step := MustParseStep(line)

		output = append(output, step)
	}

	return output
}

func MustMoveDirection(rope Rope, direction string) Rope {
	adjust := func(head int, tail *int) {
		if head > *tail {
			*tail++
		} else if head < *tail {
			*tail--
		}
	}
	move := func(primaryHead, secondaryHead, primaryTail, secondaryTail *int, mod int) {
		*primaryHead += mod

		if distance(*primaryHead, *primaryTail) > 1 {
			*primaryTail += mod
			adjust(*secondaryHead, secondaryTail)
		}
	}

	for i := range rope {
		if i == 0 {
			continue
		}

		head := rope[i-1]
		tail := rope[i]
		switch direction {
		case "U":
			move(&head.Y, &head.X, &tail.Y, &tail.X, -1)
		case "D":
			move(&head.Y, &head.X, &tail.Y, &tail.X, +1)
		case "L":
			move(&head.X, &head.Y, &tail.X, &tail.Y, -1)
		case "R":
			move(&head.X, &head.Y, &tail.X, &tail.Y, +1)
		default:
			panic("Unknown direction: " + direction)
		}
		rope[i-1] = head
		rope[i] = tail
	}
	return rope
}

func MustApply(rope Rope, step Step) Rope {
	for i := 0; i < step.Amount; i++ {
		rope = MustMoveDirection(rope, step.Direction)
	}
	return rope
}

func MustApplyStepsAndRecord(rope Rope, steps []Step) (Rope, int) {
	visited := make(map[Position]struct{})

	for _, step := range steps {
		for i := 0; i < step.Amount; i++ {
			rope = MustMoveDirection(rope, step.Direction)

			visited[rope[len(rope)-1]] = struct{}{}
		}
	}

	return rope, len(visited)
}

func distance(a, b int) int {
	d := a - b
	if d < 0 {
		d = d * -1
	}
	return d
}
