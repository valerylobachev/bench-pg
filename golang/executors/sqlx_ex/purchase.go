package sqlx_ex

import (
	"bench-pg-go/executors/sqlx_ex/model"
	"bench-pg-go/model/domain"
	"github.com/jmoiron/sqlx"
	"time"
)

func purchaseMaterial(db *sqlx.DB, op *domain.Purchase, user domain.User) {
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
		`select std_price, mov_avg_price, stock
                 from fin_material_periods
                 where material_id=$1 and period=$2 for update`,
		materialId, period.YearPeriod(),
	)
	if err != nil {
		panic(err)
	}

	document := domain.NewPurchaseDocument(
		op.PostingDate,
		op.DocNo,
		op.Material,
		op.Vendor,
		op.Price,
		mp.StdPrice,
		op.Quantity,
		user,
	)

	err = postDocument(tx, document)
	if err != nil {
		panic(err)
	}

	amount := op.Price.Mul(op.Quantity)
	// ((mp.mov_avg_price * mp.stock + amount) / (mp.stock + op.quantity)).round_dp(2);
	newMovAvgPrice := mp.MovAvgPrice.Mul(mp.Stock).Add(amount).DivRound(mp.Stock.Add(op.Quantity), 2)
	_, err = tx.Exec(
		`update fin_material_periods set 
                    mov_avg_price = $1, 
                    stock = stock + $2,  
                    updated_by = $3,   
                    updated_at = $6   
                 where material_id = $4 and period >= $5`,
		newMovAvgPrice,
		op.Quantity,
		userId,
		materialId,
		period.YearPeriod(),
		updatedAt,
	)
	if err != nil {
		panic(err)
	}

	err = tx.Commit()
	if err != nil {
		panic(err)
	}
}
