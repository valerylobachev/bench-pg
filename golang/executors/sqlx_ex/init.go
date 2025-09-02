package sqlx_ex

import (
	"bench-pg-go/executors/sqlx_ex/model"
	"bench-pg-go/model/domain"
	"github.com/jmoiron/sqlx"
	"github.com/shopspring/decimal"
	"time"
)

func clearTables(db *sqlx.DB) {
	statements := []string{
		"truncate table fin_ledger_items cascade;",
		"truncate table fin_accounts cascade;",
		"truncate table fin_material_periods cascade;",
		"truncate table fin_materials cascade;",
		"truncate table fin_business_partners cascade;",
	}

	for _, statement := range statements {
		db.MustExec(statement)
	}
}

func loadBusinessPartners(customers int, vendors int, db *sqlx.DB) {
	user := domain.NewUser(0)

	for i := 0; i < customers; i++ {
		customer := domain.NewCustomer(i)
		entity := model.BusinessPartnerEntity{
			Id:        customer.Id(),
			Name:      customer.Name(),
			UpdatedBy: user.Id(),
			UpdatedAt: time.Now(),
		}
		_, err := db.NamedExec(
			`INSERT INTO fin_business_partners (id, name, updated_by, updated_at) 
                    VALUES (:id, :name, :updated_by, :updated_at)`,
			entity,
		)
		if err != nil {
			panic(err)
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
		_, err := db.NamedExec(
			`INSERT INTO fin_business_partners (id, name, updated_by, updated_at) 
                    VALUES (:id, :name, :updated_by, :updated_at)`,
			entity,
		)
		if err != nil {
			panic(err)
		}
	}
}

func loadMaterials(materials int, startYear int, db *sqlx.DB) {
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
		_, err := db.NamedExec(
			`insert into fin_materials (id, name, next_std_price, updated_by, updated_at)
                 values (:id, :name, :next_std_price, :updated_by, :updated_at);`,
			materialEntity,
		)
		if err != nil {
			panic(err)
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
		_, err = db.NamedExec(
			`insert into fin_material_periods
                 	 (material_id, period, std_price, mov_avg_price,
                      actual_price, sell_price, stock, updated_by, updated_at)
                    values (:material_id, :period, :std_price, :mov_avg_price,
                            :actual_price, :sell_price, :stock, :updated_by, :updated_at);`,
			materialPeriodEntity,
		)
		if err != nil {
			panic(err)
		}
	}
}

func loadAccounts(db *sqlx.DB, accounts []domain.Account) {
	user := domain.NewUser(0)
	userId := user.Id()

	for _, account := range accounts {
		accountEntity := model.AccountEntity{
			Id:        account.Id,
			Name:      account.Name,
			UpdatedBy: userId,
			UpdatedAt: time.Now(),
		}
		_, err := db.NamedExec(
			`insert into fin_accounts (id, name, updated_by, updated_at) 
                     values (:id, :name, :updated_by, :updated_at);`,
			accountEntity,
		)
		if err != nil {
			panic(err)
		}
	}
}
