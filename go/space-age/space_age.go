package space

type Planet string

func Age(s float64, p Planet) float64 {
	ratio := ratios[p]
	return float64(s) / 3600 / 24 / 365.25 / ratio
}
