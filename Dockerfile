# Stage 1: Build the Rust project
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the project with optimizations
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/bybe .

# Expose the port that your Actix-Web application will listen on
EXPOSE 25566

# Command to run your application when the container starts
CMD ["./bybe"]
