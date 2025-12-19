package benchmark

import (
	"bench-pg-go/api"
	config2 "bench-pg-go/config"
	"bench-pg-go/model/domain"
	"bench-pg-go/model/metrics"
	"fmt"
	"math/rand"
	"sync"
	"time"
)

var randGen = rand.New(rand.NewSource(time.Now().UnixNano()))

func Run(
	config *config2.Config,
	executor api.ExecutorApi,
) []metrics.DomainMetric {

	executor.Init(
		config.StartYear,
		config.Customers,
		config.Vendors,
		config.Materials,
		domain.ChartOfAccounts(),
		initPurchases(config.StartYear, config.Vendors, config.Materials),
	)

	metricArr := make([]metrics.DomainMetric, 0)
	start := time.Now()
	for year := config.StartYear; year < config.StartYear+config.Years; year++ {
		yearMetrics := runYear(year, executor, config)
		metricArr = append(metricArr, yearMetrics...)
	}
	duration := time.Now().Sub(start)
	fmt.Printf("Processing %d years done in %f\n", config.Years, duration.Seconds())

	return metricArr

}

func runYear(year int, executor api.ExecutorApi, config *config2.Config) []metrics.DomainMetric {
	metricArr := make([]metrics.DomainMetric, 0)
	start := time.Now()
	for period := 1; period <= 12; period++ {
		periodMetrics :=
			runPeriod(domain.NewPeriod(year, period), executor, config)
		metricArr = append(metricArr, periodMetrics...)
	}
	duration := time.Now().Sub(start)
	fmt.Printf("Processing year %d done in %f\n", year, duration.Seconds())
	metricArr = append(metricArr,
		metrics.DomainMetric{
			Year:     year,
			Period:   nil,
			Index:    0,
			UserNo:   0,
			Action:   metrics.NewProcessYear(),
			Duration: duration.Seconds(),
		})

	return metricArr
}

func runPeriod(
	period domain.Period,
	executor api.ExecutorApi,
	config *config2.Config,
) []metrics.DomainMetric {
	fmt.Printf("Processing period: %s\n", period.String())

	operations := prepareOperations(period, config)

	start := time.Now()

	tasks := make(chan Task, len(operations))
	results := make(chan TaskResult, len(operations))
	var wg sync.WaitGroup
	wg.Add(config.Users)

	for u := range config.Users {
		user := domain.NewUser(u)
		go userWorker(user, executor, tasks, results, &wg)
	}

	go func() {
		for index, operation := range operations {
			tasks <- Task{
				Index:     index,
				Operation: operation,
			}
		}
		close(tasks)
	}()

	go func() {
		wg.Wait()
		close(results)
	}()

	metricArr := make([]metrics.DomainMetric, 0, len(operations))
	for res := range results {
		metricArr = append(metricArr, res.Metric)
	}

	duration := time.Now().Sub(start)

	fmt.Printf("Processing period %s done in %f\n", period.String(), duration.Seconds())
	month := period.Month()
	metricArr = append(metricArr,
		metrics.DomainMetric{
			Year:     period.Year(),
			Period:   &month,
			Index:    0,
			UserNo:   0,
			Action:   metrics.NewProcessPeriod(),
			Duration: duration.Seconds(),
		})

	return metricArr
}
