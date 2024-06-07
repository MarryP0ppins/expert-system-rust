FROM messense/rust-musl-cross:x86_64-musl as chef
RUN apt-get update && apt-get upgrade -y
RUN cargo install cargo-chef
WORKDIR /expert-system-rust

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /expert-system-rust/recipe.json recipe.json
RUN apt install -y ca-certificates && update-ca-certificates
RUN cp /root/.cargo/config /root/.cargo/config.toml && rm /root/.cargo/config
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=builder /expert-system-rust/target/x86_64-unknown-linux-musl/release/expert-system-rust /expert-system-rust
ENTRYPOINT ["/expert-system-rust"]
EXPOSE 8000