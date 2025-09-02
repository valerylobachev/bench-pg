package sqlx_ex

import (
	"bench-pg-go/executors/sqlx_ex/model"
	"bench-pg-go/model/domain"
	"fmt"
	"github.com/jmoiron/sqlx"
	"time"
)

func sellMaterial(db *sqlx.DB, op *domain.Sale, user domain.User) {
	tx, err := db.Beginx()
	if err != nil {
		panic(err)
	}

	period := domain.PeriodFromDate(op.PostingDate)
	materialId := op.Material.Id()
	userId := user.Id()
	updatedAt := time.Now()

	var mp model.MaterialPeriodEntity
	err = tx.Get(
		&mp,
		`select std_price, sell_price, stock
                 from fin_material_periods
                 where material_id=$1 and period=$2 for update`,
		materialId, period.YearPeriod(),
	)
	if err != nil {
		panic(err)
	}

	if mp.Stock.LessThan(op.Quantity) {
		fmt.Printf(
			"Cannot sell material %s in period %s, stock (%s) lower than required quantity (%s)\n",
			materialId, period.String(), mp.Stock.String(), op.Quantity.String(),
		)
		tx.Rollback()
		return
	}

	cogsDocument := domain.NewCogsDocument(
		op.PostingDate,
		op.CogsDocNo,
		op.Customer,
		op.Material,
		mp.StdPrice,
		op.Quantity,
		user,
	)
	postDocument(tx, cogsDocument)

	saleDocument := domain.NewSaleDocument(
		op.PostingDate,
		op.SaleDocNo,
		op.Customer,
		op.Material,
		mp.SellPrice,
		op.Quantity,
		user,
	)
	postDocument(tx, saleDocument)

	_, err = tx.Exec(
		`update fin_material_periods set 
                    stock = stock - $1,  
                    updated_by = $2,   
                    updated_at = $3   
                 where material_id = $4 and period >= $5`,
		op.Quantity,
		userId,
		updatedAt,
		materialId,
		period.YearPeriod(),
	)
	if err != nil {
		panic(err)
	}

	err = tx.Commit()
	if err != nil {
		panic(err)
	}
}
