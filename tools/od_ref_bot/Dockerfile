# syntax=docker/dockerfile:labs

FROM rust:1.82 AS builder

WORKDIR /usr/src/app
COPY src/ src/
COPY build.rs build.rs
COPY Cargo.toml Cargo.toml

ADD https://github.com/OpenDreamProject/od-dm-reference.git#main /usr/ref

RUN SOURCE_DIR=/usr/ref/content cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/od_ref_bot /usr/local/bin/od_ref_bot

CMD ["od_ref_bot"]