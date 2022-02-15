package isogram

import "strings"

// IsIsogram takes in a string and returns a bool whether the string is an
// isogram (a word or phrase without a letter being used more than one time).
func IsIsogram(s string) bool {
	runeMap := make(map[rune]bool)

	for _, r := range strings.ToLower(s) {
		if r == ' ' || r == '-' {
			continue
		}
		if runeMap[r] {
			return false
		}
		runeMap[r] = true
	}

	return true
}
