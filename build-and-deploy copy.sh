#!/bin/sh

cargo build --release --target aarch64-unknown-linux-gnu
docker build -t ghcr.io/searedcircuit/rustdemo-arm -f Dockerfile-arm64 .
docker push ghcr.io/searedcircuit/rustdemo-arm