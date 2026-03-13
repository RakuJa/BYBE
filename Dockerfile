# Stage 1: Build the Rust project
FROM rust:1-alpine AS builder

# Set the working directory in the container
WORKDIR /app

# Install all the required libraries
# GCC
RUN apk add build-base

RUN apk add musl-dev

RUN apk add python3

# Copy the project files into the container
COPY . .

# Build the project with optimizations
RUN cargo install --no-default-features --force cargo-make
RUN cargo make bybe-docker-release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Adding sqlite, cannot do it before
RUN apk add sqlite

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bybe .
COPY --from=builder /app/data/database.db data/
COPY --from=builder /app/data/names.json data/
COPY --from=builder /app/data/nicknames.json data/

ENV DATABASE_URL="sqlite:///app/data/database.db"
ENV SERVICE_STARTUP_STATE="Clean"
ENV NAMES_PATH="/app/data/names.json"
ENV NICKNAMES_PATH="/app/data/nicknames.json"
ENV BACKEND_URL="https://api.bybe.com"

# Expose the port that your Actix-Web application will listen on
EXPOSE 25566
# Command to run your application when the container starts
ENTRYPOINT ["./bybe"]
