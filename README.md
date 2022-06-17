# Tangibl

[![Build](https://github.com/battesonb/tangibl-rs/actions/workflows/build.yml/badge.svg)](https://github.com/battesonb/tangibl-rs/actions/workflows/build.yml)

This is the core Tangibl library. This source was originally written in Java,
but has been rewritten in Rust to allow the generation of a dynamic C library,
WASM etc.

## Example

```rust
use topcodes::TopCode;
use tangibl::JsonPrinter;

// Scan or generate your TopCodes.
let topcodes: Vec<TopCode> = ...;

// Parse codes in Tangibl abstract syntax tree.
let ast: Option<Start> = tangibl::parse(&topcodes);

// Create a visitor for printing JSON.
let mut json_printer = JsonPrinter::new();

println!("{}", json_printer.print(&ast));
```

The library additionally contains a JSON printer and a visitor abstraction for
performing actions based on the shape of the AST.

### Motivation for enum use in visitor

The visitor uses enum based matching instead of the more commonly used
accept/visit method pattern used in OOP languages. There are two reasons for
this:

1. Rust enum types are algebraic sum types, so a match statement must include
   all potential values.
1. Ownership and type rules are hard to reason about with the classic OOP
   approach, which makes it seem like a bad fit for Rust. Feel free to
   implement a JSON parser and custom visitor in the language of your choice.

## Looking forward

Separate interfaces will need to be made to consume the dynamic C library for
each language in which Tangibl is consumed. This should be minimal effort as the
frontend simply needs to parse the final JSON representation into a usable tree
of nodes. Rust implementations, however, can skip this step and use the AST
directly from this source.
