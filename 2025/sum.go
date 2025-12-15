package helpers

import (
	"iter"
	"math"

	"golang.org/x/exp/constraints"
)

func Sum[T constraints.Integer](it iter.Seq[T]) T {
	var agg T
	for val := range it {
		agg += val
	}
	return agg
}

func Filter[T any](it iter.Seq[T], filter func(T) bool) iter.Seq[T] {
	return func(yield func(T) bool) {
		for v := range it {
			if filter(v) {
				if !yield(v) {
					return
				}
			}
		}
	}
}

func Map[T any, O any](it iter.Seq[T], mapper func(T) O) iter.Seq[O] {
	return func(yield func(O) bool) {
		for v := range it {
			o := mapper(v)
			if !yield(o) {
				return
			}
		}
	}
}

func PowInt64(base int64, exp int) int64 {
	return int64(math.Pow(float64(base), float64(exp)))
}
