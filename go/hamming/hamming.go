package hamming

import (
	"errors"
)

// Distance measures the hamming distance between two strings containing
// DNA strand information.
func Distance(a, b string) (int, error) {
	var aRunes = []rune(a)
	var bRunes = []rune(b)
	if len(aRunes) != len(bRunes) {
		return 0, errors.New("both input strings must be of the same length")
	}

	var count int = 0
	for i := 0; i < len(aRunes); i++ {
		if aRunes[i] != bRunes[i] {
			count++
		}
	}

	return count, nil
}
