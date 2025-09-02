package statistics

import (
	config2 "bench-pg-go/config"
	"bench-pg-go/model/metrics"
	"bench-pg-go/statistics/model"
	"fmt"
	"log"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

func Run(config *config2.Config, metrics []metrics.DomainMetric) {
	connectionString := fmt.Sprintf(
		"host=%s user=%s password=%s dbname=%s port=%d sslmode=disable",
		config.Host,
		config.Username,
		config.Password,
		config.Db,
		config.Port,
	)
	db, err := gorm.Open(postgres.Open(connectionString), &gorm.Config{
		Logger:      logger.Default.LogMode(logger.Silent),
		PrepareStmt: false,
	})
	if err != nil {
		panic(err)
	}
	sqlDB, err := db.DB()
	if err != nil {
		panic(err)
	}
	sqlDB.SetMaxOpenConns(config.Connections)

	benchmarkId := createBenchmark(db, config)
	saveMetrics(db, benchmarkId, metrics)
	saveStatistics(db, benchmarkId, metrics, config.StartYear, config.Years)

}

func createBenchmark(db *gorm.DB, config *config2.Config) int64 {
	entity := model.BenchmarkEntity{
		Id:         0,
		Name:       config.Name,
		Date:       time.Time{},
		DbLib:      config.Lib,
		Customers:  config.Customers,
		Vendors:    config.Vendors,
		Materials:  config.Materials,
		Users:      config.Users,
		StartYear:  config.StartYear,
		Years:      config.Years,
		Operations: config.Operations,
	}

	res := db.Create(&entity)
	if err := res.Error; err != nil {
		log.Fatalf("Error inserting benchmark: %v", err)
	}

	return entity.Id
}
