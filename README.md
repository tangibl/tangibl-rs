# Tangibl

[![Build](https://github.com/battesonb/tangibl-rs/actions/workflows/build.yml/badge.svg)](https://github.com/battesonb/tangibl-rs/actions/workflows/build.yml)

This is the core Tangibl library. This source was originally written in Java,
but has been rewritten in rust to allow the generation of a dynamic C library,
WASM etc.

Separate interfaces will need to be made to consume the dynamic C library for
each language in which Tangibl is consumed. This should be minimal effort as the
frontend simply needs to parse the final JSON representation into a usable tree
of nodes. Rust, however, can skip this step and use the AST directly.
