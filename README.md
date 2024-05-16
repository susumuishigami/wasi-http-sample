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

※ `username` の箇所は各自書き換える

```console
$ docker buildx build --platform wasi/wasm32 -t username/wasi-http wasi-demo
```

2024/05/16現在、ここで必ず以下のエラーが出るがイメージはできている（謎）
>  => ERROR exporting to image

```
susumuis@SusumunoMacBook-Pro wasmwasi % docker images                                                           
REPOSITORY           TAG       IMAGE ID       CREATED          SIZE
wasmwasi-dev         latest    ca8ddcfb31b2   37 minutes ago   2.15GB
susumuis/wasi-http   latest    90fb360fc2d1   26 minutes ago   4.95MB
```

## execute

Enable: Docker Desktop > "Features in development" > "Use containerd for pulling and storing images"

```console
$ docker container run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 -p 7878:7878 "username/wasi-http:latest"
```

ここで `Unable to find image 'username/wasi-http:latest' locally` のようなエラーが出る場合、何度かDocker desktopを再起動すると動いた（謎）

ブラウザで http://localhost:7878 にアクセスすると `Hello, World` と表示される

## 参考にしたサイト
以下を参考にしました

- https://toranoana-lab.hatenablog.com/entry/2023/06/06/100000
- https://www.creationline.com/tech-blog/60397
