package gorm_ex

import (
	"bench-pg-go/api"
	"bench-pg-go/model/domain"
	"fmt"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"math/rand"
	"time"
)

var randGen = rand.New(rand.NewSource(time.Now().UnixNano()))

type GormExecutor struct {
	db *gorm.DB
}

func NewExecutor(
	username string,
	password string,
	host string,
	port int,
	database string,
	connections int,
) api.ExecutorApi {

	connectionString := fmt.Sprintf(
		"host=%s user=%s password=%s dbname=%s port=%d sslmode=disable",
		host,
		username,
		password,
		database,
		port,
	)

	db, err := gorm.Open(postgres.Open(connectionString), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	sqlDB, err := db.DB()
	if err != nil {
		panic(err)
	}
	sqlDB.SetMaxOpenConns(connections)
	return &GormExecutor{db}
}

func (g GormExecutor) Init(startYear int, customers int, vendors int, materials int, accounts []domain.Account, purchases []domain.Purchase) {
	user := domain.NewUser(0)
	clearTables(g.db)
	loadBusinessPartners(customers, vendors, g.db)
	loadMaterials(materials, startYear, g.db)
	loadAccounts(g.db, accounts)
	for _, purchase := range purchases {
		purchaseMaterial(g.db, &purchase, user)
	}
	openPeriod(g.db, domain.NewPeriod(startYear, 1), user)
}

func (g GormExecutor) PurchaseMaterial(operation *domain.Purchase, user domain.User) {
	purchaseMaterial(g.db, operation, user)
}

func (g GormExecutor) SellMaterial(operation *domain.Sale, user domain.User) {
	//TODO implement me
	//panic("implement me")
}

func (g GormExecutor) AccountCost(operation *domain.Cost, user domain.User) {
	//TODO implement me
	//panic("implement me")
}

func (g GormExecutor) OpenPeriod(period domain.Period, user domain.User) {
	//TODO implement me
	//panic("implement me")
	openPeriod(g.db, period.NextPeriod(), user)
}

func (g GormExecutor) ClosePeriod(period domain.Period, user domain.User) {
	//TODO implement me
	//panic("implement me")
}

func (g GormExecutor) PeriodReport(period domain.Period) {
	//TODO implement me
	//panic("implement me")
}

func (g GormExecutor) YearReport(period domain.Period) {
	//TODO implement me
	//panic("implement me")
}
