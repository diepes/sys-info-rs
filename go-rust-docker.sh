#!/usr/bin/env bash

docker run --platform=linux/amd64 -it \
    -v ${PWD}:/root/$(basename ${PWD}) \
    -w /root/$(basename ${PWD}) \
    -e CARGO_NET_GIT_FETCH_WITH_CLI=true \
    docker.io/diepes/debug:latest bash

# cargo install cargo-watch
