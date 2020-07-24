![Rust](https://github.com/gowthamgutha/rust-rocket/workflows/Rust/badge.svg)

# rust-rocket
Practice examples of rust rocket

# Building

```bash
cargo build --release --target=x86_64-unknown-linux-gnu
```

1. This builds for linux x86_64 architecture.
2. The binary contains all the dependencies (crates)
3. The `--release` tag optimizes the build for production use.
4. The binary file will be under `target/x86_64-unknown-linux-gnu/release/` folder

# Running

For just running, the program use

```bash
cargo run
```