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

### Comments

I took the opportunity to practice my knowledge of `rust` and to try some tools that I've been
wanting to play with for a while, primarily:

- `axum` is a web framework that is developed by the same group who formulated the `tokio` runtime,
    which is an exceptional tool for developing concurrent applications. The entire `tokio` ecosystem is
    excellent, and the API exposed by `axum` is very good.
- `sqlx` is a library that enables you to interact with multiple SQL databases by writing SQL
    directly in your code, while avoiding SQL injections, and supporting the natural type system of the
    language. It also offers a highly robust set of tools for handling migrations, among other things.

I found this experience to be quite gratifying and would consider this as an impressive stack to
build an app from.

### Database

With respect to the database, the `schema` can be found in the migrations directory. I appreciated
the manner in which `sqlx` allows you to interact with SQL code without the necessity of an ORM. It
provides me the feeling of having more control over the queries I am executing. For instance, the
ability to utilize the `WITH` clause significantly simplifies the creation of the `check_duplicates`
function, which could be further improved by employing an `Exists` query instead of a `SELECT
Count(*)`.

### Modularity

Every main part of the app is divided into separate modules:

- `config`: Reads the application configuration.
- `db`: Configures the database and runs migrations.
- `tasks`: Starts the tasks processing event loop.
- `app`: Launches the web application.

If necessary, each of these modules can be transformed into its own `crate` for reusability in other services.

### Background Tasks

I aimed to create a simplified version of an event loop, where workers wait for events to process
specific tasks. I chose to not use an existing one so that I could educate myself on the hurdles of
integrating this feature in `rust`. My current setup works by initiating a process on a separate
`thread` that consistently monitors the status of a `FIFO` queue for new events. When it identifies
one, it processes it. An `Event` comprises of a `name` and a JSON serialized `payload`.

To securely access the `queue`, it is enclosed within an `Arc` smart pointer and a `tokio:Mutex`. This
permits you to `lock` access to the `queue` while reading and writing to it between threads, guaranteeing
consistency among them. To ensure that a process doesn't retain the `MutexGuard` longer than
necessary, all lock operations occur inside a simple `function`, ensuring that `rust` drops the
guard reference after generating a new event.

There are three crucial features that I didn't implement for the `tasks` module:

1. Support for multiple channels: Currently, all the events are transmitted through a single `queue` which is used by
   all the task workers, so there's no filtering option. Fortunately, the `match` operator would guarantee
   the performance of the single channel, even if the volume of `tasks` increases significantly. An even
   better optimization could be the employment of an `enum` for the `Event` name, rather than a `String`.
2. Support for multiple workers: The `events::init` function currently only generates a single task worker
   running on a separate thread. It would have been beneficial to have the capacity to spawn more
   than one. Given the current architecture, it wouldn't be challenging to implement.
3. Support for a controller procedure to track worker health: Even by leveraging `rust` error
   handling features, something unexpected could occur to the worker that could bring it down, leaving the
   service unable to process new events. A possible solution for this is having a `controller` that is
   consistently checking the health of the `worker/s` and can restart them if they go down, or halt
   the application execution if it's in an unrecoverable state.

That being said, the feature works and one can easily access the `tasks` object via the `axum`
application state.

### Scalability

Given `rust`'s inherent performance and parallelism capabilities, the bottleneck between the app and
the database would most likely be due to the number of concurrent connections the DB is able to
handle. More threads could be utilized to process background tasks or application HTTP requests.
`axum` naturally supports multi-threading.

Also, since all the application endpoints are idempotent—with the database as the only hard
dependency—a feasible way to accommodate multiple new users would be by operating multiple
iterations of the app on varied
