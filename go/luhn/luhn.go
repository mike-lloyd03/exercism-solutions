package luhn

import (
	"strconv"
	"strings"
)

func Valid(input string) bool {
	check_sum := 0
	inputRunes := []rune(strings.ReplaceAll(input, " ", ""))
	isSecondFromRight := false

	if len(inputRunes) <= 1 {
		return false
	}

	for i := len(inputRunes) - 1; i >= 0; i-- {
		currChar := string(inputRunes[i])
		currInt, err := strconv.Atoi(currChar)

		if err != nil {
			return false
		}

		if isSecondFromRight {
			double := currInt * 2
			if double > 9 {
				double -= 9
			}
			check_sum += double
		} else {
			check_sum += currInt
		}
		isSecondFromRight = !isSecondFromRight
	}

	return check_sum%10 == 0
}
