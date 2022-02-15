package reverse

// Reverse returns the reverse of the input string.
func Reverse(input string) string {
	stringRunes := []rune(input)
	for i, j := 0, len(stringRunes)-1; i < j; i, j = i+1, j-1 {
		stringRunes[i], stringRunes[j] = stringRunes[j], stringRunes[i]
	}
	return string(stringRunes)
}
