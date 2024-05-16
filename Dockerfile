FROM rust

RUN mkdir /app
WORKDIR /app
COPY ./wasi-demo /app
RUN rustup target add wasm32-wasi
