FROM scratch
COPY ./target/wasm32-wasi/debug/wasi-http.wasm /wasi-http.wasm
ENTRYPOINT ["/wasi-http.wasm"]
