FROM rust:1-slim

LABEL name="thirty-back"

ARG ROOT=/var/www/app
RUN mkdir -p $ROOT
WORKDIR $ROOT

ARG PACKAGES="default-mysql-client libssl-dev openssl pkg-config"
RUN apt update \
    && apt upgrade \
    && apt install -y $PACKAGES

RUN cargo install cargo-watch

