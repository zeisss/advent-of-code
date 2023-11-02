package main

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	plan, err := ParseFile("./testdata/input.txt")
	if err != nil {
		log.Fatalf("ERROR: %v", err)
	}
	for plan.HasNextAction() {
		StackMover9001.processNextAction(&plan)
	}
	log.Println(GetTopContainerNames(plan.Stacks))
}

type Plan struct {
	Stacks  [][]string
	Actions []Action
}
type Action struct {
	Src, Dst int
	Amount   int
}

func (p Plan) HasNextAction() bool {
	return len(p.Actions) > 0
}

func ParseFile(filename string) (Plan, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return Plan{}, err
	}
	return ParseData(data)
}

func ParseData(data []byte) (Plan, error) {
	var plan Plan
	scanner := bufio.NewScanner(bytes.NewReader(data))

	for scanner.Scan() {
		line := scanner.Text()

		if strings.HasPrefix(line, " 1   2") {
			continue
		}

		sliceCount := (len(line) + 1) / 4
		// log.Printf("stacks: %d", sliceCount)

		// grow stacks as needed
		for i := len(plan.Stacks); i < sliceCount; i++ {
			plan.Stacks = append(plan.Stacks, make([]string, 0, 10))
		}

		// parse line and preprend to stack
		for i := 0; i < sliceCount; i++ {
			r := line[i*4+1 : i*4+2]
			if r != " " {
				// log.Printf("Stack: %d Container: %v", i, r)
				plan.Stacks[i] = append([]string{r}, plan.Stacks[i]...)
			}
		}

		// log.Println("line: ", line)
		if line == "" {
			break // End of stack drawing
		}
	}

	for scanner.Scan() {
		line := scanner.Text() // format: "move X from S to D"
		// log.Println("line: ", line)

		var action Action
		_, err := fmt.Sscanf(line, "move %d from %d to %d", &action.Amount, &action.Src, &action.Dst)
		if err != nil {
			return Plan{}, err
		}
		plan.Actions = append(plan.Actions, action)
	}
	return plan, nil
}

type StackMover bool

var (
	StackMover9000 StackMover = false
	StackMover9001 StackMover = true
)

func (mover StackMover) processNextAction(p *Plan) {
	if !p.HasNextAction() {
		panic("no actions left")
	}

	action := p.Actions[0]
	p.Actions = p.Actions[1:]
	p.Stacks = mover.applyAction(p.Stacks, action)
}

func (mover StackMover) applyAction(stacks [][]string, action Action) [][]string {
	src := action.Src - 1
	dst := action.Dst - 1

	if src > len(stacks) || dst > len(stacks) {
		panic("invalid action src / dst")
	}
	if action.Amount > len(stacks[src]) {
		panic("not enough containers on src stack")
	}

	srcLen := len(stacks[src])

	data := stacks[src][srcLen-action.Amount:]
	stacks[src] = stacks[src][0 : srcLen-action.Amount]

	if mover {
		stacks[dst] = append(stacks[dst], data...)
	} else {
		for i := len(data); i > 0; i-- {
			stacks[dst] = append(stacks[dst], data[i-1])
		}
	}

	return stacks
}

func GetTopContainerNames(stacks [][]string) string {
	out := ""

	for i := 0; i < len(stacks); i++ {
		out += stacks[i][len(stacks[i])-1]
	}

	return out
}
