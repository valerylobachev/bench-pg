package main

import (
	"bench-pg-go/benchmark"
	config2 "bench-pg-go/config"
	"bench-pg-go/executors"
	"bench-pg-go/statistics"
	"fmt"
	"runtime"
)

func main() {
	runtime.GOMAXPROCS(25)

	config := config2.NewConfig()

	fmt.Println(config)

	executor := executors.CreateExecutor(
		config.Username,
		config.Password,
		config.Host,
		config.Port,
		config.Db,
		config.Connections,
		config.Lib,
	)

	metrics := benchmark.Run(config, executor)

	statistics.Run(config, metrics)

}
