package grains

import (
	"errors"
)

// Square returns the number of grains on a given square
func Square(input int) (uint64, error) {
	if input <= 0 || input > 64 {
		return 0, errors.New("input value must be in the range [1 - 64]")
	}
	return 1 << (input - 1), nil
}

// Total returns the total number of grains on the chessboard
func Total() uint64 {
	return 1<<64 - 1
}
