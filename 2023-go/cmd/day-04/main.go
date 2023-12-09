package main

import (
	"fmt"
	"strconv"
	"strings"
)

func MustParseCard(line string) Card {
	card, err := parseCard(line)
	if err != nil {
		panic(err)
	}
	return card
}

func parseCard(input string) (Card, error) {
	var card Card

	// Split the input into parts: "Card ID: WinningNumbers | CardNumbers"
	parts := strings.Split(input, ":")
	if len(parts) != 2 {
		return card, fmt.Errorf("invalid input format")
	}

	// Parse ID
	idPart := strings.TrimSpace(parts[0])
	fmt.Sscanf(idPart, "Card %d", &card.ID)

	// Parse WinningNumbers
	winNumStr := strings.Fields(parts[1][:strings.Index(parts[1], "|")])
	for _, numStr := range winNumStr {
		num, err := strconv.Atoi(numStr)
		if err != nil {
			return card, fmt.Errorf("error parsing winning number: %v", err)
		}
		card.WinningNumbers = append(card.WinningNumbers, num)
	}

	// Parse CardNumbers
	cardNumStr := strings.Fields(parts[1][strings.Index(parts[1], "|")+1:])
	for _, numStr := range cardNumStr {
		num, err := strconv.Atoi(numStr)
		if err != nil {
			return card, fmt.Errorf("error parsing card number: %v", err)
		}
		card.CardNumbers = append(card.CardNumbers, num)
	}

	return card, nil
}

type Card struct {
	ID             int
	WinningNumbers []int
	CardNumbers    []int
}

func (c Card) Points() int64 {
	var winningNumbers int
	for _, number := range c.CardNumbers {
		if c.IsWinningNumber(number) {
			if winningNumbers == 0 {
				winningNumbers = 1
			} else {
				winningNumbers <<= 1
			}
		}
	}
	return int64(winningNumbers)
}
func (c Card) IsWinningNumber(n int) bool {
	for _, winningNumber := range c.WinningNumbers {
		if n == winningNumber {
			return true
		}
	}
	return false
}

func MustParse(s []string) Input {
	var input Input
	for _, line := range s {
		input = append(input, MustParseCard(line))
	}
	return input
}

type Input []Card

func (i Input) TotalPoints() int64 {
	var sum int64
	for _, c := range i {
		sum += c.Points()
	}
	return sum
}
