#build stage
FROM rust:1.98 as builder

RUN cargo install sccache --locked

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release



#runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/auth .
EXPOSE 7005
CMD ["./auth"]