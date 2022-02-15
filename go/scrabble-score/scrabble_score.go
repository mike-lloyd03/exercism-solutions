// Package scrabble contains tools for solving the advanced arithmetic of
// scrabble math
package scrabble

import "strings"

// Score returns the total word score for a single word
func Score(word string) int {
	score := 0
	for _, l := range strings.ToLower(word) {
		switch l {
		case 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't':
			score++
		case 'd', 'g':
			score += 2
		case 'b', 'c', 'm', 'p':
			score += 3
		case 'f', 'h', 'v', 'w', 'y':
			score += 4
		case 'k':
			score += 5
		case 'j', 'x':
			score += 8
		case 'q', 'z':
			score += 10
		}
	}
	return score
}
