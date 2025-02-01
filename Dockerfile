FROM rust:latest

RUN set -xe

WORKDIR /app

COPY . .


RUN cargo install sqlx-cli

RUN sqlx database create

RUN sqlx migrate run

RUN cargo sqlx prepare

RUN cargo build --release

EXPOSE 8000

CMD ["cargo", "run", "--release"]