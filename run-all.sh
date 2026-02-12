#!/usr/bin/env bash

./golang/bench-pg-go              --operations 10000 --lib go-sqlx --years 3
./golang/bench-pg-go              --operations 10000 --lib gorm    --years 3
./rust/target/release/bench-pg-rs --operations 10000 --lib tokio   --years 3
./rust/target/release/bench-pg-rs --operations 10000 --lib diesel  --years 3
./rust/target/release/bench-pg-rs --operations 10000 --lib sqlx    --years 3