# Canoe

Simple REST API build with `rust`, `sqlite3`, `sqlx`, and `axum`.

## `tl;dr`

Run the application from the pre-built binaries.

1. Clone the repository.
2. Download the pre-built binaries from the GitHub pages into the project `./bin` directory.
3. Give the binary execution permissions `chmod +x ./bin/canoe`.
4. Run the binary using the `.env` configuration file.

```bash
env $(xargs <".env") ./bin/canoe
```

## Getting Started

The recommended way to run the service in development mode is by using the `cargo watch` command,
configured to look for changes inside the `crates/` folder. This will make it so that each new
change made to any of the crates, would restart the server.

```bash
env $(xargs <./dev.env) cargo watch -q -c -w crates/ -x "run --bin canoe"
```

## Endpoints

### Funds

> To follow along start the service in `development` mode. This will start the service on
> `localhost:2908`.

Configure the following environment variables to work through the endpoints examples.

```bash
URL="http://localhost:2908"
```

#### `GET /funds`

Get the list of `funds`. You can filter by `name`, `manager`, or `year`.

```bash
# Get all funds
curl "$URL/funds" \
    -H 'Content-Type: application/json'
# Get funds managed by company 1
curl "$URL/funds?filter=manager&value=1" \
    -H 'Content-Type: application/json'
```

Response Body: `Vec<Fund>`

#### `POST /funds`

Create a new `fund`

```bash
curl "$URL/funds" \
    -H 'Content-Type: application/json' \
    -d '{"name": "FooBarBax", "manager": 1, "start_year": 2023}'
```

Response Body: `Fund`

#### `GET /funds/:id`

Gets a `fund` by its `id`.

```bash
curl "$URL/funds/1" \
    -H 'Content-Type: application/json'
```

#### `PUT /funds/:id`

Updates the `name`, `manager`, or `start_year` of a `fund`.

```bash
curl "$URL/funds/1" \
    -H 'Content-Type: application/json' \
    -X PUT \
    -d '{"name": "New Foo"}'
```

## Builds

### Dev Build

All the application configuration is provided through environment variables. To start the app in
development mode load the `dev.env` file to your session and then use `cargo run` to start the
service.

```bash
env $(xargs < ./dev.env) cargo run --bin canoe
```

### Release Build

To run a `release` build run the same command as before but pass in the `--release` option to the
`cargo run` command. You can also use the `release.env` environment variables to avoid overwriting
the `dev.db` database file.

```bash
env $(xargs < ./release.env) cargo run --bin canoe --release
```

## Build Service

As before, we use `cargo` to build the `canoe` binary.

> Be sure to use the `--profile release` option to let Rust optimize the resulting binary.

```bash
cargo build --bin canoe --profile release
```

The binary gets build on the `target/release` directory. You need to set execution permissions
before running it.

```bash
# Give the binary execution permissions
chmod +x target/release/canoe
# Run the built binary with the `production` configuration.
env $(xargs <.env) ./target/release/canoe
```

## Additional comments about the task

I took the opportunity to practise my knowledge of `rust` and to try some tools that I've been
meaning to play with for a while, mainly:

- `axum` is a web framework that is developed by the same people who invented the `tokio` runtime,
    which is an amazing tool to develop concurrent applications. The whole `tokio` ecosystem is
    great, and the API exposed by `axum` is very good.
- `sqlx` is a library that allows you to interact with multiple SQL databases by writing SQL
    directly in your code, while avoiding SQL injections, and supporting the natural type system of the
    language. It also has a very robust set of tools to handle migrations, among other stuff.

I found the experience really gratifying and I would consider this a very good stack from which to
build an app.

Regarding the database, the `schema` can be found in the migrations directory. I liked the way
`sqlx` makes you interact with the SQL code without requiring an ORM. I feel like I'm much more in
control of the queries I perform. For example, being able to use the `WITH` clause greatly
simplifies the creation of the `check_duplicates` function, which could also be improved by using an
`Exists` query instead of a `SELECT Count(*)`.




The final implementation has support for
