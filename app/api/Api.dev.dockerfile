FROM rust:1.85.1

# Install cargo-watch for live reloading
RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy all project files into the container
COPY . .

# Optional: Pre-fetch dependencies to speed up initial dev run
RUN cargo fetch

# Start with live reload on file changes
CMD ["cargo", "watch", "-s", "cargo clean && cargo run"]
