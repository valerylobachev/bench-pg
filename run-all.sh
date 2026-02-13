#!/usr/bin/env bash

./golang/bench-pg-go              --operations 15000 --lib go-sqlx --years 1 --users 30
./golang/bench-pg-go              --operations 15000 --lib gorm    --years 1 --users 30
./rust/target/release/bench-pg-rs --operations 15000 --lib tokio   --years 1 --users 30
./rust/target/release/bench-pg-rs --operations 15000 --lib diesel  --years 1 --users 30
./rust/target/release/bench-pg-rs --operations 15000 --lib sqlx    --years 1 --users 30