package statistics

import (
	"bench-pg-go/model/domain"
	"bench-pg-go/model/metrics"
	"bench-pg-go/statistics/model"
	"log"

	"github.com/shopspring/decimal"
	"gorm.io/gorm"
)

func saveMetrics(db *gorm.DB, id int64, domainMetrics []metrics.DomainMetric) {
	entites := make([]model.MetricEntity, len(domainMetrics))
	for idx, metric := range domainMetrics {
		var materialId *string
		var businessPartnerId *string
		var quantity *decimal.Decimal
		var price *decimal.Decimal
		var amount *decimal.Decimal
		var docNo *string
		var saleDocNo *string
		var cogsDocNo *string
		var postingDate *string
		var operation, ok = metric.Action.(metrics.Operation)
		if metric.Action != nil && ok {
			switch op := operation.Operation.(type) {
			case domain.Purchase:
				m := op.Material.Id()
				materialId = &m
				b := op.Vendor.Id()
				businessPartnerId = &b
				quantity = &op.Quantity
				price = &op.Price
				docNo = &op.DocNo
				d := op.PostingDate.String()
				postingDate = &d
			case domain.Sale:
				m := op.Material.Id()
				materialId = &m
				b := op.Customer.Id()
				businessPartnerId = &b
				quantity = &op.Quantity
				saleDocNo = &op.SaleDocNo
				cogsDocNo = &op.CogsDocNo
				d := op.PostingDate.String()
				postingDate = &d
			case domain.Cost:
				m := op.Material.Id()
				materialId = &m
				b := op.Vendor.Id()
				businessPartnerId = &b
				amount = &op.Amount
				docNo = &op.DocNo
				d := op.PostingDate.String()
				postingDate = &d
			}

		}
		entites[idx] = model.MetricEntity{
			Id:                0,
			BenchmarkId:       id,
			Year:              metric.Year,
			Period:            metric.Period,
			Index:             int64(metric.Index),
			UserNo:            metric.UserNo,
			Action:            metric.Action.Type(),
			MaterialId:        materialId,
			BusinessPartnerId: businessPartnerId,
			Quantity:          quantity,
			Price:             price,
			Amount:            amount,
			DocNo:             docNo,
			SaleDocNo:         saleDocNo,
			CogsDocNo:         cogsDocNo,
			PostingDate:       postingDate,
			Duration:          metric.Duration,
		}
	}
	res := db.CreateInBatches(entites, 100)
	if err := res.Error; err != nil {
		log.Fatalf("Error inserting metric: %v", err)
	}
}
