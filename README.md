<div align="center">
    <h3 align="center">Rusty Exchange</h3>
    <p align="center">
        A workspace for a financial exchange architecture written entirely in Rust.
    </p>

[![Test](https://github.com/IvanJericevich/rusty-exchange/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/IvanJericevich/rusty-exchange/actions/workflows/test.yml)
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
        <li><a href="#whitepapers">Whitepapers</a></li>
        <li><a href="#todo">To-do</a></li>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview
Inspired by [CoinTossX](https://github.com/dharmeshsing/CoinTossX), this repo aims to create a high-throughput,
low-latency fullstack matching engine written entirely in Rust.

<!-- CRATES -->
## Crates
* [API](api) ([README](api/README.md))
* [Database](database) ([README](database/README.md))
* [Order-book](orderbook) ([README](orderbook/README.md))

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
One can start all services either through docker-compose or by running the individual binaries in development.
<!-- DOCKER -->
## Docker
### Quick Start
* Clone or download this repository
* Go inside of directory, `cd rusty-exchange`
* Run this command `docker-compose up -d`

### Environments
This docker-compose file contains the following environment variables:
* `POSTGRES_USER` the default value is **postgres**
* `POSTGRES_PASSWORD` the default value is **postgres**
* `POSTGRES_DB` the default value is **Exchange**
* `PGADMIN_PORT` the default value is **5050**
* `PGADMIN_DEFAULT_EMAIL` the default value is **pgadmin4@pgadmin.org**
* `PGADMIN_DEFAULT_PASSWORD` the default value is **admin**

### Postgres:
One can access the postgres server using the following credentials:
* **Host** If accessing from localhost then `localhost:5432` else `host.docker.internal`
* **Username:** postgres (as a default)
* **Password:** postgres (as a default)

### PgAdmin:
To access PgAdmin in your browser, enter the following URL and credentials in your browser:
* **URL:** `http://localhost:5050`
* **Username:** pgadmin4@pgadmin.org (as a default)
* **Password:** admin (as a default)
* 
Thereafter, add a new server in PgAdmin:
* **Host name/address** If accessing from localhost then `localhost:5432` else `host.docker.internal`
* **Port** `5432`
* **Username** as `POSTGRES_USER`, by default: `postgres`
* **Password** as `POSTGRES_PASSWORD`, by default `postgres`

### RabbitMQ
Begin by starting a RabbitMQ server either from the docker-compose.yaml (also defined in the run configurations) or
from the command line:
```
docker run -it --rm --name rabbitmq -p 5552:5552 \
  -e RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS='-rabbitmq_stream advertised_host localhost' \
  rabbitmq:3.9
```

In order to user the Rust stream client one has to start the RabbitMQ server with the correct plugins:
* `rabbitmq_stream` to enable streaming.
* `rabbitmq_management` for creating a management UI on the host browser.

These plugins are defined in [rmq_enabled_plugins](rmq_enabled_plugins) and are activated on start-up when running
through docker-compose. To access the management UI, navigate to [http://localhost:15672/](http://localhost:15672/)
and login with username `guest` and password `guest`.

## Development
Run configurations for individual services are provided for those using the IntelliJ IDE. These run configurations
include commands to run unit tests. Any start scripts also include start scripts before launch for running any dependant
services.

## Examples

<!-- CONTRIBUTION -->
# Contribution
This project uses pre-commit hooks and commitizen to standardize commit messages and code styles.
To use the configurations, the first time you clone the repository, install the pre-commit hooks with
`pre-commit install`. If you do not have commitizen installed you can install it with `brew install commitizen`.

To commit and apply the styles, run `cz c` in the terminal. To bump the semantic version, run `cz bump`.
To generate a new change-log, run cz changelog. This will generate new entries in CHANGELOG.md.

One can apply these code style changes directly by running `cargo run clippy --fix` (set as a run configuration for
IntelliJ).

<!-- WHITEPAPERS -->
# Whitepapers
1. [Jericevich, I., Sing, D., Gebbie, T. (2021). Cointossx: An open-source low-latency high-throughput matching engine. arXiv:2102.10925.](https://arxiv.org/abs/2102.10925)
2. [Jericevich, I., Chang, P., Gebbie, T. (2021). Simulation and estimation of a point-process market-model with a matching engine. arXiv:2105.02211.](https://arxiv.org/abs/2105.02211)
3. [Jericevich, I., Chang, P., Gebbie, T. (2021). Simulation and estimation of an agent-based market-model with a matching engine. arXiv:2108.07806.](https://arxiv.org/abs/2108.07806)

<!-- RESOURCES -->
# Additional Resources
* [CoinTossX](https://github.com/dharmeshsing/CoinTossX)

<!-- TODO -->
# To-Do
* Create github workflows for docker
* Integrate postgres into github test workflow
