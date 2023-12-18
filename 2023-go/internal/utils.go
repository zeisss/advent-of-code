package internal

import (
	"os"
	"strconv"
	"strings"
)

func MustReadFile(path string) []string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return strings.Split(string(data), "\n")
}

func MustReadFileNoSplit(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return string(data)
}

func MustAtoi(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return n
}

func MustAtoi64(s string) int64 {
	n, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		panic(err)
	}
	return n
}

func Must[X any](x X, err error) X {
	if err != nil {
		panic(err)
	}
	return x
}

func Map[N any, O any](values []N, mapper func(N) O) []O {
	var result []O = make([]O, len(values))
	for i := range values {
		result[i] = mapper(values[i])
	}
	return result
}

func MapIndex[N any, O any](values []N, mapper func(int, N) O) []O {
	var result []O = make([]O, len(values))
	for i := range values {
		result[i] = mapper(i, values[i])
	}
	return result
}

func Sum[N int64 | int32 | int | float32 | float64](values []N) N {
	var sum N
	for i := range values {
		sum += values[i]
	}
	return sum
}

func Mul[N int64 | int32 | int | float32 | float64](values []N) N {
	var mul N = 1
	for i := range values {
		mul *= values[i]
	}
	return mul
}
