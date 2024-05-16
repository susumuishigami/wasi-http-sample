#1/bin/bash

docker compose run --rm dev cargo build --target wasm32-wasi
docker buildx build --platform wasi/wasm32 --provenance=false -t wasi-http wasi-http
docker container run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 -p 7878:7878 wasi-http
