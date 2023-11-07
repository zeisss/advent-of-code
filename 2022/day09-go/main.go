package main

import (
	"fmt"
	"io"
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
	rope := NewRope(0, 0, 10)
	visitor := SeenFieldsRecorder{}

	MustApplyStepsAndVisit(rope, steps, visitor)

	fmt.Printf("Unique Places visited by Tail: %d", visitor.Unique())
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

type Rope []Position

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

func (r Rope) Copy() Rope {
	positions := make([]Position, 0, len(r))
	for _, pos := range r {
		positions = append(positions, pos)
	}
	return Rope(positions)
}

type Step struct {
	Direction string
	Amount    int
}

func (s Step) String() string {
	return fmt.Sprintf("%s %d", s.Direction, s.Amount)
}

func MustParseStep(line string) Step {
	fields := strings.SplitN(strings.TrimSpace(line), " ", 2)

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
	move := func(primaryHead *int, mod int) {
		*primaryHead += mod
	}

	snapAlong := func(head, tail *Position) {
		if *head == *tail {
			return
		}

		if head.X == tail.X {
			if head.Y-tail.Y > 1 {
				tail.Y++
			} else if tail.Y-head.Y > 1 {
				tail.Y--
			}
		} else if head.Y == tail.Y {
			if head.X-tail.X > 1 {
				tail.X++
			} else if tail.X-head.X > 1 {
				tail.X--
			}
		} else if distance(head.X, tail.X) >= 2 || distance(head.Y, tail.Y) >= 2 {
			// head is in one of the corners, we need to adjust both X & Y
			if head.Y-tail.Y > 0 {
				tail.Y++
			} else if tail.Y-head.Y > 0 {
				tail.Y--
			}
			if head.X-tail.X > 0 {
				tail.X++
			} else if tail.X-head.X > 0 {
				tail.X--
			}
		}

	}

	rope = rope.Copy()

	switch direction {
	case "U":
		move(&rope[0].Y, -1)
	case "D":
		move(&rope[0].Y, +1)
	case "L":
		move(&rope[0].X, -1)
	case "R":
		move(&rope[0].X, +1)
	default:
		panic("Unknown direction: " + direction)
	}

	for i := range rope {
		if i == 0 {
			continue
		}
		tail := rope[i]
		snapAlong(&rope[i-1], &tail)
		rope[i] = tail
	}
	return rope
}

func distance(a, b int) int {
	d := a - b
	if d < 0 {
		return -1 * d
	} else {
		return d
	}
}

func MustApply(rope Rope, step Step) Rope {
	for i := 0; i < step.Amount; i++ {
		rope = MustMoveDirection(rope, step.Direction)
	}
	return rope
}

type Visitor interface {
	BeforeStep(string)
	Seen(Rope)
}

func MustApplyStepsAndVisit(rope Rope, steps []Step, visitor Visitor) Rope {
	if visitor != nil {
		visitor.BeforeStep("Initial State")
		visitor.Seen(rope)
	}
	for _, step := range steps {
		visitor.BeforeStep(step.String())
		for i := 0; i < step.Amount; i++ {
			rope = MustMoveDirection(rope, step.Direction)
			if visitor != nil {
				visitor.Seen(rope)
			}
		}
	}

	return rope
}

func RenderString(out io.Writer, startX, startY, width, height int, rope Rope) {
	// Assume: rope is never <0 or > width|height

	fmt.Println(rope)

	row := strings.Repeat(".", width) + "\n"
	field := strings.Repeat(row, height)

	data := []byte(field)

	if startX >= 0 && startY >= 0 {
		data[startX+startY*(width+1)] = 's'
	}

	// we draw the last rope element first
	for i := len(rope) - 1; i >= 0; i-- {
		p := rope[i]

		var w byte
		switch i {
		case 0:
			w = 'H'
		default:
			if len(rope) == 2 {
				w = 'T'
			} else {
				w = byte('0' + i)
			}
		}
		data[p.X+p.Y*(width+1)] = w
	}

	out.Write(data)
}

type SeenFieldsRecorder map[Position]struct{}

func (v SeenFieldsRecorder) Seen(rope Rope) {
	tail := rope[len(rope)-1]
	v[tail] = struct{}{}
}
func (v SeenFieldsRecorder) BeforeStep(s string) {}

func (v SeenFieldsRecorder) Unique() int {
	return len(v)
}

type Renderer struct {
	w, h           int
	startX, startY int
	out            io.Writer
}

func (r Renderer) BeforeStep(s string) {
	fmt.Fprintf(r.out, "== %s ==\n\n", s)
}

func (r Renderer) Seen(rope Rope) {
	RenderString(r.out, r.startX, r.startY, r.w, r.h, rope)
	fmt.Fprintln(r.out)
}
