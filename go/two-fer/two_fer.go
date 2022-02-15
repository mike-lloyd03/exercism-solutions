package twofer

import "fmt"

// ShareWith takes a string and returns the sentence
// "One for {string}, one for me." as a string.
func ShareWith(name string) string {
	var person string
	if name != "" {
		person = name
	} else {
		person = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", person)
}
