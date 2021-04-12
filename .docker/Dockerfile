FROM rust

WORKDIR /app

COPY ./src .
COPY ./Cargo.* ./

RUN cargo run
EXPOSE 8080
