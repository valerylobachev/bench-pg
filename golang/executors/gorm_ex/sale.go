package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"fmt"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
	"time"
)

func sellMaterial(db *gorm.DB, op *domain.Sale, user domain.User) {
	tx := db.Begin()
	if tx.Error != nil {
		panic(tx.Error)
	}

	period := domain.PeriodFromDate(op.PostingDate)
	materialId := op.Material.Id()
	userId := user.Id()
	updatedAt := time.Now()

	var mp model.MaterialPeriodEntity

	res := tx.Select("std_price", "sell_price", "stock").
		Where("material_id = ? and period = ?", materialId, period.YearPeriod()).
		Clauses(clause.Locking{Strength: "UPDATE"}).
		First(&mp)
	if res.Error != nil {
		panic(res.Error)
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

	update := map[string]interface{}{
		"stock":      gorm.Expr("stock - ? ", op.Quantity),
		"updated_by": userId,
		"updated_at": updatedAt,
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
