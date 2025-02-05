package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var input = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`

func TestParseRules(t *testing.T) {
	rules := ParseRules(`47|53
97|13
97|61`)
	assert.Equal(t, Rules{
		47: []int64{53},
		97: []int64{13, 61},
	}, rules)
}
func TestParseUpdates(t *testing.T) {
	updates := ParseUpdates(`61,13,29
97,13,75,29,47`)
	assert.Equal(t, Updates{
		[]int64{61, 13, 29},
		[]int64{97, 13, 75, 29, 47},
	}, updates)
}

func TestMiddleElement(t *testing.T) {
	assert.EqualValues(t, 3, MiddlePageNumber([]int64{1, 2, 3, 4, 5}))
}

func TestIsCorrectlyOrdered(t *testing.T) {
	// empty rules allow everything
	assert.True(t, IsCorrectlyOrdered(Rules{}, []int64{1, 2, 3}))
	assert.True(t, IsCorrectlyOrdered(Rules{
		1: []int64{5},
	}, []int64{1, 2, 3, 5}))

	var rules Rules = ParseRules(`47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13`)
	assert.True(t, IsCorrectlyOrdered(rules, []int64{75, 47, 61, 53, 29}))
	assert.True(t, IsCorrectlyOrdered(rules, []int64{97, 61, 53, 29, 13}))
	assert.True(t, IsCorrectlyOrdered(rules, []int64{75, 29, 13}))

	// Negative examples from task
	assert.False(t, IsCorrectlyOrdered(rules, []int64{75, 97, 47, 61, 53}))
	assert.False(t, IsCorrectlyOrdered(rules, []int64{61, 13, 29}))
	assert.False(t, IsCorrectlyOrdered(rules, []int64{97, 13, 75, 29, 47}))
}

func TestFixUpdate(t *testing.T) {
	var rules Rules = ParseRules(`47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13`)
	assert.Equal(t, []int64{97, 75, 47, 61, 53}, FixUpdate(rules, []int64{75, 97, 47, 61, 53}))
	assert.Equal(t, []int64{61, 29, 13}, FixUpdate(rules, []int64{61, 13, 29}))
	assert.Equal(t, []int64{97, 75, 47, 29, 13}, FixUpdate(rules, []int64{97, 13, 75, 29, 47}))
}
