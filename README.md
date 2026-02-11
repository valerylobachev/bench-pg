# bench-pg

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

**bench-pg** is a cross‑language PostgreSQL benchmarking suite designed to measure and compare the performance of
different database driver libraries under a realistic, business‑like workload. It provides two independent
implementations:

- **Rust** – `bench-pg-rs` (libraries: `tokio-postgres`, `sqlx`, `diesel`)
- **Go** – `bench-pg-go` (libraries: `gorm`, `go-sqlx`)

Both tools generate an identical dataset (customers, vendors, materials, users) and execute a configurable number of
transactional operations over a simulated time period. This allows you to directly compare throughput and latency across
languages and database abstraction layers under exactly the same workload.

---

## Features

- **Multi‑language, multi‑library** – Compare Rust vs. Go and different ORMs/drivers.
- **Configurable dataset size** – Control the number of customers, vendors, materials, and users.
- **Adjustable workload** – Set total operations, time span (years), and connection pool size.
- **Automated schema management** – Tables are dropped and recreated on each run.
- **Consistent CLI** – Nearly identical command‑line options across both implementations.
- **Descriptive benchmark naming** – Auto‑generated or user‑supplied names for easy identification.

---

## Requirements

- **PostgreSQL** 12+ (any recent version)
- **Rust** (edition 2021) – only for `bench-pg-rs`
- **Go** 1.20+ – only for `bench-pg-go`
- A PostgreSQL database with a user that has full read/write and schema modification privileges.

---

## Repository Structure

```
bench-pg/
├── golang/             # Go implementation (bench-pg-go)
│   ├── go.mod
│   ├── main.go
│   └── ...
├── rust/               # Rust implementation (bench-pg-rs)
│   ├── Cargo.toml
│   └── src/
├── sql/               
│   └── ddl.sql         # Database schema
└── README.md           # This file
```

---

## Database 

Database should be created and schema must be initialized. Schema file is located in  [sql/ddl.sql](sql/ddl.sql)

---

## Rust Application (`bench-pg-rs`)

The Rust benchmark is located in the `rust/` directory. It supports three database libraries via the `--lib` flag.

### Building

```bash
cd rust
cargo build --release
```

The executable will be available at `./target/release/bench-pg-rs`.

### Command Line Options

| Short | Long            | Description                           | Default     | Possible Values           |
|-------|-----------------|---------------------------------------|-------------|---------------------------|
| `-U`  | `--username`    | PostgreSQL username                   | `postgres`  |                           |
| `-P`  | `--password`    | PostgreSQL password                   | `postgres`  |                           |
| `-H`  | `--host`        | PostgreSQL host                       | `localhost` |                           |
| `-p`  | `--port`        | PostgreSQL port                       | `5432`      |                           |
| `-C`  | `--connections` | Size of the connection pool           | `20`        |                           |
| `-d`  | `--db`          | Database name                         | `benchmark` |                           |
| `-l`  | `--lib`         | Database library to use               | `tokio`     | `tokio`, `sqlx`, `diesel` |
| `-c`  | `--customers`   | Number of customers to generate       | `100`       |                           |
| `-v`  | `--vendors`     | Number of vendors to generate         | `100`       |                           |
| `-m`  | `--materials`   | Number of materials to generate       | `100`       |                           |
| `-u`  | `--users`       | Number of system users                | `12`        |                           |
| `-s`  | `--start-year`  | Start year for simulated operations   | `2025`      |                           |
| `-y`  | `--years`       | Number of years of activity           | `1`         |                           |
| `-o`  | `--operations`  | Total number of operations to execute | `20000`     |                           |
| `-n`  | `--name`        | Custom name for the benchmark run     | (auto‑gen)  |                           |
| `-h`  | `--help`        | Print help                            |             |                           |

### Example

```bash
./target/release/bench-pg-rs -l sqlx -C 50 -o 50000 -n "sqlx-50conn-50kops"
```

---

## Go Application (`bench-pg-go`)

The Go benchmark is located in the `golang/` directory. It supports two database libraries via the `--lib` flag.

### Building

```bash
cd golang
go build -o bench-pg-go .
```

The executable will be created as `./bench-pg-go`.

### Command Line Options

All options are passed as long flags (e.g., `--host localhost`).

| Flag            | Description                           | Default     | Possible Values   |
|-----------------|---------------------------------------|-------------|-------------------|
| `--username`    | PostgreSQL username                   | `postgres`  |                   |
| `--password`    | PostgreSQL password                   | `postgres`  |                   |
| `--host`        | PostgreSQL host                       | `localhost` |                   |
| `--port`        | PostgreSQL port                       | `5432`      |                   |
| `--connections` | Size of the connection pool           | `20`        |                   |
| `--db`          | Database name                         | `benchmark` |                   |
| `--lib`         | Database library to use               | `go-sqlx`   | `gorm`, `go-sqlx` |
| `--customers`   | Number of customers to generate       | `100`       |                   |
| `--vendors`     | Number of vendors to generate         | `100`       |                   |
| `--materials`   | Number of materials to generate       | `100`       |                   |
| `--users`       | Number of system users                | `12`        |                   |
| `--start-year`  | Start year for simulated operations   | `2025`      |                   |
| `--years`       | Number of years of activity           | `1`         |                   |
| `--operations`  | Total number of operations to execute | `20000`     |                   |
| `--name`        | Custom name for the benchmark run     | (auto‑gen)  |                   |

### Example

```bash
./bench-pg-go --lib gorm --connections 30 --operations 10000 --name "gorm-30conn-10kops"
```

---

## Workload Description

Both benchmarks follow an identical procedure to ensure fair comparison:

1. **Schema initialisation** – Database and schema should be created before running benchmark.

2. **Data generation** – Dimension tables are populated with the specified number of rows using random but realistic
   data.

3. **Execution** – A fixed mix of business transactions (e.g., order placement, invoice generation, stock movements) is
   executed concurrently across the connection pool. Each transaction consists of multiple SQL statements and touches
   both dimension and fact tables.

---

## License

Copyright 2025 bench-pg contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

## Contributing

Contributions are welcome! If you'd like to add support for another database library, improve the workload realism, or
fix bugs, please open an issue or submit a pull request. When adding a new library, ensure that the workload logic
remains identical to the existing implementations so that results remain comparable.

---

*Happy benchmarking!*
