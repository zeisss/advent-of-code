package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile("./testdata/input.txt")
	if err != nil {
		panic(err)
	}

	var crt CRT
	program := MustParseProgram(string(data))
	cpu := NewCPU()
	Execute(&cpu, &crt, program)
	fmt.Println(cpu)
	crt.Render(os.Stdout)
}

func MustParseProgram(input string) []Op {
	r := bufio.NewScanner(strings.NewReader(input))
	r.Split(bufio.ScanWords)

	var ops []Op
	for r.Scan() {
		switch r.Text() {
		case "noop":
			ops = append(ops, Op{Code: NoopOpCode})
		case "addx":
			if !r.Scan() {
				panic("Expected number after 'addx' token")
			}
			n, err := strconv.ParseInt(r.Text(), 10, 32)
			if err != nil {
				panic(err)
			}

			ops = append(ops, Op{Code: AddXOpCode, Arg1: int(n)})
		default:
			panic("Unknown token: " + r.Text())
		}
	}
	return ops
}

type OpCode int

const (
	NoopOpCode OpCode = iota
	AddXOpCode
)

var cycleCost = map[OpCode]int{
	NoopOpCode: 1,
	AddXOpCode: 2,
}

type Op struct {
	Code OpCode
	Arg1 int
}

type CPU struct {
	RegisterX         int
	Cycles            int
	SignalStrengthSum int
}

func NewCPU() CPU {
	return CPU{RegisterX: 1, Cycles: 0, SignalStrengthSum: 0}
}

func (cpu CPU) String() string {
	return fmt.Sprintf("CPU: RegisterX=%d Cycles=%d Signal=%d\n",
		cpu.RegisterX, cpu.Cycles, cpu.SignalStrengthSum)
}

type CRT struct {
	Pixel [40 * 6]byte
	X, Y  int
}

func (crt *CRT) Draw(registerX int) {
	// log.Printf("pixel %d -> Draw(%d)\n", crt.PixelToDraw, registerX)
	var b byte = '.'

	if registerX == crt.X || registerX-1 == crt.X || registerX+1 == crt.X {
		b = '#'
	}

	crt.Pixel[crt.X+crt.Y*40] = b
	crt.X++
	if crt.X == 40 {
		crt.X = 0
		crt.Y++
		if crt.Y == 6 {
			crt.Y = 0
		}
	}
}

func (crt CRT) Render(w io.Writer) {
	buf := string(crt.Pixel[:])
	for i := 0; i < 6; i++ {
		fmt.Fprintf(w, "%02d %s %02d\n", i, buf[i*40:(i+1)*40], i)
	}
}

func Execute(cpu *CPU, crt *CRT, ops []Op) {
	for _, op := range ops {
		for cost := cycleCost[op.Code]; cost > 0; cost-- {
			cpu.Cycles += 1
			crt.Draw(cpu.RegisterX)
			if (cpu.Cycles-20)%40 == 0 {
				cpu.SignalStrengthSum += cpu.Cycles * cpu.RegisterX
			}
		}

		switch op.Code {
		case NoopOpCode: // noop
		case AddXOpCode:
			cpu.RegisterX += op.Arg1
		default:
			panic("Unknown op code")
		}
	}
}
