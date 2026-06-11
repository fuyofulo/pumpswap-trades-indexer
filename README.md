# PumpSwap Trades Indexer

Real-time indexer for PumpSwap trades on Solana, written in Rust.

It subscribes to PumpSwap program activity over Yellowstone gRPC, detects buy and sell instructions by their Anchor discriminators, extracts pool reserves, and computes live token prices.

## How it works

1. Subscribes to account updates and transactions for the PumpSwap pool program (`pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`) over Yellowstone gRPC at confirmed commitment.
2. Pool accounts are recognized by their discriminator. Base and quote mints are read from fixed byte offsets in the account data and cached in memory per pool.
3. Buy and sell instructions are matched by their Anchor discriminators against the PumpSwap IDL (`src/pumpswap.json`).
4. SOL and token reserves are taken from pre-transaction token balances, and price is computed both ways (SOL per token, tokens per SOL), guarding against zero reserves.
5. Detected trades, pools, and prices are printed to stdout with timestamps.

## Layout

- `src/main.rs` — entry point, env loading
- `src/yellowstone/mod.rs` — the worker: subscription loop, pool cache, instruction parsing, reserve extraction, price computation
- `src/yellowstone/client.rs` — TLS gRPC client with optional `x-token` auth
- `src/yellowstone/subscriptions.rs` — subscription request filters
- `src/pumpswap.json` — PumpSwap Anchor IDL, the reference for discriminators and account layouts

## Run

Set the environment (a `.env` file works):

```dotenv
YELLOWSTONE_ENDPOINT=http://<geyser-grpc-host>:10000
YELLOWSTONE_TOKEN=<auth-token>   # optional
```

Then:

```bash
cargo run --release
```

## Scope

- Output goes to stdout. There is no storage layer.
- One-shot connection: no retry or backoff on stream failure.
- Parsing is discriminator-level: trade amounts, fees, and slippage are not decoded.

## Stack

tokio, yellowstone-grpc-client, tonic, solana-sdk, futures, dotenv.
