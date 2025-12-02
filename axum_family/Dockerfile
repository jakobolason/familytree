FROM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json

# Install git for git dependencies
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Build dependencies - this layer is cached
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code
COPY . .

# Build application
RUN cargo build --release

###############################################################

FROM debian:bookworm-slim

ENV RUST_LOG=info

WORKDIR /app

# Install runtime dependencies if needed
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/sea-orm-pro-backend-cli /app
COPY config /app/config
COPY pro_admin /app/pro_admin
COPY assets /app/assets

EXPOSE 8086

CMD ["/app/sea-orm-pro-backend-cli", "start"]
