package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"cloud.google.com/go/civil"
	"fmt"
	"github.com/shopspring/decimal"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
	"time"
)

type cdRecord struct {
	prevStock    decimal.Decimal `gorm:"column:prev_stock;not null"`
	receipt      decimal.Decimal `gorm:"column:receipt;not null"`
	consumption  decimal.Decimal `gorm:"column:consumption;not null"`
	diffAmount   decimal.Decimal `gorm:"column:diff_amount;not null"`
	nextStdPrice decimal.Decimal `gorm:"column:next_std_price;not null"`
}

func closePeriod(db *gorm.DB, period domain.Period, user domain.User) {

	userId := user.Id()

	materials := make([]model.MaterialEntity, 0)
	res := db.Select("id").Find(&materials)
	if res.Error != nil {
		panic(res.Error)
	}

	for _, material := range materials {
		materialId := material.Id
		currYp := period.YearPeriod()
		nextYp := period.NextPeriod().YearPeriod()
		prevYp := period.PrevPeriod().YearPeriod()
		updatedAt := time.Now()

		tx := db.Begin()
		if tx.Error != nil {
			panic(tx.Error)
		}

		var mp model.MaterialPeriodEntity
		res := tx.Select("std_price").
			Where("material_id = ? and period = ?", materialId, currYp).
			Clauses(clause.Locking{Strength: "UPDATE"}).
			First(&mp)
		if res.Error != nil {
			panic(res.Error)
		}

		var prevStock decimal.Decimal
		var receipt decimal.Decimal
		var consumption decimal.Decimal
		var diffAmount decimal.Decimal
		var nextStdPrice decimal.Decimal

		prevStockSubQuery := tx.Select("stock").
			Where("material_id = ? and period = ?", materialId, prevYp).
			Table("fin_material_periods").Scan(&prevStock)
		if prevStockSubQuery.Error != nil {
			panic(prevStockSubQuery.Error)
		}

		receiptSubQuery := tx.Select("COALESCE(sum(quantity),0)").
			Where("material_id = ? and period = ? and account_id = '10.01' and debt_credit = 'D'",
				materialId, currYp).
			Table("fin_ledger_items").Scan(&receipt)
		if receiptSubQuery.Error != nil {
			panic(receiptSubQuery.Error)
		}

		consumptionSubQuery := tx.Select("COALESCE(sum(quantity),0)").
			Where("material_id = ? and period = ? and account_id = '10.01' and debt_credit = 'C'",
				materialId, currYp).
			Table("fin_ledger_items").Scan(&consumption)
		if consumptionSubQuery.Error != nil {
			panic(consumptionSubQuery.Error)
		}

		diffAmountSubQuery := tx.Select("COALESCE(sum(amount),0)").
			Where("material_id = ? and period = ? and account_id = '10.01' and debt_credit = 'C'",
				materialId, currYp).
			Table("fin_ledger_items").Scan(&diffAmount)
		if diffAmountSubQuery.Error != nil {
			panic(diffAmountSubQuery.Error)
		}

		nextStdPriceSubQuery := tx.Select("std_price").
			Where("material_id = ? and period = ?", materialId, nextYp).
			Table("fin_material_periods").Scan(&nextStdPrice)
		if nextStdPriceSubQuery.Error != nil {
			panic(nextStdPriceSubQuery.Error)
		}

		//var cd cdRecord
		//res = tx.Select("(?) as prev_stock, (?) as receipt, (?) as consumption, (?) as diff_amount, (?) as next_std_price",
		//	prevStockSubQuery, receiptSubQuery, consumptionSubQuery, diffAmountSubQuery, nextStdPriceSubQuery).
		//	Table("fin_material_periods").
		//	Scan(&cd)
		//if res.Error != nil {
		//	panic(res.Error)
		//}

		stdPrice := mp.StdPrice
		//prevStock := cd.prevStock
		//receipt := cd.receipt
		//consumption := cd.consumption
		//diffAmount := cd.diffAmount
		//nextStdPrice := cd.nextStdPrice

		// Общая стоимость: ta2 = s1 * sp2 + r2 * sp2 +d2
		totalAmount := prevStock.Mul(stdPrice).Add(receipt.Mul(stdPrice)).Add(diffAmount)
		// Общий запас: ts2 = s1 + r2
		totalStock := prevStock.Add(receipt)
		// Факт цена: ap = ta2 / ts2
		actualPrice := totalAmount.Div(totalStock)
		// Запас на конец периода: s3 = s1 + r2 + c2
		currStock := prevStock.Add(receipt).Add(consumption)

		docNo := fmt.Sprintf("CLOSE-%s-%s", period.String(), materialId)

		// Отклонения на запас: ds2 = d2 * s3 / (s1 + r2)
		// Проводка Dt 10.01 Ct 10.02 ds2
		diffToStockAmount := diffAmount.Mul(currStock).Div(prevStock.Add(receipt))

		postDifferences(
			tx,
			period.LastDate(),
			docNo,
			materialId,
			diffToStockAmount,
			domain.INVENTORY_ACCOUNT,
			domain.INVENTORY_DIFF_ACCOUNT,
			userId,
			updatedAt,
		)

		// Отклонения на COGS: dc2 = d2 - ds2
		// Проводка Dt 90.02 Ct 10.02 dc2
		diffToCogsAmount := diffAmount.Sub(diffToStockAmount)
		postDifferences(
			tx,
			period.LastDate(),
			docNo,
			materialId,
			diffToCogsAmount,
			domain.COGS_ACCOUNT,
			domain.INVENTORY_DIFF_ACCOUNT,
			userId,
			updatedAt,
		)

		docNo = fmt.Sprintf("OPEN-%s-%s", period.String(), materialId)

		// Отклонение след. периода ds3 = s3 * sp2 + ds2 - s3 * sp3
		// Проводка Dt 10.02 Ct 10.01 ds3
		nextDiffAmount := currStock.Mul(stdPrice).Add(diffToStockAmount).Sub(currStock.Mul(nextStdPrice))
		postDifferences(
			tx,
			period.NextPeriod().FirstDate(),
			docNo,
			materialId,
			nextDiffAmount,
			domain.INVENTORY_DIFF_ACCOUNT,
			domain.INVENTORY_ACCOUNT,
			userId,
			updatedAt,
		)

		// update next_std_price material period
		res = tx.Model(&model.MaterialEntity{}).
			Where("id = ?", materialId).
			Updates(map[string]interface{}{
				"next_std_price": nextStdPrice,
				"updated_by":     userId,
				"updated_at":     updatedAt,
			})
		if res.Error != nil {
			panic(res.Error)
		}

		// update actual price
		res = tx.Model(&model.MaterialPeriodEntity{}).
			Where("material_id = ? and period = ?", materialId, currYp).
			Updates(map[string]interface{}{
				"actual_price": actualPrice,
				"updated_by":   userId,
				"updated_at":   updatedAt,
			})
		if res.Error != nil {
			panic(res.Error)
		}

		res = tx.Commit()
		if res.Error != nil {
			panic(res.Error)
		}
	}
}

