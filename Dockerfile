# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:slim-buster as BUILDER

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

FROM alpine:3.9

COPY --from=BUILDER /usr/src/app/target/release/meshi-api meshi-api

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8000

# Run the web service on container startup.
ENTRYPOINT ["./meshi-api"]