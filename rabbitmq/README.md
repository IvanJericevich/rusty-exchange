<div align="center">
    <h3 align="center">Database</h3>
    <p align="center">
        A library crate for publishing messages to and subscribing to messages from RabbitMQ queues.
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
* postgresql@14
* (Optional) A UI for viewing/editing/managing databases such as PgAdmin or DBeaver

<!-- USAGE -->
# Usage
Begin by starting a RabbitMQ server. Then set the following environment variables in
the [.env](.env) file:
* `RMQ_URL`: The name of the database to connect to.