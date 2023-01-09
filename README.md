<div align="center">
    <h3 align="center">Rusty Exchange</h3>
    <p align="center">
        A workspace for a financial exchange architecture written entirely in Rust.
    </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#overview">Overview</a></li>
        <ol>
            <li><a href="#crates">Crates</a></li>
            <li><a href="#stack">Stack</a></li>
        </ol>
        <li><a href="#usage">Usage</a></li>
        <ol>
            <li><a href="#docker">Docker</a></li>
        </ol>
        <li><a href="#contribution">Contribution</a></li>
        <li><a href="#todo">To-do</a></li>
        <li><a href="#whitepapers">Whitepapers</a></li>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview
Inspired by [CoinTossX](https://github.com/dharmeshsing/CoinTossX), this repo aims to create a high-throughput,
low-latency fullstack matching engine written entirely in Rust.

<!-- CRATES -->
## Crates
* [API](api) ([README.md](api/README.md))
* [Database](database) ([README.md](database/README.md))

<!-- STACK -->
## Stack
### Services
* Postgres
* RabbitMQ
* Docker

### Libraries
* SeaOrm
* Actix-Web
* Utoipa
* RabbitMQ Stream Client

### Microservices
* API
  * Retrieve data for clients and frontend
  * Handle requests for exchange information
  * Serve market data to websocket channels
  * Submit new/amended/canceled orders to the matching engine
* Matching engine
  * Listen for incoming orders via RabbitMQ
  * Process new limit orders and store them in a limit order book
  * Match market orders to existing limit orders
  * Publish fills to the database and the API websocket via RabbitMQ

<!-- USAGE -->
# Usage
<!-- DOCKER -->
## Docker
### Quick Start
* Clone or download this repository
* Go inside of directory,  `cd fullstack-rs`
* Run this command `docker-compose up -d`

### Environments
This docker-compose file contains the following environment variables:

* `POSTGRES_USER` the default value is **postgres**
* `POSTGRES_PASSWORD` the default value is **Boomers4life!123**
* `PGADMIN_PORT` the default value is **5050**
* `PGADMIN_DEFAULT_EMAIL` the default value is **pgadmin4@pgadmin.org**
* `PGADMIN_DEFAULT_PASSWORD` the default value is **admin**

### Access to postgres:
* `localhost:5432`
* **Username:** postgres (as a default)
* **Password:** Boomers4life!123 (as a default)

### Access to PgAdmin:
* **URL:** `http://localhost:5050`
* **Username:** pgadmin4@pgadmin.org (as a default)
* **Password:** admin (as a default)

### Add a new server in PgAdmin:
* **Host name/address** `postgres`
* **Port** `5432`
* **Username** as `POSTGRES_USER`, by default: `postgres`
* **Password** as `POSTGRES_PASSWORD`, by default `Boomers4life!123`

<!-- CONTRIBUTION -->
# Contribution
This project uses pre-commit hooks and commitizen to standardize commit messages and code styles.
To use the configurations, the first time you clone the repository, install the pre-commit hooks with
`pre-commit install`. If you do not have commitizen installed you can install it with `brew install commitizen`.

To commit and apply the styles, run `cz c` in the terminal. To bump the semantic version, run `cz bump`.
To generate a new change-log, run cz changelog. This will generate new entries in CHANGELOG.md.

One can apply these code style changes directly by running `cargo run clippy --fix` (set as a run configuration for
IntelliJ).

<!-- TODO -->
# To-do
* Create github workflows
* Create dockerfiles for each service

<!-- WHITEPAPERS -->
# Whitepapers

<!-- RESOURCES -->
# Additional Resources
