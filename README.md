CADBase Backend
=====
The platform for publishing and sharing information on drawings and manufacturers.

## The CADBase project is based on the Canduma project
- bases - [link](https://github.com/clifinger/canduma)

## Collection of major crates used in CADBase
- actix - [link](https://actix.rs/)
- actix-web - [link](https://docs.rs/actix-web/)
- actix-cors - [link](https://docs.rs/actix-cors/)
- actix-multipart - [link](https://docs.rs/actix-multipart/)
- diesel - [link](http://diesel.rs/)
- async-graphql - [link](https://docs.rs/crates/async-graphql)
- uuid - [link](https://docs.rs/uuid/)
- time - [link](https://docs.rs/time/)
- chrono - [link](https://docs.rs/chrono/)
- serde_json - [link](https://docs.serde.rs/serde_json/)
- argon2rs - [link](https://github.com/bryant/argon2rs)
- jsonwebtoken - [link](https://docs.rs/jsonwebtoken)
- anyhow - [link](https://github.com/dtolnay/anyhow)
- thiserror - [link](https://github.com/dtolnay/thiserror)
- shrinkwraprs - [link](https://docs.rs/shrinkwraprs/)
- sanitize-filename - [link](https://docs.rs/sanitize-filename/)
- structopt - [link](https://docs.rs/structopt/)
- regex - [link](https://docs.rs/regex/)
- digest - [link](https://docs.rs/digest/)
- blake3 - [link](https://docs.rs/blake3/)
- bytes - [link](https://docs.rs/bytes/)

## Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

## Getting Started

```sh
docker-compose up
cp .env.example .env
diesel setup --database-url='postgres://postgres:password@localhost/cdbs'
diesel migration run
cargo run
```
