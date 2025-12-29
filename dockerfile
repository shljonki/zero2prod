FROM lukemathwalker/cargo-chef:latest-rust-1.91.1 AS chef
#svim chefovima ce postaviti /app kao workdir
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
# vrti se u /app jer je tako postavljeno u chef koraku
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
#isto smo u /appu, kopiramo iz /app (absolute path) u /app (relative path)
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached. 
COPY . .
ENV SQLX_OFFLINE=true
#ovo je isto u /app
RUN cargo build --release --bin zero2prod

# Runtime stage
FROM debian:bookworm-slim AS runtime
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration files at runtime!
COPY configuration configuration
ENV APP_ENV="production"
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]