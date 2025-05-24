package metrics

type Statistics struct {
	TotalCount    int
	TotalDuration float64
	OpsPerSec     float64
	Min           float64
	P50           float64
	P75           float64
	P95           float64
	P99           float64
	P999          float64
	Max           float64
	Mean          float64
	StdDev        float64
}
