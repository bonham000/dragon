# Dragon Server

This is a very simple Rust web server built using Rocket and Diesel which provides basic CRUD functionality on top of a Postgres database.

This service was built to support the Chinese learning app [*Everyday Luck*](https://everyday-luck.surge.sh).

## Development

To run the app, you must have Rust and the Diesel ORM installed. The app uses Rocket, which requires Rust nightly.

To install Rust [see here](https://www.rust-lang.org/tools/install) or just run `curl https://sh.rustup.rs -sSf | sh`.

To install Diesel run `cargo install diesel_cli --no-default-features --features postgres`.

Now add an environment variable called `DATABASE_URL` which points to a running Postres instance.

After that, you need to let Diesel setup your database. Run `diesel setup`.

Now you should be good to go and you can start the app with `cargo run`.
