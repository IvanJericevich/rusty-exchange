<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A library crate for a RESTful API which will serve and receive data to/from a user via the http protocol.
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

<!-- OVERVIEW -->
# Overview

<!-- STACK -->
## Stack
* Actix-Web
* OpenAPI
* JWT

<!-- USAGE -->
# Usage
Routes can be found in the [src/routes](src/routes) directory. All models and ORM operations are imported from the
database library crate. Other needed models are created in the [src/models](src/models) directory.

Middleware can be found in the [src/middleware](src/middleware) directory.

To view the OpenAPI schemas and docs navigate to [http://localhost:8080/swagger/](http://localhost:8080/swagger/).
OpenAPI schemas for each route can be found by navigating to [http://localhost:8080/{route}-schema/openapi.json](http://localhost:8080/<route>-schema/openapi.json)

## Testing
Unit tests can be found in each of the routes. In order to run the tests, the user must create an empty postgres
database instance with credentials:
* `POSTGRES_DB`: Exchange - the name of the database to connect to.
* `POSTGRES_HOST`: localhost - the host on which the postgres server is running.
* `POSTGRES_USER`: postgres - the username associated with the database.
* `POSTGRES_PASSWORD`: postgres - the password for connecting to the database.

Set the above as environment variables as well and run `cargo test`. Alternatively one can run the "Test API" IntelliJ
run configuration. For each route the test will set up by running all migrations on the empty database and end by
rolling back the migrations after the tests complete.