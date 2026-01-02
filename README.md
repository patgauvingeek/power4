# Power 4

## Developer

To call `cargo` and other commands:

```bash
docker run --rm -it -v $(pwd):/app -w /app -u $(id -u):$(id -g) rust:alpine sh
cargo test
cargo run
```

## Build

To generate product and run it:

```bash
docker build . --tag power-4
docker run --rm -it power-4
```