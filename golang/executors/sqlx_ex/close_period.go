package sqlx_ex

import (
	"bench-pg-go/executors/sqlx_ex/model"
	"bench-pg-go/model/domain"
	"cloud.google.com/go/civil"
	"database/sql"
	"errors"
	"fmt"
	"github.com/jmoiron/sqlx"
	"github.com/shopspring/decimal"
	"time"
)

type cdRecord struct {
	prevStock    decimal.Decimal `sqlx:"column:prev_stock;not null"`
	receipt      decimal.Decimal `sqlx:"column:receipt;not null"`
	consumption  decimal.Decimal `sqlx:"column:consumption;not null"`
	diffAmount   decimal.Decimal `sqlx:"column:diff_amount;not null"`
	nextStdPrice decimal.Decimal `sqlx:"column:next_std_price;not null"`
}

func closePeriod(db *sqlx.DB, period domain.Period, user domain.User) {

	userId := user.Id()

	var materialIds []string
	err := db.Select(&materialIds, `select id from fin_materials`)
	if err != nil {
		panic(err)
	}

	for _, materialId := range materialIds {
		currYp := period.YearPeriod()
		nextYp := period.NextPeriod().YearPeriod()
		prevYp := period.PrevPeriod().YearPeriod()
		updatedAt := time.Now()

		tx, err := db.Beginx()
		if err != nil {
			panic(err)
		}

		var stdPrice decimal.Decimal
		err = tx.Get(&stdPrice,
			`select std_price from fin_material_periods
                 where material_id = $1 and period = $2 for update;`,
			materialId, currYp,
		)
		if err != nil {
			panic(err)
		}

		var prevStock decimal.Decimal
		var receipt decimal.Decimal
		var consumption decimal.Decimal
		var diffAmount decimal.Decimal
		var nextStdPrice decimal.Decimal

		err = tx.Get(
			&prevStock,
			`select stock from fin_material_periods
                     where material_id = $1 and period = $2`,
			materialId, prevYp,
		)
		if err != nil && !errors.Is(err, sql.ErrNoRows) {
			panic(err)
		}

		err = tx.Get(
			&receipt,
			`select COALESCE(sum(quantity),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.01' and debt_credit = 'D'`,
			currYp, materialId,
		)
		if err != nil {
			panic(err)
		}

		err = tx.Get(
			&consumption,
			`select COALESCE(sum(quantity),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.01' and debt_credit = 'C'`,
			currYp, materialId,
		)
		if err != nil {
			panic(err)
		}

		err = tx.Get(
			&diffAmount,
			`select  COALESCE(sum(amount),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.02'`,
			currYp, materialId,
		)
		if err != nil {
			panic(err)
		}

		err = tx.Get(
			&nextStdPrice,
			`select std_price from fin_material_periods
                     where material_id = $1 and period = $2`,
			materialId, nextYp,
		)
		if err != nil {
			panic(err)
		}

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
		_, err = tx.Exec(
			`update fin_materials
					 set next_std_price = $1,
						 updated_by = $2, 
						 updated_at = $3
					 where id = $4`,
			nextStdPrice, userId, updatedAt, materialId,
		)
		if err != nil {
			panic(err)
		}

		// update actual price
		_, err = tx.Exec(
			`update fin_material_periods set 
					  actual_price = $1,
					  updated_by = $2, 
					  updated_at = $3 
				   where material_id = $4 and period = $5`,
			actualPrice, userId, updatedAt, materialId, currYp,
		)
		if err != nil {
			panic(err)
		}

		err = tx.Commit()
		if err != nil {
			panic(err)
		}
	}
}

func postDifferences(
	tx *sqlx.Tx,
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
			MaterialId:        sql.Null[string]{V: materialId, Valid: true},
			BusinessPartnerId: sql.Null[string]{},
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
			MaterialId:        sql.Null[string]{V: materialId, Valid: true},
			BusinessPartnerId: sql.Null[string]{},
			DebtCredit:        domain.CREDIT,
			Amount:            amount.Neg(),
			Credit:            decimal.Zero,
			Debt:              amount,
			Quantity:          decimal.Zero,
			UpdatedBy:         userId,
			UpdatedAt:         updatedAt,
		},
	}

	_, err := tx.NamedExec(
		`insert into fin_ledger_items ( period, doc_no, posting_date,
                          account_id, business_partner_id, material_id, debt_credit,
                          amount, debt, credit, quantity,
                          updated_by, updated_at)
           		 values (:period, :doc_no, :posting_date,
                         :account_id, :business_partner_id, :material_id, :debt_credit,
                         :amount, :debt, :credit, :quantity,
                         :updated_by, :updated_at)`,
		lines,
	)
	if err != nil {
		panic(err)
	}

}
