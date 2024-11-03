# Websocket example for sending commands and parameters

## Run

```sh
cargo run --example server_example
```

## Client

### websocat

```sh
websocat ws://127.0.0.1:9001
```

Type:
`{"type": "Command", "data": {"rec": false}}`
