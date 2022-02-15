// Package raindrops turns numbers into the magical sounds of rainfall
package raindrops

import "fmt"

// Convert is where the magic happens.
func Convert(number int) string {
	result := ""

	if number%3 == 0 {
		result += "Pling"
	}
	if number%5 == 0 {
		result += "Plang"
	}
	if number%7 == 0 {
		result += "Plong"
	}

	if result == "" {
		return fmt.Sprint(number)
	}
	return result
}
