# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install --path .

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8080

# Run the web service on container startup.
CMD ["meshi-api"]