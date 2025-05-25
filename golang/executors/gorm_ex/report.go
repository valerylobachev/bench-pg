package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"gorm.io/gorm"
)

func report(db *gorm.DB, startPeriod, endPeriod domain.Period) {
	startPeriodYp := startPeriod.YearPeriod()
	endPeriodYp := endPeriod.YearPeriod()

	var balance map[string]interface{}

	res := db.Model(&model.LedgerItemEntity{}).
		Select("account_id", "sum(amount)").
		Where("period < ? ", startPeriodYp).
		Group("account_id").
		Scan(&balance)
	if res.Error != nil {
		panic(res.Error)
	}

	var turnaround map[string]interface{}
	res = db.Model(&model.LedgerItemEntity{}).
		Select("account_id", "sum(debt) as debt", "sum(credit) as credit").
		Where("period >= ? and period <= ? ", startPeriodYp, endPeriodYp).
		Group("account_id").
		Scan(&turnaround)
	if res.Error != nil {
		panic(res.Error)
	}
}
