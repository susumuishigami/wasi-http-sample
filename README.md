## 概要

WASM-WASIを使って簡易的なWebサーバーを作ってみた。
Dockerがあれば開発できるようにした。

Docker Desktop for mac で動作確認しています。

## setup

```console
$ docker compose build
```

## compile

```console
$ docker compose run --rm dev cargo build --target wasm32-wasi
```

## build image

```console
$ docker buildx build --platform wasi/wasm32 --provenance=false -t wasi-http wasi-http
```

## execute

Enable: Docker Desktop > "Features in development" > "Use containerd for pulling and storing images"

```console
$ docker container run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 -p 7878:7878 wasi-http
```

ブラウザで http://localhost:7878 にアクセスすると `Hello, World` と表示される

## run.sh

compile, build, executeをまとめて実行します

```console
$ ./run.sh
```

## 参考にしたサイト
以下を参考にしました

- https://toranoana-lab.hatenablog.com/entry/2023/06/06/100000
- https://www.creationline.com/tech-blog/60397
