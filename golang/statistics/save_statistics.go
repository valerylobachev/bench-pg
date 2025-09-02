package statistics

import (
	"bench-pg-go/model/domain"
	"bench-pg-go/model/metrics"
	"bench-pg-go/statistics/model"
	"log"

	"github.com/samber/lo"
	"gonum.org/v1/gonum/floats"
	"gonum.org/v1/gonum/stat"
	"gorm.io/gorm"
)

type statCondition struct {
	Action      string
	FilterFn    func(metrics.DomainMetric) bool
	CalcPeriods bool
}

func newStatCondition(
	action string,
	filterFn func(metrics.DomainMetric) bool,
	calcPeriods bool,
) statCondition {
	return statCondition{Action: action, FilterFn: filterFn, CalcPeriods: calcPeriods}
}

var PROCESSING_RULES = []statCondition{
	newStatCondition("PROCESS_YEAR", filterByProcessYear, false),
	newStatCondition("PROCESS_PERIOD", filterByProcessPeriod, false),
	newStatCondition("YEAR_REPORT", filterByYearReport, true),
	newStatCondition("PERIOD_REPORT", filterByPeriodReport, true),
	newStatCondition("OPEN_PERIOD", filterByOpenPeriod, false),
	newStatCondition("CLOSE_PERIOD", filterByClosePeriod, false),
	newStatCondition("COST", filterByCost, true),
	newStatCondition("SALE", filterBySale, true),
	newStatCondition("PURCHASE", filterByPurchase, true),
}

func saveStatistics(db *gorm.DB, benchmarkId int64, domainMetrics []metrics.DomainMetric, startYear int, years int) {
	for _, rule := range PROCESSING_RULES {
		actionMetrics := lo.Filter(domainMetrics, func(item metrics.DomainMetric, index int) bool {
			return rule.FilterFn(item)
		})
		durations := lo.Map(actionMetrics, func(item metrics.DomainMetric, index int) float64 {
			return item.Duration
		})

		totalStat := calculateStatistics(durations)
		saveStat(db, benchmarkId, nil, nil, rule.Action, totalStat)

		for year := startYear; year < startYear+years; year++ {
			yearMetrics := lo.Filter(actionMetrics, func(item metrics.DomainMetric, index int) bool {
				return item.Year == year
			})
			durations := lo.Map(yearMetrics, func(item metrics.DomainMetric, index int) float64 {
				return item.Duration
			})
			stat := calculateStatistics(durations)
			saveStat(db, benchmarkId, &year, nil, rule.Action, stat)

			if rule.CalcPeriods {
				for month := 1; month <= 12; month++ {
					monthMetrics := lo.Filter(yearMetrics, func(item metrics.DomainMetric, index int) bool {
						return item.Period != nil && *item.Period == month
					})
					durations := lo.Map(monthMetrics, func(item metrics.DomainMetric, index int) float64 {
						return item.Duration
					})
					stat := calculateStatistics(durations)
					saveStat(db, benchmarkId, &year, &month, rule.Action, stat)
				}
			}

		}

	}
}

func saveStat(db *gorm.DB, benchmarkId int64, year *int, month *int, action string, s metrics.Statistics) {
	entity := model.StatisticEntity{
		Id:            0,
		BenchmarkId:   benchmarkId,
		Action:        action,
		Year:          year,
		Month:         month,
		TotalCount:    int64(s.TotalCount),
		TotalDuration: s.TotalDuration,
		OpsPerSec:     s.OpsPerSec,
		Min:           s.Min,
		P50:           s.P50,
		P75:           s.P75,
		P95:           s.P95,
		P99:           s.P99,
		P99_9:         s.P999,
		Max:           s.Max,
		Mean:          s.Mean,
		StdDev:        s.StdDev,
	}
	res := db.Create(&entity)
	if err := res.Error; err != nil {
		log.Fatalf("Error inserting metric: %v", err)
	}
}

func calculateStatistics(durations []float64) metrics.Statistics {
	totalCount := len(durations)
	totalDuration := floats.Sum(durations)
	var smin float64
	var p50 float64
	var p75 float64
	var p95 float64
	var p99 float64
	var p999 float64
	var smax float64
	var mean float64
	var stdDev float64
	if totalCount > 0 {
		stat.SortWeighted(durations, nil)
		smin = floats.Min(durations)
		p50 = stat.Quantile(0.5, stat.Empirical, durations, nil)
		p75 = stat.Quantile(0.75, stat.Empirical, durations, nil)
		p95 = stat.Quantile(0.95, stat.Empirical, durations, nil)
		p99 = stat.Quantile(0.99, stat.Empirical, durations, nil)
		p999 = stat.Quantile(0.999, stat.Empirical, durations, nil)
		smax = floats.Max(durations)
		mean = stat.Mean(durations, nil)
		stdDev = stat.StdDev(durations, nil)
	}
	statistics := metrics.Statistics{
		TotalCount:    totalCount,
		TotalDuration: totalDuration,
		OpsPerSec:     float64(totalCount) / totalDuration,
		Min:           smin,
		P50:           p50,
		P75:           p75,
		P95:           p95,
		P99:           p99,
		P999:          p999,
		Max:           smax,
		Mean:          mean,
		StdDev:        stdDev,
	}
	return statistics
}

func filterByProcessYear(m metrics.DomainMetric) bool {
	_, ok := m.Action.(metrics.ProcessYear)
	return ok
}

func filterByProcessPeriod(m metrics.DomainMetric) bool {
	_, ok := m.Action.(metrics.ProcessPeriod)
	return ok
}
func filterByYearReport(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.YearReport)
		return ok
	}
	return false
}
func filterByPeriodReport(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.PeriodReport)
		return ok
	}
	return false
}
func filterByOpenPeriod(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.OpenPeriod)
		return ok
	}
	return false
}
func filterByClosePeriod(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.ClosePeriod)
		return ok
	}
	return false
}
func filterByCost(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.Cost)
		return ok
	}
	return false
}
func filterBySale(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.Sale)
		return ok
	}
	return false
}
func filterByPurchase(m metrics.DomainMetric) bool {
	op, isOp := m.Action.(metrics.Operation)
	if isOp {
		_, ok := op.Operation.(domain.Purchase)
		return ok
	}
	return false
}
