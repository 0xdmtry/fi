FROM rust:1.85.1

# Install cargo-watch for live reloading
RUN cargo install cargo-watch

# Set working directory
 # ✅ matches the volume mount and copied files
WORKDIR /app

# Copy everything from the context (which is app/api)
 # ✅ includes Cargo.toml and src/
COPY . .

# Pre-fetch dependencies
RUN cargo fetch

# Run app with live reload
CMD ["cargo", "watch", "-x", "run"]
