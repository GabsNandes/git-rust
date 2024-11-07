# Build stage
FROM clux/muslrust as builder

WORKDIR /app

# Set custom toolchain and cargo home directories
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo

# Accept the build argument for DATABASE_URL
ARG DATABASE_URL

# Use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

# Set the default toolchain to stable
RUN rustup default stable

# Install the musl target explicitly
RUN rustup target add x86_64-unknown-linux-musl

# Build the application in release mode with musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Copy the statically linked binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/realcrud /usr/local/bin/realcrud

# Ensure the binary has the right permissions to execute
RUN chmod +x /usr/local/bin/realcrud

# Set the entrypoint to the Rust binary
CMD ["realcrud"]
