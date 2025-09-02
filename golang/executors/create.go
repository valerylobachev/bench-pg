package executors

import (
	"bench-pg-go/api"
	"bench-pg-go/executors/gorm_ex"
	"bench-pg-go/executors/sqlx_ex"
)

func CreateExecutor(
	username string,
	password string,
	host string,
	port int,
	db string,
	connections int,
	lib string,
) api.ExecutorApi {

	if lib == "gorm" {
		return gorm_ex.NewExecutor(
			username,
			password,
			host,
			port,
			db,
			connections,
		)
	} else if lib == "go-sqlx" {
		return sqlx_ex.NewExecutor(
			username,
			password,
			host,
			port,
			db,
			connections,
		)
	}

	panic("invalid lib: " + lib)

}
