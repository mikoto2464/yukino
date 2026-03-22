# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build           # Debug build
cargo build --release # Release build
cargo run             # Run server (listens on 0.0.0.0:8088)
cargo check           # Check without building (faster)
cargo test            # Run tests
cargo clippy          # Lint
cargo fmt             # Format code
```

## Architecture

**yukino-backend** is a Rust HTTP API server built with:

- **Axum 0.8.x** — async web framework (routes, handlers, extractors)
- **Tokio** — async runtime (`#[tokio::main]` entry point in `src/main.rs`)
- **Hyper + Tower** — underlying HTTP/middleware stack (transitive via axum)
- **Serde + serde_json** — JSON serialization (transitive via axum)

The server binds to port **8088** by default. The single entry point is `src/main.rs`, which builds the Axum `Router` and starts the TCP listener. Routes are registered on the router using Axum's `routing::get/post/etc.` helpers.

Rust edition **2024** is used.
