FROM rust:1.64.0-bullseye

RUN apt-get update \
    && apt-get install -y \
        git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

USER 1000
