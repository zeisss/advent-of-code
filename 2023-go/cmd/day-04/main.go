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

func (c Card) CountWins() int {
	var wins int
	for _, number := range c.CardNumbers {
		if c.IsWinningNumber(number) {
			wins++
		}
	}
	return wins
}

func (c Card) Points() int64 {
	wins := c.CountWins()
	if wins == 0 {
		return 0
	}
	n := 1
	for i := 1; i < wins; i++ {
		n <<= 1
	}
	return int64(n)
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

func (input Input) TotalPoints() int64 {
	var sum int64
	for _, c := range input {
		sum += c.Points()
	}
	return sum
}

func (input Input) CountCummulativeScratchCards() int64 {
	var copies []int = make([]int, len(input))
	for i := range copies { // Assume we have 1 scratch card each ID
		copies[i] = 1
	}

	var count int64

	for _, card := range input {
		// fmt.Println("=>", card.ID, count)
		if wins := card.CountWins(); wins > 0 {
			for i := 0; i < wins; i++ {
				copies[card.ID-1+i+1] += copies[card.ID-1]
			}
		}
		// fmt.Println("  Adding", int64(copies[card.ID-1]))
		count += int64(copies[card.ID-1])

		// fmt.Printf("  %v\n", copies)
	}

	return count
}
