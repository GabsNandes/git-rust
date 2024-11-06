# Build stage
FROM rust:1.82-slim as builder

WORKDIR /app

# Accept the build argument for DATABASE_URL
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Install dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/realcrud /usr/local/bin/realcrud

# Ensure the binary has the right permissions to execute
RUN chmod +x /usr/local/bin/realcrud

# Set the entrypoint to the Rust binary
CMD ["realcrud"]
