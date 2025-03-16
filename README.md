CADBase Backend
=====
The platform for publishing and sharing information on drawings and manufacturers.

## Getting Started

```sh
docker-compose up
cp .env.example .env
diesel setup --database-url='postgres://postgres:password@localhost/cdbs'
diesel migration run
cargo run
```
### Generate RSA keys for JWT

In development mode you can keep the one in `/keys` folder.

```shell script
// private key
$ openssl genrsa -out rs256-4096-private.pem 4096

// public key
$ openssl rsa -in rs256-4096-private.pem -pubout > rs256-4096-public.pem
```

### Logging

Logging controlled by middleware::Logger [actix.rs](https://actix.rs/docs/errors/)

To enable debug logging set `RUST_LOG=debug` in `.env`

### Testing

#### Initialization

First run `yarn` or `npm install` to get all required packages

#### yarn run test

To run you can use `npm run test` or `yarn test`.

#### yarn run testci
Running all tests
```bash
$ yarn run testci
```
Running the selected test
```bash
$ yarn testci -- user.test.js
```

## Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

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
