#!/usr/bin/env just --justfile

publish *ARGS:
    cd proc && cargo publish {{ARGS}}
    cargo publish {{ARGS}}