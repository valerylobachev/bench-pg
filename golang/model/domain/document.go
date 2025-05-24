package domain

import (
	"cloud.google.com/go/civil"
	"github.com/shopspring/decimal"
)

type Document struct {
	PostingDate civil.Date
	DocNo       string
	UpdatedBy   User
	Lines       []DocumentLine
}

type DocumentLine struct {
	Account         string
	BusinessPartner *string
	Material        *string
	DebtCredit      string
	Amount          decimal.Decimal
	Quantity        *decimal.Decimal
}

func NewPurchaseDocument(
	postingDate civil.Date,
	docNo string,
	material Material,
	vendor Vendor,
	price decimal.Decimal,
	stdPrice decimal.Decimal,
	quantity decimal.Decimal,
	updatedBy User,
) *Document {

	amount := price.Mul(quantity)
	stdAmount := stdPrice.Mul(quantity)
	diffAmount := amount.Sub(stdAmount)
	businessPartner := vendor.Id()
	materialId := material.Id()
	lines := make([]DocumentLine, 0, 3)
	lines = append(lines,
		DocumentLine{
			Account:         INVENTORY_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: nil,
			DebtCredit:      DEBT,
			Amount:          amount,
			Quantity:        &quantity,
		},
	)

	if diffAmount.Equal(decimal.NewFromInt(0)) {
		debtCredit := CREDIT
		if diffAmount.GreaterThan(decimal.NewFromInt(0)) {
			debtCredit = DEBT
		}
		diffLine := DocumentLine{
			Account:         INVENTORY_DIFF_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: nil,
			DebtCredit:      debtCredit,
			Amount:          diffAmount,
			Quantity:        nil,
		}
		lines = append(lines, diffLine)
	}

	creditorLine := DocumentLine{
		Account:         PAYABLE_ACCOUNT,
		Material:        nil,
		BusinessPartner: &businessPartner,
		DebtCredit:      CREDIT,
		Amount:          amount.Neg(),
		Quantity:        nil,
	}
	lines = append(lines, creditorLine)
	return &Document{
		PostingDate: postingDate,
		DocNo:       docNo,
		UpdatedBy:   updatedBy,
		Lines:       lines,
	}

}

func NewCostDocument(
	postingDate civil.Date,
	docNo string,
	material Material,
	vendor Vendor,
	amount decimal.Decimal,
	updatedBy User,
) *Document {
	businessPartner := vendor.Id()
	materialId := material.Id()
	lines := []DocumentLine{
		DocumentLine{
			Account:         INVENTORY_DIFF_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: nil,
			DebtCredit:      DEBT,
			Amount:          amount,
			Quantity:        nil,
		},
		DocumentLine{
			Account:         PAYABLE_ACCOUNT,
			Material:        nil,
			BusinessPartner: &businessPartner,
			DebtCredit:      CREDIT,
			Amount:          amount.Neg(),
			Quantity:        nil,
		},
	}
	return &Document{
		PostingDate: postingDate,
		DocNo:       docNo,
		UpdatedBy:   updatedBy,
		Lines:       lines,
	}
}

func NewCogsDocument(
	postingDate civil.Date,
	docNo string,
	customer Customer,
	material Material,
	stdPrice decimal.Decimal,
	quantity decimal.Decimal,
	updatedBy User,
) *Document {
	businessPartner := customer.Id()
	materialId := material.Id()
	stdAmount := stdPrice.Mul(quantity)
	lines := []DocumentLine{
		DocumentLine{
			Account:         COGS_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: &businessPartner,
			DebtCredit:      DEBT,
			Amount:          stdAmount,
			Quantity:        &quantity,
		},
		DocumentLine{
			Account:         INVENTORY_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: nil,
			DebtCredit:      CREDIT,
			Amount:          stdAmount.Neg(),
			Quantity:        &quantity,
		},
	}
	return &Document{
		PostingDate: postingDate,
		DocNo:       docNo,
		UpdatedBy:   updatedBy,
		Lines:       lines,
	}
}

func NewSaleDocument(
	postingDate civil.Date,
	docNo string,
	customer Customer,
	material Material,
	price decimal.Decimal,
	quantity decimal.Decimal,
	updatedBy User,
) *Document {
	amount := price.Mul(quantity)
	businessPartner := customer.Id()
	materialId := material.Id()
	lines := []DocumentLine{
		DocumentLine{
			Account:    RECEIVABLE_ACCOUNT,
			Material:   nil,
			DebtCredit: DEBT,
			Amount:     amount,
			Quantity:   &quantity,
		},
		DocumentLine{
			Account:         SALES_ACCOUNT,
			Material:        &materialId,
			BusinessPartner: &businessPartner,
			DebtCredit:      CREDIT,
			Amount:          amount.Neg(),
			Quantity:        &quantity,
		},
	}
	return &Document{
		PostingDate: postingDate,
		DocNo:       docNo,
		UpdatedBy:   updatedBy,
		Lines:       lines,
	}

}
