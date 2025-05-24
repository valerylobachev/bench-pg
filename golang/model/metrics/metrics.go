package metrics

type DomainMetric struct {
	Year     int
	Period   *int
	Index    int
	UserNo   int
	Action   Action
	Duration float64
}
