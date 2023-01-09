<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A binary and library crate for the management of a postgresql database and the versions thereof using an asynchronous ORM.
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
          <li><a href="#testing">Testing</a></li>
        </ol>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview
This crate serves to provide all the functionality relating to the database of an exchange.

<!-- STACK -->
## Stack
* [postgresql@14](https://www.postgresql.org/)
* [SeaORM](https://www.sea-ql.org/SeaORM/) asynchronous ORM
* [Standard rust asynchronous library](https://docs.rs/async-std/latest/async_std/)

<!-- PREREQUISITES -->
## Prerequisites
* postgresql@14
* (Optional) A UI for viewing/editing/managing databases such as PgAdmin or DBeaver

<!-- USAGE -->
# Usage
Begin by starting a postgres server and creating a database instance. Then set the following environment variables in
the run configurations:
* `DB_NAME`: The name of the database to connect to (default = `Rust`).
* `DB_HOST`: The host on which the postgres server is running (default = `localhost`).
* `DB_USERNAME`: The username associated with the database (default = `postgres`).
* `DB_PASSWORD`: The password for connecting to the database (default = `Boomers4life!123`).

Alternatively one can choose to rather provide a single environment variable for the `DATABASE_URL`
(default = `postgresql://postgres:Boomers4life!123@localhost:5432/Rust`).

This will allow for the database connection to be properly authenticated when each migration is run. Note that all run
configurations come with pre-launch commands to build the crate and start postgres.

All core SQL query operations are found in [src/core/query.rs](src/core/query.rs). All core mutation/CRUD operations
are found in [src/core/mutation.rs](src/core/mutation.rs).

All functionality relating to migrations are found in the [src/migrator](src/migrator) directory.

Code generated SeaOrm entities are exported in the [src/entities](src/entities) directory along with API request models
and their respective OpenApi schemas.

<!-- MIGRATIONS -->
## Migrations
Specify the table schemas and migration scripts in the migrator directory. Be sure to include any new migrations
scripts in the [Migrator](src/migrator/mod.rs) struct. One must include both the downgrade and upgrade scripts since,
each time the program is run, the migration scripts work by rolling back all migrations and then reapplying them
(this applies only when the `--refresh` option is used).

The binary file in this create is for running migrations programmatically. To run a migration on a database run
`cargo run`. This will execute [main.rs](src/main.rs). The current programmatic configuration is to completely refresh
the database.

For more information regarding a more granular management of migrations, refer to the section below - Migrator CLI.

<!-- CLI -->
### Migrator CLI
The following commands can be executed to perform more granular migrations functions. Ensure that you export the URL
of the running database before you execute any of the commands below - otherwise the `-u` option is required in the
commands below. Note that if you are using the IntelliJ IDE the below commands are already set as run configurations
in the [.idea](.idea) directory.
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
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
  ```sh
  sea-orm-cli migrate up \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Apply first 10 pending migrations
  ```sh
  sea-orm-cli migrate up \
    -n 10 \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Rollback last applied migrations
  ```sh
  sea-orm-cli migrate down \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Rollback last 10 applied migrations
  ```sh
  sea-orm-cli migrate down \
    -n 10 \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Drop all tables from the database, then reapply all migrations
  ```sh
  sea-orm-cli migrate fresh \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Rollback all applied migrations, then reapply all migrations
  ```sh
  sea-orm-cli migrate refresh \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Rollback all applied migrations
  ```sh
  sea-orm-cli migrate reset \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```
- Check the status of all migrations
  ```sh
  sea-orm-cli migrate status \
    -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
    -d .
  ```

<!-- CODEGEN -->
## Generate models/entities from database schema
Once the database has been successfully migrated, the updated models/entities can be generated. This is done by the
`sea-orm` cli which looks at the table schemas on the connected postgresql server. You may be required run
`cargo install sea-orm-cli` if not done already (see dev-dependencies in [Cargo.toml](Cargo.toml)). The command below
generates entities/models in the [entities](src/entities) directory.
```sh
sea-orm-cli generate entity --with-serde both \
  -u postgresql://<DB_USERNAME>:<DB_PASSWORD>@<DB_HOST>:5432/<DB_NAME> \
  -o src/entities
  -d .
```
The above command is also set as an IntelliJ run configuration

## Generate database schema from models/entities

<!-- TESTING -->
## Testing
To run all unit tests and integrations tests run `cargo test --features mock`. All integration tests can be found in
the [tests](tests) directory. Integration tests are conducted with a 'mock' database (a feature of `sea-orm`). This
requires the optional 'mock' feature.

# Additional Resources
* 