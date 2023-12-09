package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

var steps = []string{
	"seed",
	"soil",
	"fertilizer",
	"water",
	"light",
	"temperature",
	"humidity",
	"location",
}

type Almanac struct {
	Seeds []int

	Mappings map[string][]Mapping
}

type Mapping struct {
	DestinationStart int
	SourceStart      int
	Range            int
}

func MustParseAlmanac(lines []string) Almanac {
	var almanac Almanac
	almanac.Mappings = make(map[string][]Mapping, len(steps))

	for _, seed := range strings.Split(lines[0][7:], " ") {
		almanac.Seeds = append(almanac.Seeds, internal.MustAtoi(seed))
	}

	var currentMap string
	for _, line := range lines[2:] {
		if i := strings.LastIndex(line, " map:"); i >= 0 {
			currentMap = line[0:i]
		} else if strings.TrimSpace(line) == "" {
			// noop
		} else {
			if currentMap == "" {
				panic("currentMap empty")
			}
			var m Mapping
			fmt.Sscanf(line, "%d %d %d", &m.DestinationStart, &m.SourceStart, &m.Range)
			almanac.Mappings[currentMap] = append(almanac.Mappings[currentMap], m)
		}
	}

	return almanac
}

func (a Almanac) FindLowestSeedLocation() int {
	lowest := math.MaxInt32
	for _, seed := range a.Seeds {
		m := a.Resolve(seed)
		if m["location"] < lowest {
			lowest = m["location"]
		}
	}
	return lowest
}

func (a Almanac) Resolve(seed int) map[string]int {
	resolve := make(map[string]int)

	input := seed
	prevStep := "seed"
	for _, step := range steps {
		mapping := fmt.Sprintf("%s-to-%s", prevStep, step)

		n := resolveMappings(a.Mappings[mapping], input)
		resolve[step] = n

		prevStep = step
		input = n
	}

	return resolve
}

func resolveMappings(mappings []Mapping, input int) int {
	for _, m := range mappings {
		if n, ok := m.Resolve(input); ok {
			return n
		}
	}
	return input
}

func (m Mapping) Resolve(n int) (int, bool) {
	if n >= m.SourceStart && n <= m.SourceStart+m.Range {
		return m.DestinationStart + (n - m.SourceStart), true
	}
	return n, false
}