func postDifferences(
	tx *gorm.DB,
	postingDate civil.Date,
	docNo string,
	materialId string,
	amount decimal.Decimal,
	debtAccount string,
	creditAccount string,
	userId string,
	updatedAt time.Time,
) {

	if amount.Equal(decimal.Zero) {
		return
	}

	if amount.LessThan(decimal.Zero) {
		amount = amount.Neg()
		acc := debtAccount
		debtAccount = creditAccount
		creditAccount = acc
	}

	period := domain.PeriodFromDate(postingDate)

	lines := []model.LedgerItemEntity{
		{
			Id:                0,
			Period:            period.YearPeriod(),
			DocNo:             docNo,
			PostingDate:       postingDate.String(),
			AccountId:         debtAccount,
			MaterialId:        &materialId,
			BusinessPartnerId: nil,
			DebtCredit:        domain.DEBT,
			Amount:            amount,
			Debt:              amount,
			Credit:            decimal.Zero,
			Quantity:          decimal.Zero,
			UpdatedBy:         userId,
			UpdatedAt:         updatedAt,
		},
		{
			Id:                0,
			Period:            period.YearPeriod(),
			DocNo:             docNo,
			PostingDate:       postingDate.String(),
			AccountId:         creditAccount,
			MaterialId:        &materialId,
			BusinessPartnerId: nil,
			DebtCredit:        domain.CREDIT,
			Amount:            amount.Neg(),
			Credit:            decimal.Zero,
			Debt:              amount,
			Quantity:          decimal.Zero,
			UpdatedBy:         userId,
			UpdatedAt:         updatedAt,
		},
	}

	res := tx.Create(&lines)
	if res.Error != nil {
		panic(res.Error)
	}

}
