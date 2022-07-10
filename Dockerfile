FROM rust:1.62.0

RUN USER=root cargo new --bin marusya
WORKDIR /marusya

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm ./src/*.rs

COPY ./src ./src

RUN cargo install --path .

CMD ["marusya"]
