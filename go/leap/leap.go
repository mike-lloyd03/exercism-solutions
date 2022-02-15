// Package leap calculates whether the given year is a leap year or not.
package leap

// IsLeapYear does what it sounds like it does.
func IsLeapYear(year int) bool {
	return ((year%4 == 0) && !(year%100 == 0)) || (year%400 == 0)
}
