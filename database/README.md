<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A library crate for the management of a postgresql database and the versions thereof using an asynchronous ORM.
    </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#overview">Overview</a></li>
        <ol>
          <li><a href="#stack">Stack</a></li>
          <li><a href="#prerequisites">Prerequisites</a></li>
        </ol>
        <li><a href="#usage">Usage</a></li>
        <ol>
          <li><a href="#migrations">Migrations</a></li>
          <ol><li><a href="#cli">Migrator CLI</a></li></ol>
          <li><a href="#codegen">Generate models/entities</a></li>
          <li><a href="#building">Building</a></li>
          <li><a href="#testing">Testing</a></li>
        </ol>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview

<!-- STACK -->
## Stack
* [postgresql@14](https://www.postgresql.org/)
* [SeaORM](https://www.sea-ql.org/SeaORM/) asynchronous ORM
* [Standard rust asynchronous library](https://docs.rs/async-std/latest/async_std/)

<!-- PREREQUISITES -->
## Prerequisites
* Rust
* postgresql@14
* (Optional) A UI for viewing/editing/managing databases such as PgAdmin or DBeaver

<!-- USAGE -->
# Usage
Begin by starting a postgres server and creating a database instance. Then set the following environment variables in the [.env](.env) file:
* `DB_NAME`: The name of the database to connect to.
* `DB_HOST`: The host on which the postgres server is running.
* `DB_USERNAME`: The username associated with the database.
* `DB_PASSWORD`: The password for connecting to the database.

This will allow for the database connection to be properly authenticated when each migration is run.

<!-- MIGRATIONS -->
## Migrations
Specify the table schemas and migration scripts in the migrator directory. Be sure to include any new migrations scripts in the [Migrator](src/migrator/mod.rs) struct. One must include both the downgrade and upgrade scripts since, each time the program is run, the migration scripts work by rolling back all migrations and then reapplying them (this applies only when the `--refresh` option is used).

The binary file in this create is for running migrations programmatically. To run a migration on a database run `cargo run`. This will execute [main.rs](src/main.rs). The current programmatic configuration is to completelt refresh the database.

For more information regarding a more granular management of migrations, refer to the section below - Migrator CLI.

<!-- CLI -->
### Migrator CLI
The following commands can be executed to perform more granular migrations functions. Ensure that you export the URL of the running database before you execute any of the commands below - otherwise the `-u` option is required in the commands below.
```sh
export DATABASE_URL="postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME>"
```

- Generate a new migration file
  ```sh
  sea-orm-cli migrate generate <MIGRATION_NAME> --migration-dir src/migrator
  ```
- Apply all pending migrations
  ```sh
  sea-orm-cli migrate \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
  ```sh
  sea-orm-cli migrate up \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Apply first 10 pending migrations
  ```sh
  sea-orm-cli migrate up \
    -n 10 \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Rollback last applied migrations
  ```sh
  sea-orm-cli migrate down \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Rollback last 10 applied migrations
  ```sh
  sea-orm-cli migrate down \
    -n 10 \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Drop all tables from the database, then reapply all migrations
  ```sh
  sea-orm-cli migrate fresh \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Rollback all applied migrations, then reapply all migrations
  ```sh
  sea-orm-cli migrate refresh \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Rollback all applied migrations
  ```sh
  sea-orm-cli migrate reset \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```
- Check the status of all migrations
  ```sh
  sea-orm-cli migrate status \
    -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
    -d .
  ```

<!-- CODEGEN -->
## Generate models/entities
Once the database has been successfully migrated, the updated models/entities can be generated. This is done by the `sea-orm` cli which looks at the table schemas on the connected postgresql server. You may be required run `cargo install sea-orm-cli` if not done already (see dev-dependencies in [Cargo.toml](Cargo.toml)). The command below generates entities/models in the [entities](src/entities) directory.
```sh
sea-orm-cli generate entity --with-serde both \
  -u postgresql://postgres:Fluffydog1996@localhost:5432/Rust \
  -o src/entities
  -d .
```
The user may also run the `generate_entities.sh` script to do the same thing with the proper environment variables set.

<!-- BUILDING -->
## Building
Run `cargo build` to build the [target](target) library and binary.

<!-- TESTING -->
## Testing
To run all unit tests and integrations tests run `cargo test --features mock`. Unit tests are found in [main.rs](src/main.rs) while all integration tests can be found in the [tests](tests) directory. Integration tests are conducted with a 'mock' database (a feature of `sea-orm`). This requires the optional 'mock' feature.