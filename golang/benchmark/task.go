package benchmark

import (
	"bench-pg-go/model/domain"
	"bench-pg-go/model/metrics"
)

type Task struct {
	Index     int
	Operation domain.Operation
}

type TaskResult struct {
	Metric metrics.DomainMetric
}
