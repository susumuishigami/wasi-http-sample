FROM rust

RUN mkdir /app
WORKDIR /app
COPY ./wasi-http /app
RUN rustup target add wasm32-wasi
