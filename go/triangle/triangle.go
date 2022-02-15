// Package triangle contains tools for checking the validity of given triangle sides
package triangle

import (
	"math"
)

// Kind is an enum returning one of the 4 kinds of triangles
type Kind int

// These are the four kinds of triangles
const (
	NaT Kind = iota // not a triangle
	Equ             // equilateral
	Iso             // isosceles
	Sca             // scalene
)

// KindFromSides returns the kind of triangle that would be given by the three input sides
func KindFromSides(a, b, c float64) Kind {
	if !isValidNumber(a) || !isValidNumber(b) || !isValidNumber(c) {
		return NaT
	}

	if a > b+c || b > a+c || c > a+b {
		return NaT
	}

	if a == b && b == c {
		return Equ
	}

	if a == b || b == c || a == c {
		return Iso
	}
	return Sca
}

// isValidNumber returns true when a given value is a number, positive, and not infinite
func isValidNumber(num float64) bool {
	return !math.IsNaN(num) && !math.IsInf(num, 0) && num > 0
}
