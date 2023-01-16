# Use the official Rust image as the build stage
FROM rust:latest as build-stage

# Set the working directory
WORKDIR /usr/src/crablendar

# Copy the Cargo.toml file to the build stage
COPY Cargo.toml .

# Copy the rest of the code to the build stage
COPY src src

# Build the app
RUN cargo build --release

# Use the bitnami/minideb image as the final stage
FROM bitnami/minideb:latest

# Set the working directory
WORKDIR /app

#Install ca certificates
RUN apt update && apt install -y ca-certificates

# Copy the built binary from the build stage
COPY --from=build-stage /usr/src/crablendar/target/release/crablendar /usr/local/bin/crablendar

EXPOSE 8080
# Run the binary
CMD ["/usr/local/bin/crablendar"]
