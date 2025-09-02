package sqlx_ex

import (
	"bench-pg-go/executors/sqlx_ex/model"
	"bench-pg-go/model/domain"
	"database/sql"
	"errors"
	"fmt"
	"github.com/jmoiron/sqlx"
	"github.com/shopspring/decimal"
	"time"
)

func postDocument(tx *sqlx.Tx, document *domain.Document) error {

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
		var materialId sql.Null[string]
		if line.Material != nil {
			materialId.V = *line.Material
			materialId.Valid = true
		}
		var businessPartnerId sql.Null[string]
		if line.BusinessPartner != nil {
			businessPartnerId.V = *line.BusinessPartner
			businessPartnerId.Valid = true
		}
		item := model.LedgerItemEntity{
			Id:                0,
			Period:            periodYp,
			DocNo:             document.DocNo,
			PostingDate:       document.PostingDate.String(),
			AccountId:         line.Account,
			MaterialId:        materialId,
			BusinessPartnerId: businessPartnerId,
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

	_, err := tx.NamedExec(
		`insert into fin_ledger_items ( period, doc_no, posting_date,
                          account_id, business_partner_id, material_id, debt_credit,
                          amount, debt, credit, quantity,
                          updated_by, updated_at)
           		 values (:period, :doc_no, :posting_date,
                         :account_id, :business_partner_id, :material_id, :debt_credit,
                         :amount, :debt, :credit, :quantity,
                         :updated_by, :updated_at)`,
		items,
	)
	if err != nil {
		return err
	}

	return nil
}
