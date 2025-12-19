package benchmark

import (
	"bench-pg-go/api"
	"bench-pg-go/model/domain"
	"bench-pg-go/model/metrics"
	"fmt"
	"sync"
	"time"
)

type operationResult struct {
	year   int
	month  int
	action metrics.Action
}

func userWorker(user domain.User, executor api.ExecutorApi, tasks chan Task, results chan TaskResult, wg *sync.WaitGroup) {
	defer wg.Done()
	for task := range tasks {

		start := time.Now()
		result := processOperation(user, task.Operation, executor)
		duration := time.Now().Sub(start)

		results <- TaskResult{
			metrics.DomainMetric{
				Year:     result.year,
				Period:   &result.month,
				Index:    task.Index,
				UserNo:   user.No(),
				Action:   result.action,
				Duration: duration.Seconds(),
			},
		}
	}
}

func processOperation(
	user domain.User,
	operation domain.Operation,
	executor api.ExecutorApi,
) operationResult {
	var result operationResult
	switch op := operation.(type) {
	case domain.Purchase:
		executor.PurchaseMaterial(&op, user)
		p := domain.PeriodFromDate(op.PostingDate)
		result = operationResult{p.Year(), p.Month(), metrics.Operation{op}}
	case domain.Sale:
		executor.SellMaterial(&op, user)
		p := domain.PeriodFromDate(op.PostingDate)
		result = operationResult{p.Year(), p.Month(), metrics.Operation{op}}
	case domain.Cost:
		executor.AccountCost(&op, user)
		p := domain.PeriodFromDate(op.PostingDate)
		result = operationResult{p.Year(), p.Month(), metrics.Operation{op}}
	case domain.PeriodReport:
		executor.PeriodReport(op.Period)
		result = operationResult{op.Period.Year(), op.Period.Month(), metrics.Operation{op}}
	case domain.YearReport:
		executor.YearReport(op.Period)
		result = operationResult{op.Period.Year(), op.Period.Month(), metrics.Operation{op}}
	case domain.OpenPeriod:
		start := time.Now()
		executor.OpenPeriod(op.Period, user)
		duration := time.Now().Sub(start)
		fmt.Printf("Open period %s done in %f\n", op.Period.NextPeriod().String(), duration.Seconds())
		result = operationResult{op.Period.Year(), op.Period.Month(), metrics.Operation{op}}
	case domain.ClosePeriod:
		start := time.Now()
		executor.ClosePeriod(op.Period, user)
		duration := time.Now().Sub(start)
		fmt.Printf("Close period %s done in %f\n", op.Period.PrevPeriod().String(), duration.Seconds())
		result = operationResult{op.Period.Year(), op.Period.Month(), metrics.Operation{op}}
	}
	return result
}
