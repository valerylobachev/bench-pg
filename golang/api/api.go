package api

import "bench-pg-go/model/domain"

type ExecutorApi interface {
	Init(
		startYear int,
		customers int,
		vendors int,
		materials int,
		accounts []domain.Account,
		purchases []domain.Purchase,
	)
	PurchaseMaterial(operation *domain.Purchase, user domain.User)
	SellMaterial(operation *domain.Sale, user domain.User)
	AccountCost(operation *domain.Cost, user domain.User)
	OpenPeriod(period domain.Period, user domain.User)
	ClosePeriod(period domain.Period, user domain.User)
	PeriodReport(period domain.Period)
	YearReport(period domain.Period)
}
