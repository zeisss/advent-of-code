package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/require"
)

const INPUT = `noop
addx 3
addx -5`

var LARGE_EXAMPLE_INPUT = `addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
	addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop`

func TestParseProgram(t *testing.T) {
	program := MustParseProgram(INPUT)
	require.NotNil(t, program)

	program = MustParseProgram(LARGE_EXAMPLE_INPUT)
	require.NotNil(t, program)
}

func ExampleINPUT() {
	program := MustParseProgram(INPUT)
	cpu := NewCPU()
	var crt CRT

	Execute(&cpu, &crt, program)

	fmt.Println(cpu.String())

	// Output:
	// CPU: RegisterX=-1 Cycles=5 Signal=0
}

func ExampleCPU_LARGE_EXAMPLE_INPUT() {
	program := MustParseProgram(LARGE_EXAMPLE_INPUT)
	cpu := NewCPU()
	var crt CRT

	Execute(&cpu, &crt, program)

	fmt.Println(cpu.String())

	// Output:
	// CPU: RegisterX=17 Cycles=240 Signal=13140
}

func ExampleCRT_LARGE_EXAMPLE_INPUT() {
	program := MustParseProgram(LARGE_EXAMPLE_INPUT)
	cpu := NewCPU()
	var crt CRT

	Execute(&cpu, &crt, program)
	crt.Render(os.Stdout)

	// Output:
	// 00 ##..##..##..##..##..##..##..##..##..##.. 00
	// 01 ###...###...###...###...###...###...###. 01
	// 02 ####....####....####....####....####.... 02
	// 03 #####.....#####.....#####.....#####..... 03
	// 04 ######......######......######......#### 04
	// 05 #######.......#######.......#######..... 05
}
