<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A binary crate for a RESTful API which will serve and receive data to/from a user via the http protocol.
    </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#overview">Overview</a></li>
        <ol>
          <li><a href="#stack">Stack</a></li>
        </ol>
        <li><a href="#usage">Usage</a></li>
    </ol>
</details>
<br />

<!-- OVERVIEW -->
# Overview

<!-- STACK -->
## Stack
* Actix-Web
* OpenAPI
* JWT

<!-- USAGE -->
# Usage
During development it can be very handy to have cargo automatically recompile the code on changes. This can be
accomplished very easily by running the binary with `cargo watch -x 'run'`. This requires that you install cargo-watch
(see dev-dependencies in [Cargo.toml](Cargo.toml)).

Routes can be found in the [src/routes](src/routes) directory. All models and ORM operations are imported from the
database library crate. Other needed models are created in the [src/models](src/models) directory.

Middleware can be found in the [src/middleware](src/middleware) directory.

To view the OpenAPI schemas and docs navigate to [localhost:8080/swagger/](localhost:8080/swagger/)

## Testing
Unit tests can be found in each of the routes. In order to run the tests, the user must create an empty postgres
database instance with credentials:
* `DB_NAME`: Rust - the name of the database to connect to.
* `DB_HOST`: localhost - the host on which the postgres server is running.
* `DB_USERNAME`: postgres - the username associated with the database.
* `DB_PASSWORD`: Boomers4life!123 - the password for connecting to the database.

Set the above as environment variables as well and run `cargo test`. Alternatively one can run the "Test API" IntelliJ
run configuration. For each route the test will set up by running all migrations on the empty database and end by
rolling back the migrations after the tests complete.