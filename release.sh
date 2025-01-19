#!/bin/bash
docker run -it --rm     -v /home/dvadell/docks/docker_developer/home/jr/:/workdir     -v /home/dvadell/docks/docker_developer/home/.cargo/git:/root/.cargo/git     -v /home/dvadell/docks/docker_developer/home/.cargo/registry:/root/.cargo/registry     registry.gitlab.com/rust_musl_docker/image:stable-latest     cargo build --release -vv --target=x86_64-unknown-linux-musl
