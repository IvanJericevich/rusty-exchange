<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A binary crate for a RESTful API which will serve and receive data from a user via the http protocol.
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
Routes can be found in the routes directory. All models and ORM operations are imported from the database library crate.
Middleware can be found in the middleware directory. To view the OpenAPI schema navigate to ... To view the swagger docs navigate to ...

<!-- STACK -->
## Stack
* Actix-Web
* OpenAPI
* JWT

<!-- USAGE -->
# Usage
During development it can be very handy to have cargo automatically recompile the code on changes. This can be accomplished very easily by running the binary with `cargo watch -x 'run'`. This requires that you install cargo-watch