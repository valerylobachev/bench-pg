package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
	"time"
)

func purchaseMaterial(db *gorm.DB, op *domain.Purchase, user domain.User) {
	tx := db.Begin()
	if tx.Error != nil {
		panic(tx.Error)
	}

	period := domain.PeriodFromDate(op.PostingDate)
	materialId := op.Material.Id()
	userId := user.Id()
	updatedAt := time.Now()

	var mp model.MaterialPeriodEntity

	res := tx.Select("std_price", "mov_avg_price", "stock").
		Where("material_id = ? and period = ?", materialId, period.YearPeriod()).
		Clauses(clause.Locking{Strength: "UPDATE"}).
		First(&mp)
	if res.Error != nil {
		panic(res.Error)
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

	err := postDocument(tx, document)
	if err != nil {
		panic(err)
	}

	amount := op.Price.Mul(op.Quantity)
	// ((mp.mov_avg_price * mp.stock + amount) / (mp.stock + op.quantity)).round_dp(2);
	newMovAvgPrice := mp.MovAvgPrice.Mul(mp.Stock).Add(amount).DivRound(mp.Stock.Add(op.Quantity), 2)
	update := map[string]interface{}{
		"mov_avg_price": newMovAvgPrice,
		"stock":         gorm.Expr("stock + ? ", op.Quantity),
		"updated_by":    userId,
		"updated_at":    updatedAt,
	}

	res = tx.Model(&model.MaterialPeriodEntity{}).
		Where("material_id = ? and period >= ?", materialId, period.YearPeriod()).
		Updates(&update)
	if res.Error != nil {
		panic(res.Error)
	}

	res = tx.Commit()
	if res.Error != nil {
		panic(res.Error)
	}
}
