FROM alpine:3.17 as builder

FROM builder as add-rust-binary

# Copy application binary from builder image
COPY /target/x86_64-unknown-linux-musl/release/telegram-bot /usr/local/bin

# Run the application
CMD ["/usr/local/bin/telegram-bot"]
