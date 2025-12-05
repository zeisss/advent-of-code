package main

func Sum(i []int) int {
	var count int
	for _, n := range i {
		count += n
	}
	return count
}

func StringToIntSlice(s string) []int {
	var result []int
	for _, c := range s {
		result = append(result, RuneToInt(c))
	}
	return result
}

func RuneToInt(r rune) int {
	switch r {
	case '0':
		return 0
	case '1':
		return 1
	case '2':
		return 2
	case '3':
		return 3
	case '4':
		return 4
	case '5':
		return 5
	case '6':
		return 6
	case '7':
		return 7
	case '8':
		return 8
	case '9':
		return 9
	default:
		panic("Invalid rune")
	}
}
