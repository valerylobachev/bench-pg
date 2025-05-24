package gorm_ex

import (
	"bench-pg-go/executors/gorm_ex/model"
	"bench-pg-go/model/domain"
	"github.com/shopspring/decimal"
	"gorm.io/gorm"
	"time"
)

func clearTables(db *gorm.DB) {
	statements := []string{
		"truncate table fin_ledger_items cascade;",
		"truncate table fin_accounts cascade;",
		"truncate table fin_material_periods cascade;",
		"truncate table fin_materials cascade;",
		"truncate table fin_business_partners cascade;",
	}

	for _, statement := range statements {
		res := db.Exec(statement)
		if res.Error != nil {
			panic(res.Error)
		}
	}
}

func loadBusinessPartners(customers int, vendors int, db *gorm.DB) {
	user := domain.NewUser(0)

	for i := 0; i < customers; i++ {
		customer := domain.NewCustomer(i)
		entity := model.BusinessPartnerEntity{
			Id:        customer.Id(),
			Name:      customer.Name(),
			UpdatedBy: user.Id(),
			UpdatedAt: time.Now(),
		}
		res := db.Create(&entity)
		if res.Error != nil {
			panic(res.Error)
		}
	}

	for i := 0; i < vendors; i++ {
		vendor := domain.NewVendor(i)
		entity := model.BusinessPartnerEntity{
			Id:        vendor.Id(),
			Name:      vendor.Name(),
			UpdatedBy: user.Id(),
			UpdatedAt: time.Now(),
		}
		res := db.Create(&entity)
		if res.Error != nil {
			panic(res.Error)
		}
	}
}

func loadMaterials(materials int, startYear int, db *gorm.DB) {
	user := domain.NewUser(0)
	userId := user.Id()
	periodYp := domain.NewPeriod(startYear, 1).PrevPeriod().YearPeriod()

	for i := 0; i < materials; i++ {
		material := domain.NewMaterial(i)
		materialId := material.Id()
		stdPrice := decimal.NewFromInt(int64(randGen.Intn(100) + 100))
		materialEntity := model.MaterialEntity{
			Id:           materialId,
			Name:         material.Name(),
			NextStdPrice: stdPrice,
			UpdatedBy:    userId,
			UpdatedAt:    time.Now(),
		}
		res := db.Create(&materialEntity)
		if res.Error != nil {
			panic(res.Error)
		}

		sellPrice := stdPrice.Mul(decimal.NewFromInt(2))
		materialPeriodEntity := model.MaterialPeriodEntity{
			MaterialId:  materialId,
			Period:      periodYp,
			StdPrice:    stdPrice,
			MovAvgPrice: stdPrice,
			ActualPrice: stdPrice,
			SellPrice:   sellPrice,
			Stock:       decimal.NewFromInt(0),
			UpdatedBy:   userId,
			UpdatedAt:   time.Now(),
		}
		res = db.Create(&materialPeriodEntity)
		if res.Error != nil {
			panic(res.Error)
		}
	}
}

func loadAccounts(db *gorm.DB, accounts []domain.Account) {
	user := domain.NewUser(0)
	userId := user.Id()

	for _, account := range accounts {
		accountEntity := model.AccountEntity{
			Id:        account.Id,
			Name:      account.Name,
			UpdatedBy: userId,
			UpdatedAt: time.Now(),
		}
		res := db.Create(&accountEntity)
		if res.Error != nil {
			panic(res.Error)
		}
	}
}
