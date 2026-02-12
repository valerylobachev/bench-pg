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

<!-- TOC -->

* [Features](#features)
* [Requirements](#requirements)
* [Repository Structure](#repository-structure)
* [Database](#database)
* [Rust Application (`bench-pg-rs`)](#rust-application-bench-pg-rs)
    * [Building](#building)
    * [Command Line Options](#command-line-options)
    * [Example](#example)
* [Go Application (`bench-pg-go`)](#go-application-bench-pg-go)
    * [Building](#building-1)
    * [Command Line Options](#command-line-options-1)
    * [Example](#example-1)
* [Workload Description](#workload-description)
* [Benchmark Results](#benchmark-results)
    * [Observations](#observations)
* [Code Size](#code-size)
    * [Observations](#observations-1)
* [License](#license)
* [Contributing](#contributing)

<!-- TOC -->

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
│   ├── src/
│   └── ...
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
| `-C`  | `--connections` | Size of the connection pool           | `40`        |                           |
| `-d`  | `--db`          | Database name                         | `benchmark` |                           |
| `-l`  | `--lib`         | Database library to use               | `tokio`     | `tokio`, `sqlx`, `diesel` |
| `-c`  | `--customers`   | Number of customers to generate       | `100`       |                           |
| `-v`  | `--vendors`     | Number of vendors to generate         | `100`       |                           |
| `-m`  | `--materials`   | Number of materials to generate       | `100`       |                           |
| `-u`  | `--users`       | Number of system users                | `40`        |                           |
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
| `--connections` | Size of the connection pool           | `40`        |                   |
| `--db`          | Database name                         | `benchmark` |                   |
| `--lib`         | Database library to use               | `go-sqlx`   | `gorm`, `go-sqlx` |
| `--customers`   | Number of customers to generate       | `100`       |                   |
| `--vendors`     | Number of vendors to generate         | `100`       |                   |
| `--materials`   | Number of materials to generate       | `100`       |                   |
| `--users`       | Number of system users                | `40`        |                   |
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

1. **Schema initialization** – Database and schema should be created before running benchmark.

2. **Data generation** – Dimension tables are populated with the specified number of rows using random but realistic
   data.

3. **Execution** – A fixed mix of business transactions (e.g., order placement, invoice generation, stock movements) is
   executed concurrently across the connection pool. Each transaction consists of multiple SQL statements and touches
   both dimension and fact tables.

---

## Benchmark Results

To demonstrate the kind of performance data `bench-pg` produces, we ran all five supported database libraries under
identical conditions.  
The test was executed on a MacBook Pro (Apple M2 Max 12 vCPUs, 96 GB RAM, SSD storage) with the following workload
parameters:

| Option      | Value  |
|-------------|--------|
| Connections | 40     |
| Users       | 40     |
| Customers   | 100    |
| Vendors     | 100    |
| Materials   | 100    |
| Start Year  | 2025   |
| Years       | 3      |
| Operations  | 10,000 |

Each library was given a connection pool of 40 connections (the `--connections` setting) and ran the full mix of
business transactions concurrently. The table below shows the total execution time and detailed latency percentiles for
each library.

| db_lib              | total_duration (s) | min (s) | p50 (s) | p75 (s) | p95 (s) | p99 (s) | p99.9 (s) | max (s) | mean (s) | std_dev (s) |
|---------------------|-------------------:|--------:|--------:|--------:|--------:|--------:|----------:|--------:|---------:|------------:|
| Go sqlx             |                760 |    3.01 |   20.68 |   29.51 |   38.31 |   39.49 |     39.49 |   39.49 |    21.10 |       10.65 |
| Go GORM             |                788 |    2.67 |   21.89 |   29.19 |   41.19 |   43.17 |     43.17 |   43.17 |    21.90 |       10.90 |
| Rust tokio-postgres |                848 |    3.04 |   23.57 |   32.25 |   40.94 |   42.55 |     42.95 |   43.00 |    23.56 |       11.76 |
| Rust Diesel         |                885 |    3.55 |   25.21 |   32.95 |   41.64 |   42.96 |     43.27 |   43.31 |    24.59 |       11.36 |
| Rust sqlx           |                896 |    3.30 |   25.79 |   35.16 |   42.07 |   44.79 |     45.61 |   45.70 |    24.90 |       12.20 |

*Lower `total_duration` and lower latency percentiles are better.*

![Benchmark Chart](chart.png)

### Observations

- **Go implementations** outperformed all Rust libraries in this test.  
  `go-sqlx` was the fastest overall, completing the workload in **760 seconds** with the lowest p50 (20.68 ms) and p95 (
  38.31 ms) latencies.
- **GORM**, despite being a full‑featured ORM, finished only 3.7% slower than `go-sqlx` and still beat every Rust
  library.
- Among Rust libraries, **tokio-postgres** was the quickest, while **sqlx** (which uses asynchronous Rust) showed
  slightly higher latencies and total runtime.  
  This may reflect differences in connection pooling behavior and the overhead of the async runtime under this specific
  workload.
- **Diesel** – a synchronous, type‑safe ORM – performed similarly to `sqlx`, with marginally better p95 and p99 values.
- All libraries exhibited stable performance, with standard deviations around 10–12 ms and tail latencies (p99.9)
  remaining close to their p99 values.

These results illustrate the kind of comparative data you can obtain with `bench-pg`.  
Remember that real‑world performance depends on many factors – hardware, PostgreSQL configuration, network latency, and
the exact query mix. We encourage you to run your own benchmarks using settings that match your production environment.

---

## Code Size

One of the project goals is to keep the benchmark harness **simple and auditable**, while the library‑specific
implementations illustrate how different database drivers and ORMs express the same workload. The table below shows the
lines of code (excluding blank lines and comments) for each component.

| Component                          | Lines of code |
|------------------------------------|--------------:|
| Go benchmark harness               |           291 |
| Rust benchmark harness             |           295 |
| Rust sqlx implementation           |           695 |
| Rust tokio-postgres implementation |           756 |
| Go GORM implementation             |           799 |
| Go sqlx implementation             |           872 |
| Rust diesel implementation         |           880 |

### Observations

- The **benchmark harnesses** are extremely compact – **~300 lines** in both languages. This makes it easy to verify the
  workload logic and configuration handling.
- **Go sqlx** (872 lines) and **GORM** (799 lines) are the largest Go implementations. The difference is modest,
  suggesting that both libraries require similar amounts of boilerplate for this benchmark.
- Among Rust libraries, **sqlx** is the most concise (**695 lines**), thanks to its built‑in connection pooling and
  async query macros.  
  **tokio-postgres** (756 lines) is slightly larger, reflecting manual pool setup.  
  **Diesel** (880 lines) requires schema definitions and explicit model structs, which increases the line count.
- Overall, the **Go implementations are comparable in size** to the Rust ones, with sqlx and tokio-postgres falling in
  between the two Go drivers.

These figures give a rough indication of development and maintenance effort. However, lines of code should not be
over‑interpreted – they do not account for the complexity of the libraries themselves or the expressiveness of each
language. They do show that **all implementations are reasonably sized** and easy to understand.

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
