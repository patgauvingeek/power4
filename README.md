# Power 4

## Developer

To call `cargo` and other commands:

```bash
docker run --rm -it -v $(pwd):/app -w /app -u $(id -u):$(id -g) rust:alpine sh
```

## Build

To generate product and run it:

```bash
docker build . --tag connect-four
docker run --rm -it connect-four
```