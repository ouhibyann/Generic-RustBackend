# Generic RustBackend

This project is a Rust-based backend service that aggregates order book data for the BTC/USDT trading pair from multiple
exchanges.
It currently integrates with Kraken and Huobi using REST APIs and Binance via WebSocket.

## Requirements

- Rust (1.85.1)
- Cargo (1.85.1)
- Actix Web (4.10.2)
- Tokio (1.44.1) / tokio-tungstenite (0.26.2)
- Reqwest (0.12.15)
- Serde (1.0.219)
- async-trait (0.1.88)
- Config (0.15.11)

## REST API Endpoints:

### Configuration Management

Loads configuration from config.toml file to add new exchange, change endpoints, trading paris and others.
<br> You may override those settings via prefixed 'APP' env variables which allows you to test an exchange without
modifying any file first
```shell
- export APP_KRAKEN_TRADING_PAIR="XBTUSD"
- export APP_KRAKEN_ENDPOINT="https://api.kraken.com/0/public/Depth"
```
Currently exposed REST endpoints using Actix Web to retrieve:

- Kraken order book data (/kraken/mid_price)
- Huobi order book data (/huobi/mid_price)

```cargo run -- rest```

```shell
curl -X GET http://127.0.0.1:8080/kraken/mid_price
curl -X GET http://127.0.0.1:8080/huobi/mid_price
```

## WebSocket Mode:

Binance WebSocket integration can be activated via command-line arguments.

```cargo run -- ws```
