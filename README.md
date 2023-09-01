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
