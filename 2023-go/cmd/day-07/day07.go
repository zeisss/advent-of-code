package day07

import (
	"sort"
	"strings"

	"github.com/zeisss/advent-of-code/2023-go/internal"
)

type Hand struct {
	Labels string
	Bid    int64
}

func Parse(input string) ([]Hand, error) {
	var hands []Hand
	for _, line := range strings.Split(input, "\n") {
		fields := strings.Fields(line)
		hands = append(hands, Hand{
			Labels: fields[0],
			Bid:    internal.MustAtoi64(fields[1]),
		})
	}
	return hands, nil
}

func Part1(hands []Hand) int64 {
	sort.Sort(SortByType(hands))
	scores := internal.MapIndex(hands, func(i int, h Hand) int64 {
		return (int64(i) + 1) * h.Bid
	})
	return internal.Sum(scores)
}

type Kind int

const (
	FiveOfAKind  Kind = 6
	FourOfAkind  Kind = 5
	FullHouse    Kind = 4
	ThreeOfAKind Kind = 3
	TwoPair      Kind = 2
	OnePair      Kind = 1
	HighCard     Kind = 0
)

func (k Kind) String() string {
	switch k {
	case FiveOfAKind:
		return "five-of-a-kind"
	case FourOfAkind:
		return "four-of-a-kind"
	case FullHouse:
		return "full-house"
	case ThreeOfAKind:
		return "three-of-a-kind"
	case TwoPair:
		return "two-pair"
	case OnePair:
		return "one-pair"
	case HighCard:
		return "high-card"
	default:
		panic("unknown kind")
	}
}

// Type returns the type of card from (6 = Five of a kind to 0 = High cards)
func (h Hand) Type() Kind {
	c := AnalyzeHand(h.Labels)
	var triplets int
	var pairs int
	var singles int
	for i := 0; i < len(c); i++ {
		switch c[i] {
		case 5:
			// we cannot have multiple of these, just return directly
			return FiveOfAKind
		case 4:
			// we cannot have multiple of these, just return directly
			return FourOfAkind
		case 3:
			triplets++
		case 2:
			pairs++
		case 1:
			singles++
		}
	}

	if triplets == 1 {
		if pairs == 1 {
			return FullHouse
		}
		return ThreeOfAKind
	} else if pairs == 2 {
		return TwoPair
	} else if pairs == 1 {
		return OnePair
	} else {
		return HighCard
	}
}

type SortByType []Hand

func (a SortByType) Len() int      { return len(a) }
func (a SortByType) Swap(i, j int) { a[i], a[j] = a[j], a[i] }
func (a SortByType) Less(i, j int) bool {
	if a[i].Type() == a[j].Type() {
		for p := 0; p < 5; p++ {
			if labelToWorth(rune(a[i].Labels[p])) < labelToWorth(rune(a[j].Labels[p])) {
				return true
			}
			if labelToWorth(rune(a[i].Labels[p])) > labelToWorth(rune(a[j].Labels[p])) {
				return false
			}
		}
		return false
	}
	return a[i].Type() < a[j].Type()
}

func AnalyzeHand(hand string) [13]uint {
	var count [13]uint

	for _, s := range hand {
		count[labelToWorth(s)]++
	}

	return count
}

func labelToWorth(r rune) int {
	switch r {
	case 'A':
		return 12
	case 'K':
		return 11
	case 'Q':
		return 10
	case 'J':
		return 9
	case 'T':
		return 8
	case '9':
		return 7
	case '8':
		return 6
	case '7':
		return 5
	case '6':
		return 4
	case '5':
		return 3
	case '4':
		return 2
	case '3':
		return 1
	case '2':
		return 0
	default:
		panic("Unknown label")
	}
}
