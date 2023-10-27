# Stage 1: Build the Rust project
FROM rust:1.73-alpine as builder

# Set the working directory in the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Install all the required libraries
# GCC
RUN apk add build-base

RUN apk add musl-dev
RUN cargo install cross

# cross needs docker to work
RUN apk add --update docker openrc
RUN rc-update add docker boot

# Static binary magic
#RUN rustup target add aarch64-unknown-linux-musl
#RUN rustup toolchain install stable-aarch64-unknown-linux-musl

# Build the project with optimizations
RUN cargo build --target x86_64-unknown-linux-musl --release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bybe .

# Expose the port that your Actix-Web application will listen on
EXPOSE 25566
RUN apk add --no-cache bash
# Command to run your application when the container starts
ENTRYPOINT ["./bybe"]
