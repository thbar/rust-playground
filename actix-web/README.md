A playground around [Actix Web](https://actix.rs), a web framework for Rust.

### Building

```
cargo build
cargo run
```

### Example queries

```
# hello world
curl -i http://localhost:8080/

# echo with POST body
curl -X POST --data "The payload" localhost:8080/echo

# with the headers
curl -i -X POST --data "The payload" localhost:8080/echo
```