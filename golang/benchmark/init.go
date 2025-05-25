package benchmark

import (
	"bench-pg-go/model/domain"
	"cloud.google.com/go/civil"
	"fmt"
	"github.com/shopspring/decimal"
)

func initPurchases(year int, vendors int, materials int) []domain.Purchase {
	res := make([]domain.Purchase, 0)
	for i := 0; i < materials; i++ {
		material := domain.NewMaterial(i)
		vendor := domain.NewVendor(randGen.Intn(vendors))
		price := decimal.NewFromInt(int64(randGen.Intn(100) + 100))
		quantity := decimal.NewFromInt(1000 * int64(randGen.Intn(100)+100))
		res = append(res,
			domain.Purchase{
				Material: material,
				Vendor:   vendor,
				Price:    price,
				Quantity: quantity,
				PostingDate: civil.Date{
					Year:  year - 1,
					Month: 12,
					Day:   31,
				},
				DocNo: fmt.Sprintf("INIT-%08d", i),
			})
	}
	return res
}
