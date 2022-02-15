// Package proverb tells riveting tales of bravery, courage, and overcoming
// the challenges of life.
package proverb

import "fmt"

// Proverb will tell your kids a story that will magically make them go to
// sleep without getting up seven times to ask for water or to go pee.
func Proverb(rhyme []string) []string {
	if len(rhyme) == 0 {
		return []string{}
	}
	result := []string{}
	for i := 0; i < len(rhyme)-1; i++ {
		result = append(result, fmt.Sprintf("For want of a %s the %s was lost.", rhyme[i], rhyme[i+1]))
	}

	result = append(result, fmt.Sprintf("And all for the want of a %s.", rhyme[0]))
	return result
}
