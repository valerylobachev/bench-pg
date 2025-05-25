package gorm_ex

import (
	"bench-pg-go/model/domain"
	"gorm.io/gorm"
)

func accountCost(db *gorm.DB, op *domain.Cost, user domain.User) {
	tx := db.Begin()
	if tx.Error != nil {
		panic(tx.Error)
	}

	document := domain.NewCostDocument(
		op.PostingDate,
		op.DocNo,
		op.Material,
		op.Vendor,
		op.Amount,
		user,
	)
	postDocument(tx, document)

	res := tx.Commit()
	if res.Error != nil {
		panic(res.Error)
	}
}
