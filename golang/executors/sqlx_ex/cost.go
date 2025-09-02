package sqlx_ex

import (
	"bench-pg-go/model/domain"
	"github.com/jmoiron/sqlx"
)

func accountCost(db *sqlx.DB, op *domain.Cost, user domain.User) {
	tx, err := db.Beginx()
	if err != nil {
		panic(err)
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

	err = tx.Commit()
	if err != nil {
		panic(err)
	}
}
