#!/bin/bash

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.36-slim cargo build --release
