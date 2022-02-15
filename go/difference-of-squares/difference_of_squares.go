package diffsquares

// SquareOfSum returns the square of the sum of all the integers between zero
// and the argument (inclusive).
func SquareOfSum(x int) int {
	sum := x * (x + 1) / 2

	return sum * sum

}

// SumOfSquares returns the sum of the squares of all the integers between zero
// and the argument (inclusive).
func SumOfSquares(x int) int {
	return (x * (x + 1) * (2*x + 1)) / 6
}

// Difference returns the difference between the SquareOfSum and the SumOfSquares
func Difference(x int) int {
	return SquareOfSum(x) - SumOfSquares(x)
}
