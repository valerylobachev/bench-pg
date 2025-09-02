package sqlx_ex

import (
	"bench-pg-go/model/domain"
	"github.com/jmoiron/sqlx"
	"github.com/shopspring/decimal"
)

type balanceModel struct {
	AccountId string          `db:"account_id"`
	Amount    decimal.Decimal `db:"amount"`
}
type turnaroundModel struct {
	AccountId string          `db:"account_id"`
	Debt      decimal.Decimal `db:"debt"`
	Credit    decimal.Decimal `db:"credit"`
}

func report(db *sqlx.DB, startPeriod, endPeriod domain.Period) {
	startPeriodYp := startPeriod.YearPeriod()
	endPeriodYp := endPeriod.YearPeriod()

	var balance []balanceModel
	err := db.Select(
		&balance,
		`select account_id, sum(amount) as amount 
                 from fin_ledger_items
                 where period < $1
                 group by account_id`,
		startPeriodYp,
	)
	if err != nil {
		panic(err)
	}

	var turnaround []turnaroundModel
	err = db.Select(
		&turnaround,
		`select account_id, sum(debt) as debt, sum(credit) as credit 
                 from fin_ledger_items
                 where period >= $1 and period <= $2
                 group by account_id`,
		startPeriodYp, endPeriodYp,
	)
	if err != nil {
		panic(err)
	}

}
