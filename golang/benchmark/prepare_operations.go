package benchmark

import (
	config2 "bench-pg-go/config"
	"bench-pg-go/model/domain"
	"cmp"
	"fmt"
	"github.com/shopspring/decimal"
	"slices"
)

type PreOperation struct {
	operation domain.Operation
	order     int
}

func prepareOperations(period domain.Period, config *config2.Config) []domain.Operation {
	ops := make([]PreOperation, 0, config.Operations)

	for i := 0; i < config.Operations*20/100; i++ {
		order := randGen.Intn(config.Operations)
		operation := domain.NewPeriodReport(period)
		ops = append(ops, PreOperation{operation, order})
	}

	for i := 0; i < config.Operations*10/100; i++ {
		order := randGen.Intn(config.Operations)
		operation := domain.NewYearReport(period)
		ops = append(ops, PreOperation{operation, order})
	}

	for i := 0; i < config.Operations*20/100; i++ {
		order := randGen.Intn(config.Operations)
		postingDate := period.Date(order * period.LastDay() / config.Operations)
		operation := domain.NewCost(
			domain.NewMaterial(randGen.Intn(config.Materials)),
			domain.NewVendor(randGen.Intn(config.Vendors)),
			decimal.NewFromInt(int64(randGen.Intn(1000)+1000)),
			fmt.Sprintf("COST-%s-%08d", period.String(), i),
			postingDate,
		)
		ops = append(ops, PreOperation{operation, order})
	}

	for i := 0; i < config.Operations*25/100; i++ {
		order := randGen.Intn(config.Operations)
		postingDate := period.Date(order * period.LastDay() / config.Operations)
		operation := domain.NewPurchase(
			domain.NewMaterial(randGen.Intn(config.Materials)),
			domain.NewVendor(randGen.Intn(config.Vendors)),
			decimal.NewFromInt(int64(randGen.Intn(1000)+1000)),
			decimal.NewFromInt(int64(randGen.Intn(100)+100)),
			fmt.Sprintf("PURCH-%s-%08d", period.String(), i),
			postingDate,
		)
		ops = append(ops, PreOperation{operation, order})
	}

	for i := 0; i < config.Operations*25/100; i++ {
		order := randGen.Intn(config.Operations)
		postingDate := period.Date(order * period.LastDay() / config.Operations)
		operation := domain.NewSale(
			domain.NewMaterial(randGen.Intn(config.Materials)),
			domain.NewCustomer(randGen.Intn(config.Customers)),
			decimal.NewFromInt(int64(randGen.Intn(100)+100)),
			fmt.Sprintf("SALE-%s-%08d", period.String(), i),
			fmt.Sprintf("COGS-%s-%08d", period.String(), i),
			postingDate,
		)
		ops = append(ops, PreOperation{operation, order})
	}

	ops = append(ops, PreOperation{domain.NewOpenPeriod(period), config.Operations})
	closeOrder := (randGen.Intn(4) + 1) * config.Operations / period.LastDay()
	ops = append(ops, PreOperation{domain.NewClosePeriod(period), closeOrder})

	slices.SortFunc(ops, func(a, b PreOperation) int {
		return cmp.Compare(a.order, b.order)
	})

	operations := make([]domain.Operation, 0, len(ops))
	for _, op := range ops {
		operations = append(operations, op.operation)
	}
	return operations
}
