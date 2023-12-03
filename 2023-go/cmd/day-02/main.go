package main

import (
	"fmt"
	"strings"
)

type RGB struct{ r, g, b int }
type Game struct {
	ID  int
	RGB []RGB
}

func SumGameIDsWithCondition(input []string, checker func(Game) int) int {
	var sum int
	for _, line := range input {
		game := ParseLine(line)
		n := checker(game)
		sum += n
	}
	return sum
}

func Part1(g Game) int {
	for _, rgb := range g.RGB {
		if rgb.r > 12 || rgb.g > 13 || rgb.b > 14 {
			return 0
		}
	}
	return g.ID
}

func Part2(g Game) int {
	var fewest RGB = g.RGB[0]
	for _, rgb := range g.RGB {
		if fewest.r < rgb.r {
			fewest.r = rgb.r
		}
		if fewest.g < rgb.g {
			fewest.g = rgb.g
		}
		if fewest.b < rgb.b {
			fewest.b = rgb.b
		}
	}
	return fewest.r * fewest.g * fewest.b
}

func ParseLine(line string) Game {
	var game Game

	gameAndColors := strings.SplitN(line, ":", 2)
	fmt.Sscanf(gameAndColors[0], "Game %d:", &game.ID)

	clues := strings.Split(gameAndColors[1], ";")
	for _, clue := range clues {
		rgb := parseColorClue(strings.TrimSpace(clue))
		game.RGB = append(game.RGB, rgb)
	}
	return game
}

func parseColorClue(clue string) RGB {
	var rgb RGB
	colors := strings.Split(clue, ",") // <n> <color>
	for _, assignment := range colors {
		var color string
		var amount int
		fmt.Sscanf(strings.TrimSpace(assignment), "%d %s", &amount, &color)

		switch color {
		case "red":
			rgb.r = amount
		case "blue":
			rgb.b = amount
		case "green":
			rgb.g = amount
		default:
			panic("unknown color: '" + color + "' input: '" + assignment + "'")
		}
	}

	return rgb
}
