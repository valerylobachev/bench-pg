package config

import (
	"fmt"
	"strconv"
	"strings"

	flag "github.com/spf13/pflag"
)

type Config struct {
	Username    string
	Password    string
	Host        string
	Port        int
	Connections int
	Db          string
	Lib         string
	Customers   int
	Vendors     int
	Materials   int
	Users       int
	StartYear   int
	Years       int
	Operations  int
	Name        string
}

func (c Config) String() string {
	lines := []string{
		c.Name,
		"  Username: " + c.Username,
		"  Password: " + c.Password,
		"  Host: " + c.Host,
		"  Port: " + strconv.Itoa(c.Port),
		"  Connections: " + strconv.Itoa(c.Connections),
		"  Db: " + c.Db,
		"  Lib: " + c.Lib,
		"  Customers: " + strconv.Itoa(c.Customers),
		"  Vendors: " + strconv.Itoa(c.Vendors),
		"  Materials: " + strconv.Itoa(c.Materials),
		"  Users: " + strconv.Itoa(c.Users),
		"  StartYear: " + strconv.Itoa(c.StartYear),
		"  Years: " + strconv.Itoa(c.Years),
		"  Operations: " + strconv.Itoa(c.Operations),
	}
	return strings.Join(lines, "\n")
}

func NewConfig() *Config {
	config := Config{}
	flag.StringVar(&config.Username, "username", "postgres", "database username")
	flag.StringVar(&config.Password, "password", "postgres", "database password")
	flag.StringVar(&config.Host, "host", "localhost", "database host")
	flag.IntVar(&config.Port, "port", 5432, "database port")
	flag.StringVar(&config.Db, "db", "benchmark", "database name")
	flag.IntVar(&config.Connections, "connections", 20, "max db connections")
	//flag.StringVar(&config.Lib, "lib", "gorm", "database library")
	flag.StringVar(&config.Lib, "lib", "go-sqlx", "database library")
	flag.IntVar(&config.Customers, "customers", 100, "total customers")
	flag.IntVar(&config.Vendors, "vendors", 100, "total vendors")
	flag.IntVar(&config.Materials, "materials", 100, "total materials")
	flag.IntVar(&config.Users, "users", 12, "total users")
	flag.IntVar(&config.StartYear, "start-year", 2025, "start year")
	flag.IntVar(&config.Years, "years", 1, "years")
	flag.IntVar(&config.Operations, "operations", 20000, "total operations")
	flag.StringVar(&config.Name, "name", "", "benchmark name")

	if config.Name == "" {
		config.Name = fmt.Sprintf(
			"Benchmark %s with %d users, %d operations, %d materials, %d years",
			config.Lib, config.Users, config.Operations, config.Materials, config.Years,
		)
	}

	flag.Parse()

	return &config
}
