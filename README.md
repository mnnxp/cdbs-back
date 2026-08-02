CADBase Backend
=====
A high-performance, containerized GraphQL API server for managing and sharing CAD models, technical drawings, and manufacturer data. Built with Rust, Diesel, and Actix-web.

## Getting Started

> **Database:** This repository is backend-only. Database migrations are managed in a separate repository.
> Make sure PostgreSQL is running and the database is set up before starting the server.

```sh
# Copy environment variables
cp .env.example .env

# Setup environment variables
vi .env

# Generate JWT keys (required)
mkdir -p keys
openssl genrsa -out keys/rs256-4096-private.pem 4096
openssl rsa -in keys/rs256-4096-private.pem -pubout > keys/rs256-4096-public.pem

# Run the server
cargo run
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| **Server** |||
| `API_POINT` | GraphQL endpoint URL | `http://127.0.0.1:3000/graphql` |
| `DOMAIN` | Server bind domain | `0.0.0.0` |
| `PORT` | Server port | `3000` |
| `RUST_LOG` | Log level (info, debug, trace, warn, error) | `info` |
| **Authentication** |||
| `JWT_PRIVATE_KEY` | Raw PEM content of the RSA private key | `"-----BEGIN PRIVATE KEY-----\nMIIJQQIBADANB..."` |
| `JWT_PUBLIC_KEY`  | Raw PEM content of the RSA public key | `"-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqh..."` |
| `AUTH_DURATION_IN_HOUR` | JWT token validity in hours | `24` |
| **CORS** |||
| `ALLOWED_ORIGINS` | Comma-separated list of allowed CORS origins | `http://localhost:3000,http://127.0.0.1:3000` |
| **Database** |||
| `POSTGRES_HOST` | PostgreSQL host | `localhost` |
| `POSTGRES_USER` | PostgreSQL user | `postgres` |
| `POSTGRES_PASSWORD` | PostgreSQL password | `password` |
| `POSTGRES_DB` | PostgreSQL database name | `cdbs` |
| **S3 Storage** |||
| `S3_APPLICATION_KEY_ID` | S3 access key ID | (required) |
| `S3_APPLICATION_KEY` | S3 secret access key | (required) |
| `S3_ACCESS_EXPIRATION_AT` | S3 credentials expiration date | `2030-01-01T00:00:00` |
| `S3_BUCKET` | S3 bucket name | `your_bucket` |
| `S3_REGION` | S3 region | `your_region` |
| `S3_ENDPOINT` | S3 endpoint URL | `https://your-s3-endpoint` |
| `S3_EXP_PRESIGNED_URL` | Presigned URL expiration in seconds | `800` |
| **System UUIDs** |||
| `ROOT_COMPONENT_UUID` | Root component UUID (self-referencing parent) | `a5953fd9-7393-4f1e-a899-06b5e159dbf1` |
| `ROOT_STANDARD_UUID` | Root standard UUID (self-referencing parent) | `303ec2aa-2066-42e3-93fb-de4fb9344bcb` |
| `ROOT_MODIFICATION_UUID` | Root modification UUID | `aba22d59-4f6c-44a4-9a37-2d38f0e577a8` |
| `DEFAULT_IMAGE_UUID` | Default placeholder image UUID | `bc1c2151-86d0-4656-9c9d-d016dd584297` |

> **Note:** All environment variables can also be passed as CLI arguments. Run `cargo run -- --help` for details.
> **Warning:** System UUIDs must match existing database values. Do not change them unless you know what you are doing.

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
## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository and create a feature branch
2. Run `cargo fmt` and `cargo clippy` before committing
3. Open a Pull Request with a clear description of changes
4. Ensure all tests pass: `cargo test` and `yarn testci`

By contributing, you agree that your contributions will be licensed under the AGPL v3 license.

For major changes, please open an issue first to discuss what you would like to change.

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

## License

**CADBase** is dual-licensed:

- **AGPL v3** — for open source use, ensuring that all modifications made to the software and offered over a network are shared with the community.
- **Commercial License** — for organizations that wish to use CADBase in proprietary environments or without the source-code disclosure obligations of the AGPL v3.

For commercial licensing inquiries, contact: info@cadbase.rs