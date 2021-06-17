# See: https://crates.io/crates/cargo-chef

FROM rust:1.52-alpine as planner

ADD \
    Cargo.toml \
    Cargo.lock \
    /workspace/

WORKDIR /workspace

# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN apk add --no-cache --update musl-dev
RUN cargo install cargo-chef
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.52-alpine as cacher

WORKDIR /workspace

RUN apk add --no-cache --update musl-dev openssl-dev
RUN cargo install cargo-chef
COPY --from=planner /workspace/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.52-alpine as builder

COPY . /workspace
WORKDIR /workspace

# Copy over the cached dependencies
COPY --from=cacher /workspace/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN apk add --no-cache --update mariadb-connector-c-dev musl-dev openssl-dev
# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin air-balloon-burner

FROM rust:1.52-alpine as diesel-build

COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN apk add --no-cache --update mariadb-connector-c-dev musl-dev openssl-dev
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install diesel_cli --no-default-features --features mysql

FROM alpine:3.12 as runtime
# https://github.com/mischov/meeseeks/issues/98#issuecomment-636615680
RUN apk add --no-cache --update mariadb-connector-c libgcc openssl

WORKDIR /app
COPY /migrations /app/migrations
COPY /docker/docker-entrypoint.sh /docker-entrypoint.sh

COPY --from=diesel-build /usr/local/cargo/bin/diesel /usr/local/bin
COPY --from=builder /workspace/target/release/air-balloon-burner /usr/local/bin

ENTRYPOINT ["/docker-entrypoint.sh"]
