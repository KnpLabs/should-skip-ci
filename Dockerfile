FROM rust:1.59.0-bullseye as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:1.59.0-bullseye as release

RUN apt-get update \
    && apt-get install -y \
        git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/ssc /usr/bin/ssc

USER 1000

CMD ["ssc"]
