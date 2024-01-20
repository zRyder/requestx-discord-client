# Use a Rust image as the base image
FROM rust:1.75 as builder

# Set the working directory inside the container
WORKDIR /usr/src/requestx-api

# Copy the entire local Rust project into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Copy the built executable from the previous stage
RUN cp target/release/requestx-discord-client .

# Specify the default command to run when the container starts
CMD ["./requestx-discord-client"]