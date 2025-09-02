package sqlx_ex

import (
	"bench-pg-go/api"
	"bench-pg-go/model/domain"
	"fmt"
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
	"math/rand"
	"time"
)

var randGen = rand.New(rand.NewSource(time.Now().UnixNano()))

type SqlxExecutor struct {
	db *sqlx.DB
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

	db, err := sqlx.Connect("postgres", connectionString)
	if err != nil {
		panic(err)
	}

	db.SetMaxOpenConns(connections)

	return &SqlxExecutor{db}
}

func (g SqlxExecutor) Init(startYear int, customers int, vendors int, materials int, accounts []domain.Account, purchases []domain.Purchase) {
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

func (g SqlxExecutor) PurchaseMaterial(operation *domain.Purchase, user domain.User) {
	purchaseMaterial(g.db, operation, user)
}

func (g SqlxExecutor) SellMaterial(operation *domain.Sale, user domain.User) {
	sellMaterial(g.db, operation, user)
}

func (g SqlxExecutor) AccountCost(operation *domain.Cost, user domain.User) {
	accountCost(g.db, operation, user)
}

func (g SqlxExecutor) OpenPeriod(period domain.Period, user domain.User) {
	openPeriod(g.db, period.NextPeriod(), user)
}

func (g SqlxExecutor) ClosePeriod(period domain.Period, user domain.User) {
	closePeriod(g.db, period.PrevPeriod(), user)
}

func (g SqlxExecutor) PeriodReport(period domain.Period) {
	report(g.db, period, period)
}

func (g SqlxExecutor) YearReport(period domain.Period) {
	report(g.db, period.FirstPeriod(), period.LastPeriod())
}
