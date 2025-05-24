package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"errors"
	"fmt"
	"github.com/shopspring/decimal"
	"gorm.io/gorm"
	"time"
)

func postDocument(tx *gorm.DB, document *domain.Document) error {

	balance := decimal.Zero
	for _, line := range document.Lines {
		balance = balance.Add(line.Amount)
	}
	if !balance.Equal(decimal.Zero) {
		return errors.New(fmt.Sprintf("balance is %s, must be zero", balance.String()))
	}

	periodYp := domain.PeriodFromDate(document.PostingDate).YearPeriod()
	userId := document.UpdatedBy.Id()
	updatedAt := time.Now()

	items := make([]model.LedgerItemEntity, 0, len(document.Lines))
	for _, line := range document.Lines {
		debt := decimal.Zero
		if line.DebtCredit == domain.DEBT {
			debt = line.Amount
		}
		credit := decimal.Zero
		if line.DebtCredit == domain.CREDIT {
			credit = line.Amount.Neg()
		}
		quantity := decimal.Zero
		if line.Quantity != nil {
			quantity = *line.Quantity
		}
		item := model.LedgerItemEntity{
			Id:                0,
			Period:            periodYp,
			DocNo:             document.DocNo,
			PostingDate:       document.PostingDate.String(),
			AccountId:         line.Account,
			MaterialId:        line.Material,
			BusinessPartnerId: line.BusinessPartner,
			DebtCredit:        line.DebtCredit,
			Amount:            line.Amount,
			Debt:              debt,
			Credit:            credit,
			Quantity:          quantity,
			UpdatedBy:         userId,
			UpdatedAt:         updatedAt,
		}
		items = append(items, item)
	}

	res := tx.Create(&items)
	if res.Error != nil {
		return res.Error
	}

	return nil
}
