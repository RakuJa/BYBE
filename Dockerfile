# Stage 1: Build the Rust project
FROM rust:1.75-alpine as builder

# Set the working directory in the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Install all the required libraries
# GCC
RUN apk add build-base

RUN apk add musl-dev

# Build the project with optimizations
RUN cargo build --target x86_64-unknown-linux-musl --release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Adding sqlite, cannot do it before
RUN apk add sqlite

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bybe .
COPY --from=builder /app/database.db .

ENV DATABASE_URL="sqlite:///app/database.db"

# Expose the port that your Actix-Web application will listen on
EXPOSE 25566
# Command to run your application when the container starts
ENTRYPOINT ["./bybe"]
