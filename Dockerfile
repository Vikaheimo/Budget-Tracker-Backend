FROM rust:bullseye AS builder
WORKDIR /build
COPY . .
RUN cargo build
# Add release build later!!!!!!!!!!!!!!

FROM debian:bullseye-slim
WORKDIR /app
# Fix this too
COPY --from=builder /build/target/debug/Budget-Tracker-Backend /app/
RUN /app/Budget-Tracker-Backend
COPY *.toml /app/
COPY migrations /app/migrations
EXPOSE 8000
ENTRYPOINT ["/app/Budget-Tracker-Backend"]
