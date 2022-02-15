package lasagna

const OvenTime int = 40 //minutes

func RemainingOvenTime(timeInOven int) int {
	return OvenTime - timeInOven
}

func PreparationTime(numLayers int) int {
	return numLayers * 2
}

func ElapsedTime(numLayers, timeInOven int) int {
	return PreparationTime(numLayers) + timeInOven
}
