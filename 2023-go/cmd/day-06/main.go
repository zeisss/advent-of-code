package main

import (
	"bufio"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

type Race struct {
	DistanceRecord int64
	Milliseconds   int64
}

func Parse(input string) ([]Race, error) {
	s := bufio.NewScanner(strings.NewReader(input))
	s.Split(bufio.ScanWords)

	var result []Race
	var distance bool
	var row int
	for s.Scan() {
		if s.Text() == "Time:" {
			row = 0
		} else if s.Text() == "Distance:" {
			distance = true
			row = 0
		} else {
			n := internal.MustAtoi64(s.Text())
			if distance {
				result[row].DistanceRecord = n
			} else {
				result = append(result, Race{Milliseconds: n})
			}
			row++
		}
	}
	return result, s.Err()
}

func Part1(r []Race) int {
	o := internal.Map(r, CalculateOptions)
	return internal.Mul(o)
}

func CalculateOptions(race Race) int {
	var beatingRecordOptions int
	for i := int64(0); i <= race.Milliseconds; i++ {
		r := CalcRange(i, race.Milliseconds)
		if r > race.DistanceRecord {
			// fmt.Printf("Pressing %d milliseconds gives %d > %d\n", i, r, race.DistanceRecord)
			beatingRecordOptions++
		}
	}

	return beatingRecordOptions
}

func CalcRange(buttonTime, raceTime int64) int64 {
	if buttonTime <= 0 {
		return 0
	}
	if buttonTime >= raceTime {
		return 0
	}
	return (raceTime - buttonTime) * buttonTime
}
