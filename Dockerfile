# select build image
FROM rust:1.84.1 AS build

WORKDIR /cdbs-back

# Cache dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source
COPY ./src ./src

# Rebuild with code
RUN touch src/main.rs && cargo build --release

# our final base
FROM debian:trixie-slim

RUN apt-get update && apt-get upgrade -y \
	&& apt-get install --yes ca-certificates libpq5 openssl;

RUN apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /var/log/*

RUN openssl version

# copy the build artifact from the build stage
COPY --from=build /cdbs-back/target/release/cdbs-back /usr/local/bin

# RUN ls -la /
RUN ls -lh /usr/local/bin/cdbs-back

# set the startup command to run your binary
ENTRYPOINT ["cdbs-back"]
