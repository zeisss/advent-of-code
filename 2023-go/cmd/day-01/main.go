package main

import (
	"strings"
	"unicode"
)

func Reduce(s []string, mapper func(string) (int, int)) int {
	var sum int
	for _, line := range s {
		a, b := mapper(line)
		sum += a*10 + b
	}
	return sum
}

func DigitCandidates(line string) (int, int) {
	firstDigitIndex := strings.IndexFunc(line, unicode.IsDigit)
	lastDigitIndex := strings.LastIndexFunc(line, unicode.IsDigit)

	a := int(line[firstDigitIndex] - '0')
	b := int(line[lastDigitIndex] - '0')

	return a, b
}

var words = map[string]int{
	"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9,
	"1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9,
}

func WordAndDigitCandidates(line string) (int, int) {
	var findex, fvalue, lindex, lvalue int
	findex = 9999
	lindex = -1

	for word, value := range words {
		fwordIndex := strings.Index(line, word)
		if fwordIndex >= 0 && fwordIndex < findex {
			findex = fwordIndex
			fvalue = value
		}
		lwordIndex := strings.LastIndex(line, word)
		if lwordIndex >= 0 && lwordIndex > lindex {
			lindex = lwordIndex
			lvalue = value
		}
	}

	return fvalue, lvalue
}
