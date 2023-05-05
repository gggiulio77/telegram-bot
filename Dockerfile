FROM ubuntu:latest as builder

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

FROM builder as add-rust-binary

# Copy application binary from builder image
COPY /target/release/telegram-bot /usr/local/bin

# Run the application
CMD ["/usr/local/bin/telegram-bot"]
