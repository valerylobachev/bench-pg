package domain

import (
	"cloud.google.com/go/civil"
	"github.com/shopspring/decimal"
)

const (
	PurchaseOperation     = "PURCHASE"
	SaleOperation         = "SALE"
	CostOperation         = "COST"
	OpenPeriodOperation   = "OPEN_PERIOD"
	ClosePeriodOperation  = "CLOSE_PERIOD"
	PeriodReportOperation = "PERIOD_REPORT"
	YearReportOperation   = "YEAR_REPORT"
)

type Operation interface {
	Code() string
}

type Purchase struct {
	Material    Material
	Vendor      Vendor
	Quantity    decimal.Decimal
	Price       decimal.Decimal
	DocNo       string
	PostingDate civil.Date
}

func NewPurchase(
	material Material,
	vendor Vendor,
	quantity decimal.Decimal,
	price decimal.Decimal,
	docNo string,
	postingDate civil.Date,
) Purchase {
	return Purchase{
		material,
		vendor,
		quantity,
		price,
		docNo,
		postingDate,
	}
}

func (u Purchase) Code() string {
	return PurchaseOperation
}

type Sale struct {
	Material    Material
	Customer    Customer
	Quantity    decimal.Decimal
	SaleDocNo   string
	CogsDocNo   string
	PostingDate civil.Date
}

func NewSale(
	material Material,
	customer Customer,
	quantity decimal.Decimal,
	saleDocNo string,
	cogsDocNo string,
	postingDate civil.Date,
) Sale {
	return Sale{
		material,
		customer,
		quantity,
		saleDocNo,
		cogsDocNo,
		postingDate,
	}
}

func (u Sale) Code() string {
	return SaleOperation
}

type Cost struct {
	Material    Material
	Vendor      Vendor
	Amount      decimal.Decimal
	DocNo       string
	PostingDate civil.Date
}

func NewCost(
	material Material,
	vendor Vendor,
	amount decimal.Decimal,
	docNo string,
	postingDate civil.Date,
) Cost {
	return Cost{
		material,
		vendor,
		amount,
		docNo,
		postingDate,
	}
}

func (u Cost) Code() string {
	return CostOperation
}

type OpenPeriod struct {
	Period Period
}

func NewOpenPeriod(period Period) OpenPeriod {
	return OpenPeriod{period}
}

func (u OpenPeriod) Code() string {
	return OpenPeriodOperation
}

type ClosePeriod struct {
	Period Period
}

func NewClosePeriod(period Period) ClosePeriod {
	return ClosePeriod{period}
}

func (u ClosePeriod) Code() string {
	return ClosePeriodOperation
}

type PeriodReport struct {
	Period Period
}

func NewPeriodReport(period Period) PeriodReport {
	return PeriodReport{period}
}

func (u PeriodReport) Code() string {
	return PeriodReportOperation
}

type YearReport struct {
	Period Period
}

func NewYearReport(period Period) YearReport {
	return YearReport{period}
}

func (u YearReport) Code() string {
	return YearReportOperation
}
