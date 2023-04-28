# Use an official Rust image as the build stage
FROM rust:1.69 as build

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml file to the working directory
COPY Cargo.toml .

# Create a dummy src folder and a main.rs file to build the dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the dummy src folder and main.rs file
RUN rm -rf src

# Copy the source code to the working directory
COPY src ./src

# Build the application
RUN cargo build --release

# Start a new stage to create the final image
FROM debian:buster-slim

# Install necessary packages for SSL
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the final binary from the build stage to the final image
COPY --from=build /usr/src/app/target/release/kubernetes_cpu_limit_test /app/kubernetes_cpu_limit_test

# Set the binary as the container's entrypoint
ENTRYPOINT ["/app/kubernetes_cpu_limit_test"]