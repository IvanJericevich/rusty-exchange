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
          <li><a href="#contribution">Contribution</a></li>
        </ol>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview
Inspired by [CoinTossX](), this repo aims to create a high-throughput, low-latency fullstack matching engine written
entirely in Rust

<!-- CRATES -->
## Crates
* [API](api) ([docs](api/README.md))
* [Database](database) ([docs](database/README.md))

## Stack
### Services
* Postgres
* RabbitMQ
* Docker

### Libraries
* SeaOrm
* Actix-Web

<!-- CONTRIBUTION -->
## Contribution
This project uses pre-commit hooks and commitizen to standardize commit messages and code styles.
To use the configurations, the first time you clone the repository, install the pre-commit hooks with
`pre-commit install`. If you do not have commitizen installed you can install it with `brew install commitizen`.

To commit and apply the styles, run `cz c` in the terminal. To bump the semantic version, run `cz bump`.
To generate a new change-log, run cz changelog. This will generate new entries in CHANGELOG.md.

One can apply these code style changes directly by running `cargo run clippy --fix` (set as a run configuration for
IntelliJ).

# TODO
* Create database integration tests
* Create API tests

# Whitepapers
