FROM rust:latest AS chef 

RUN cargo install cargo-chef 
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json # WARN: Caching layer!

COPY . .
RUN \
    # HACK: The bloody protoc wouldn't fucking install normally, so just curl it from Github releases.
    curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v25.1/protoc-25.1-linux-x86_64.zip \
    && unzip protoc-25.1-linux-x86_64.zip -d $HOME/.local \
    && export PATH="$PATH:$HOME/.local/bin" \
    && cargo build --release \
    && mv ./target/release/tcp-chat ./app



# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY --from=builder /app/app /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
