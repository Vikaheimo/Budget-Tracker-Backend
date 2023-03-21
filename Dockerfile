FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/Budget-Tracker-Backend /app/
COPY *.toml /app/
COPY migrations /app/migrations
EXPOSE 8000
ENTRYPOINT [ "/app/Budget-Tracker-Backend" ]
