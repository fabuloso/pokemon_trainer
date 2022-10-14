FROM rust:1-buster

ARG TARGETARCH

WORKDIR /code

RUN apt update && apt install -y cmake

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres --version 0.6.0 && \
    curl -o /usr/local/bin/wait-for-it https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /usr/local/bin/wait-for-it
