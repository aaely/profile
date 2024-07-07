# Use an official Rust image as the build environment
FROM rust:latest as builder

# Install wasm-pack
RUN cargo install wasm-pack

# Create a new directory for the project
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the project using wasm-pack
RUN wasm-pack build --target web

# Use a lightweight web server to serve the static files
FROM nginx:alpine

# Copy the generated wasm and other static files to the web server's root directory
COPY --from=builder /app/pkg /usr/share/nginx/html
COPY static /usr/share/nginx/html

# Expose the port that nginx will run on
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]