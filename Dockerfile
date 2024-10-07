FROM rust:1.72-slim-buster as builder

WORKDIR /usr/src/csvcut
COPY . .

RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/csvcut/target/release/csvcut /usr/local/bin/csvcut

ENTRYPOINT ["csvcut"]