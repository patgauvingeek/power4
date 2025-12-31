# Power 4

## Developer

To call `cargo` and other commands

```bash
docker run --rm -it -v $(pwd):/app -w /app -u $(id -u):$(id -g) rust:alpine sh
```
