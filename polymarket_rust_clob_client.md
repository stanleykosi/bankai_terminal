Directory structure:
└── polymarket-rs-clob-client/
    ├── README.md
    ├── Cargo.toml
    ├── CHANGELOG.md
    ├── clippy.toml
    ├── LICENSE
    ├── rustfmt.toml
    ├── SECURITY.md
    ├── .pre-commit-config.yaml
    ├── benches/
    │   ├── clob_order_operations.rs
    │   ├── deserialize_clob.rs
    │   └── deserialize_websocket.rs
    ├── examples/
    │   ├── approvals.rs
    │   ├── bridge.rs
    │   ├── check_approvals.rs
    │   ├── ctf.rs
    │   ├── data.rs
    │   ├── rtds_crypto_prices.rs
    │   ├── clob/
    │   │   ├── async.rs
    │   │   ├── authenticated.rs
    │   │   ├── aws_authenticated.rs
    │   │   ├── builder_authenticated.rs
    │   │   ├── heartbeats.rs
    │   │   ├── streaming.rs
    │   │   ├── unauthenticated.rs
    │   │   ├── rfq/
    │   │   │   ├── quotes.rs
    │   │   │   └── requests.rs
    │   │   └── ws/
    │   │       ├── orderbook.rs
    │   │       ├── unsubscribe.rs
    │   │       └── user.rs
    │   └── gamma/
    │       ├── client.rs
    │       └── streaming.rs
    ├── src/
    │   ├── auth.rs
    │   ├── error.rs
    │   ├── lib.rs
    │   ├── serde_helpers.rs
    │   ├── types.rs
    │   ├── bridge/
    │   │   ├── client.rs
    │   │   ├── mod.rs
    │   │   └── types/
    │   │       ├── mod.rs
    │   │       ├── request.rs
    │   │       └── response.rs
    │   ├── clob/
    │   │   ├── mod.rs
    │   │   ├── order_builder.rs
    │   │   ├── types/
    │   │   │   ├── mod.rs
    │   │   │   ├── request.rs
    │   │   │   └── response.rs
    │   │   └── ws/
    │   │       ├── client.rs
    │   │       ├── interest.rs
    │   │       ├── mod.rs
    │   │       ├── subscription.rs
    │   │       └── types/
    │   │           ├── mod.rs
    │   │           ├── request.rs
    │   │           └── response.rs
    │   ├── ctf/
    │   │   ├── client.rs
    │   │   ├── error.rs
    │   │   ├── mod.rs
    │   │   └── types/
    │   │       ├── mod.rs
    │   │       ├── request.rs
    │   │       └── response.rs
    │   ├── data/
    │   │   ├── client.rs
    │   │   ├── mod.rs
    │   │   └── types/
    │   │       ├── mod.rs
    │   │       ├── request.rs
    │   │       └── response.rs
    │   ├── gamma/
    │   │   ├── client.rs
    │   │   ├── mod.rs
    │   │   └── types/
    │   │       ├── mod.rs
    │   │       ├── request.rs
    │   │       └── response.rs
    │   ├── rtds/
    │   │   ├── client.rs
    │   │   ├── error.rs
    │   │   ├── mod.rs
    │   │   ├── subscription.rs
    │   │   └── types/
    │   │       ├── mod.rs
    │   │       ├── request.rs
    │   │       └── response.rs
    │   └── ws/
    │       ├── config.rs
    │       ├── connection.rs
    │       ├── error.rs
    │       ├── mod.rs
    │       └── traits.rs
    ├── tests/
    │   ├── auth.rs
    │   ├── bridge.rs
    │   ├── ctf.rs
    │   ├── data.rs
    │   ├── rfq.rs
    │   └── common/
    │       └── mod.rs
    └── .github/
        ├── CODE_OF_CONDUCT.md
        ├── CODEOWNERS
        ├── CONTRIBUTING.md
        ├── dependabot.yaml
        └── workflows/
            ├── ci.yml
            ├── conventional-title.yml
            └── release-plz.yml


Files Content:

(Files content cropped to 300k characters, download full ingest to see more)
================================================
FILE: README.md
================================================
![Polymarket](assets/logo.png)

# Polymarket Rust Client

[![Crates.io](https://img.shields.io/crates/v/polymarket-client-sdk.svg)](https://crates.io/crates/polymarket-client-sdk)
[![CI](https://github.com/Polymarket/rs-clob-client/actions/workflows/ci.yml/badge.svg)](https://github.com/Polymarket/rs-clob-client/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Polymarket/rs-clob-client/graph/badge.svg?token=FW1BYWWFJ2)](https://codecov.io/gh/Polymarket/rs-clob-client)

An ergonomic Rust client for interacting with Polymarket services, primarily the Central Limit Order Book (CLOB).
This crate provides strongly typed request builders, authenticated endpoints, `alloy` support and more.

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [Feature Flags](#feature-flags)
- [Re-exported Types](#re-exported-types)
- [Examples](#examples)
  - [CLOB Client](#clob-client)
  - [WebSocket Streaming](#websocket-streaming)
  - [Optional APIs](#optional-apis)
- [Additional CLOB Capabilities](#additional-clob-capabilities)
- [Setting Token Allowances](#token-allowances)
- [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
- [Contributing](#contributing)
- [About Polymarket](#about-polymarket)

## Overview

- **Typed CLOB requests** (orders, trades, markets, balances, and more)
- **Dual authentication flows**
    - Normal authenticated flow
    - [Builder](https://docs.polymarket.com/developers/builders/builder-intro) authentication flow
- **Type-level state machine**
    - Prevents using authenticated endpoints before authenticating
    - Compile-time enforcement of correct transitions
- **Signer support** via `alloy::signers::Signer`
    - Including remote signers, e.g. AWS KMS
- **Zero-cost abstractions** — no dynamic dispatch in hot paths
- **Order builders** for easy construction & signing
- **Full `serde` support**
- **Async-first design** with `reqwest`


## Getting started

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
polymarket-client-sdk = "0.3"
```

or

```bash
cargo add polymarket-client-sdk
```

Then run any of the examples
```bash
cargo run --example unauthenticated
```

## Feature Flags

The crate is modular with optional features for different Polymarket APIs:

| Feature      | Description                                                                                                                                    |
|--------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `clob`       | Core CLOB client for order placement, market data, and authentication                                                                          |
| `tracing`    | Structured logging via [`tracing`](https://docs.rs/tracing) for HTTP requests, auth flows, and caching                                         |
| `ws`         | WebSocket client for real-time orderbook, price, and user event streaming                                                                      |
| `rtds`       | Real-time data streams for crypto prices (Binance, Chainlink) and comments                                                                     |
| `data`       | Data API client for positions, trades, leaderboards, and analytics                                                                             |
| `gamma`      | Gamma API client for market/event discovery, search, and metadata                                                                              |
| `bridge`     | Bridge API client for cross-chain deposits (EVM, Solana, Bitcoin)                                                                              |
| `rfq`        | RFQ API (within CLOB) for submitting and querying quotes                                                                                       |
| `heartbeats` | Clob feature that automatically sends heartbeat messages to the Polymarket server, if the client disconnects all open orders will be cancelled |
| `ctf`        | CTF API client to perform split/merge/redeem on binary and neg risk markets

Enable features in your `Cargo.toml`:

```toml
[dependencies]
polymarket-client-sdk = { version = "0.3", features = ["ws", "data"] }
```

## Re-exported Types

This SDK re-exports commonly used types from external crates so you don't need to add them to your `Cargo.toml`:

### From `types` module

```rust
use polymarket_client_sdk::types::{
    Address, ChainId, Signature, address,  // from alloy::primitives
    DateTime, NaiveDate, Utc,              // from chrono
    Decimal, dec,                          // from rust_decimal + rust_decimal_macros
};
```

### From `auth` module

```rust
use polymarket_client_sdk::auth::{
    LocalSigner, Signer,          // from alloy::signers (LocalSigner + trait)
    Uuid, ApiKey,                 // from uuid (ApiKey = Uuid)
    SecretString, ExposeSecret,   // from secrecy
    builder::Url,                 // from url (for remote builder config)
};
```

### From `error` module

```rust
use polymarket_client_sdk::error::{
    StatusCode, Method,           // from reqwest (for error inspection)
};
```

This allows you to work with the SDK without managing version compatibility for these common dependencies.

## Examples

See `examples/` for the complete set. Below are hand-picked examples for common use cases.

### CLOB Client

#### Unauthenticated client (read-only)
```rust,ignore
use polymarket_client_sdk::clob::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    let ok = client.ok().await?;
    println!("Ok: {ok}");

    Ok(())
}
```

#### Authenticated client

Set `POLYMARKET_PRIVATE_KEY` as an environment variable with your private key.

##### [EOA](https://www.binance.com/en/academy/glossary/externally-owned-account-eoa) wallets
If using MetaMask or hardware wallet, you must first set token allowances. See [Token Allowances](#token-allowances) section below.

```rust,ignore
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use polymarket_client_sdk::clob::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let ok = client.ok().await?;
    println!("Ok: {ok}");

    let api_keys = client.api_keys().await?;
    println!("API keys: {api_keys:?}");

    Ok(())
}
```

##### Proxy/Safe wallets
For proxy/Safe wallets, the funder address is **automatically derived** using CREATE2 from your signer's EOA address:

```rust,ignore
let client = Client::new("https://clob.polymarket.com", Config::default())?
    .authentication_builder(&signer)
    .signature_type(SignatureType::GnosisSafe)  // Funder auto-derived via CREATE2
    .authenticate()
    .await?;
```

The SDK computes the deterministic wallet address that Polymarket deploys for your EOA. This is the same address
shown on polymarket.com when you log in with a browser wallet.

If you need to override the derived address (e.g., for advanced use cases), you can explicitly provide it:

```rust,ignore
let client = Client::new("https://clob.polymarket.com", Config::default())?
    .authentication_builder(&signer)
    .funder(address!("<your-polymarket-wallet-address>"))
    .signature_type(SignatureType::GnosisSafe)
    .authenticate()
    .await?;
```

You can also derive these addresses manually:

```rust,ignore
use polymarket_client_sdk::{derive_safe_wallet, derive_proxy_wallet, POLYGON};

// For browser wallet users (GnosisSafe)
let safe_address = derive_safe_wallet(signer.address(), POLYGON);

// For Magic/email wallet users (Proxy)
let proxy_address = derive_proxy_wallet(signer.address(), POLYGON);
```

##### Funder Address
The **funder address** is the actual address that holds your funds on Polymarket. When using proxy wallets (email wallets
like Magic or browser extension wallets), the signing key differs from the address holding the funds. The SDK automatically
derives the correct funder address using CREATE2 when you specify `SignatureType::Proxy` or `SignatureType::GnosisSafe`.
You can override this with `.funder(address)` if needed.

##### Signature Types
The **signature_type** parameter tells the system how to verify your signatures:
- `signature_type=0` (default): Standard EOA (Externally Owned Account) signatures - includes MetaMask, hardware wallets,
   and any wallet where you control the private key directly
- `signature_type=1`: Email/Magic wallet signatures (delegated signing)
- `signature_type=2`: Browser wallet proxy signatures (when using a proxy contract, not direct wallet connections)

See [SignatureType](src/clob/types/mod.rs#L182) for more information.

##### Place a market order

```rust,ignore
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::clob::types::{Amount, OrderType, Side};
use polymarket_client_sdk::types::Decimal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let order = client
        .market_order()
        .token_id("<token-id>")
        .amount(Amount::usdc(Decimal::ONE_HUNDRED)?)
        .side(Side::Buy)
        .order_type(OrderType::FOK)
        .build()
        .await?;
    let signed_order = client.sign(&signer, order).await?;
    let response = client.post_order(signed_order).await?;
    println!("Order response: {:?}", response);

    Ok(())
}
```

##### Place a limit order

```rust,ignore
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::clob::types::Side;
use polymarket_client_sdk::types::Decimal;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let order = client
        .limit_order()
        .token_id("<token-id>")
        .size(Decimal::ONE_HUNDRED)
        .price(dec!(0.1))
        .side(Side::Buy)
        .build()
        .await?;
    let signed_order = client.sign(&signer, order).await?;
    let response = client.post_order(signed_order).await?;
    println!("Order response: {:?}", response);

    Ok(())
}
```

#### Builder-authenticated client

For institutional/third-party app integrations with remote signing:
```rust,ignore
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::auth::builder::Config as BuilderConfig;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::clob::types::SignatureType;
use polymarket_client_sdk::clob::types::request::TradesRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
    let builder_config = BuilderConfig::remote("http://localhost:3000/sign", None)?; // Or your signing server

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .signature_type(SignatureType::Proxy)  // Funder auto-derived via CREATE2
        .authenticate()
        .await?;

    let client = client.promote_to_builder(builder_config).await?;

    let ok = client.ok().await?;
    println!("Ok: {ok}");

    let api_keys = client.api_keys().await?;
    println!("API keys: {api_keys:?}");

    let builder_trades = client.builder_trades(&TradesRequest::default(), None).await?;
    println!("Builder trades: {builder_trades:?}");

    Ok(())
}
```

### WebSocket Streaming

Real-time orderbook and user event streaming. Requires the `ws` feature.

```toml
polymarket-client-sdk = { version = "0.3", features = ["ws"] }
```

```rust,ignore
use futures::StreamExt as _;
use polymarket_client_sdk::clob::ws::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    // Subscribe to orderbook updates for specific assets
    let asset_ids = vec!["<asset-id>".to_owned()];
    let stream = client.subscribe_orderbook(asset_ids)?;
    let mut stream = Box::pin(stream);

    while let Some(book_result) = stream.next().await {
        let book = book_result?;
        println!("Orderbook update for {}: {} bids, {} asks",
            book.asset_id, book.bids.len(), book.asks.len());
    }
    Ok(())
}
```

Available streams:
- `subscribe_orderbook()` - Bid/ask levels for assets
- `subscribe_prices()` - Price change events
- `subscribe_midpoints()` - Calculated midpoint prices
- `subscribe_orders()` - User order updates (authenticated)
- `subscribe_trades()` - User trade executions (authenticated)

See [`examples/clob/ws/`](examples/clob/ws/) for more WebSocket examples including authenticated user streams.

### Optional APIs

#### Data API
Trading analytics, positions, and leaderboards. Requires the `data` feature.

```rust,ignore
use polymarket_client_sdk::data::Client;
use polymarket_client_sdk::data::types::request::PositionsRequest;
use polymarket_client_sdk::types::address;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();
    let user = address!("0x0000000000000000000000000000000000000000"); // Your address

    let request = PositionsRequest::builder().user(user).limit(10)?.build();
    let positions = client.positions(&request).await?;
    println!("Open positions: {:?}", positions);
    Ok(())
}
```

See [`examples/data.rs`](examples/data.rs) for trades, leaderboards, activity, and more.

#### Gamma API
Market and event discovery. Requires the `gamma` feature.

```rust,ignore
use polymarket_client_sdk::gamma::Client;
use polymarket_client_sdk::gamma::types::request::{EventsRequest, SearchRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    // Find active events
    let request = EventsRequest::builder().active(true).limit(5).build();
    let events = client.events(&request).await?;
    println!("Found {} events", events.len());

    // Search for markets
    let search = SearchRequest::builder().q("bitcoin").build();
    let results = client.search(&search).await?;
    println!("Search results: {:?}", results);
    Ok(())
}
```

See [`examples/gamma.rs`](examples/gamma/client.rs) for tags, series, comments, and sports endpoints.

#### Bridge API
Cross-chain deposits from EVM chains, Solana, and Bitcoin. Requires the `bridge` feature.

```rust,ignore
use polymarket_client_sdk::bridge::Client;
use polymarket_client_sdk::bridge::types::DepositRequest;
use polymarket_client_sdk::types::address;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    // Get deposit addresses for your wallet
    let request = DepositRequest::builder()
        .address(address!("0x0000000000000000000000000000000000000000")) // Your address
        .build();
    let response = client.deposit(&request).await?;

    println!("EVM: {}", response.address.evm);
    println!("Solana: {}", response.address.svm);
    println!("Bitcoin: {}", response.address.btc);
    Ok(())
}
```

See [`examples/bridge.rs`](examples/bridge.rs) for supported assets and minimum deposits.

## Additional CLOB Capabilities

Beyond basic order placement, the CLOB client supports:

- **Rewards & Earnings** - Query maker rewards, daily earnings, and reward percentages
- **Streaming Pagination** - `stream_data()` for iterating through large result sets
- **Batch Operations** - `post_orders()` and `cancel_orders()` for multiple orders at once
- **Order Scoring** - Check if orders qualify for maker rewards
- **Notifications** - Manage trading notifications
- **Balance Management** - Query and refresh balance/allowance caches
- **Geoblock Detection** - Check if trading is available in your region

See [`examples/clob/authenticated.rs`](examples/clob/authenticated.rs) for comprehensive usage.

## Token Allowances

### Do I need to set allowances?
MetaMask and EOA users must set token allowances.
If you are using a proxy or [Safe](https://help.safe.global/en/articles/40869-what-is-safe)-type wallet, then you do not.

### What are allowances?
Think of allowances as permissions. Before Polymarket can move your funds to execute trades, you need to give the
exchange contracts permission to access your USDC and conditional tokens.

### Quick Setup
You need to approve two types of tokens:
1. **USDC** (for deposits and trading)
2. **Conditional Tokens** (the outcome tokens you trade)

Each needs approval for the exchange contracts to work properly.

### Setting Allowances
Use [examples/approvals.rs](examples/approvals.rs) to approve the right contracts. Run once to approve USDC. Then change
the `TOKEN_TO_APPROVE` and run for each conditional token.

**Pro tip**: You only need to set these once per wallet. After that, you can trade freely.

## Minimum Supported Rust Version (MSRV)

**MSRV: Rust [1.88](https://releases.rs/docs/1.88.0/)**

Older versions *may* compile, but are not supported.

This project aims to maintain compatibility with a Rust version that is at least six months old.

Version updates may occur more frequently than the policy guideline states if external forces require it. For example,
a CVE in a downstream dependency requiring an MSRV bump would be considered an acceptable reason to violate the six-month
guideline.


## Contributing
We encourage contributions from the community. Check out our [contributing guidelines](.github/CONTRIBUTING.md) for
instructions on how to contribute to this SDK.


## About Polymarket
[Polymarket](https://docs.polymarket.com/polymarket-learn/get-started/what-is-polymarket) is the world’s largest prediction market, allowing you to stay informed and profit from your knowledge by
betting on future events across various topics.
Studies show prediction markets are often more accurate than pundits because they combine news, polls, and expert
opinions into a single value that represents the market’s view of an event’s odds. Our markets reflect accurate, unbiased,
and real-time probabilities for the events that matter most to you. Markets seek truth.



================================================
FILE: Cargo.toml
================================================
[package]
name = "polymarket-client-sdk"
description = "Polymarket CLOB (Central Limit Order Book) API client SDK"
version = "0.4.1"
authors = [
    "Polymarket Engineering <engineering@polymarket.com>",
    "Chaz Byrnes <chaz@polymarket.com>",
]
readme = "README.md"
repository = "https://github.com/polymarket/rs-clob-client"
license = "MIT"
keywords = ["polymarket", "clob", "orderbook", "trading", "prediction-market"]
categories = [
    "api-bindings",
    "cryptography::cryptocurrencies",
    "finance",
    "web-programming::http-client",
]
edition = "2024"
rust-version = "1.88.0" # MSRV

[package.metadata.docs.rs]
all-features = true

[features]
default = []
clob = []
data = []
gamma = []
bridge = []
ctf = ["alloy/contract", "alloy/providers"]
rfq = []
tracing = ["dep:tracing", "dep:serde_ignored", "dep:serde_path_to_error"]
ws = ["dep:backoff", "dep:bitflags", "dep:tokio", "dep:tokio-tungstenite"]
rtds = ["dep:backoff", "dep:tokio", "dep:tokio-tungstenite"]
heartbeats = ["dep:tokio", "dep:tokio-util"]

[dependencies]
alloy = { version = "1.4.3", default-features = false, features = [
    "dyn-abi",
    "reqwest",
    "reqwest-rustls-tls",
    "serde",
    "signer-local",
    "signers",
    "sol-types"
] }
async-stream = "0.3.6"
async-trait = "0.1.89"
backoff = { version = "0.4.0", optional = true }
base64 = "0.22.1"
bitflags = { version = "2.10.0", optional = true }
bon = "3.8.2"
chrono = { version = "0.4.43", features = ["serde"] }
dashmap = "6.1.0"
futures = "0.3.31"
hmac = "0.12.1"
phf = { version = "0.13.1", features = ["macros"] }
rand = "0.9.2"
reqwest = { version = "0.13.1", features = ["json", "query", "rustls"] }
rust_decimal = { version = "1.40.0", features = ["serde"] }
rust_decimal_macros = "1.40.0"
secrecy = { version = "0.10", features = ["serde"] }
serde = "1.0.228"
serde_html_form = { version = "0.4" }
serde_ignored = { version = "0.1", optional = true }
serde_json = "1.0.149"
serde_path_to_error = { version = "0.1", optional = true }
serde_repr = "0.1.20"
serde_with = { version = "3.16.1", features = ["chrono_0_4", "json"] }
sha2 = "0.10.9"
strum_macros = "0.27.2"
tokio = { version = "1.49.0", features = ["rt-multi-thread", "macros"], optional = true }
tokio-tungstenite = { version = "0.28.0", features = ["rustls-tls-native-roots"], optional = true }
tokio-util = { version = "0.7.18", optional = true }
tracing = { version = "0.1", optional = true }
url = "2.5.8"
uuid = { version = "1.19.0", features = ["serde", "v4", "v7"] }

[dev-dependencies]
alloy = { version = "1.4.3", default-features = false, features = [
    "contract",
    "providers",
    "reqwest",
    "signer-aws",
    "signer-local",
] }
anyhow = "1.0.100"
aws-config = "1.8.12"
aws-sdk-kms = "1.98.0"
criterion = { version = "0.8.1", features = ["html_reports"] }
futures-util = "0.3.31"
httpmock = "0.8.2"
tokio = { version = "1.49.0", features = ["rt-multi-thread", "macros"] }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[[example]]
name = "async"
path = "examples/clob/async.rs"
required-features = ["clob"]

[[example]]
name = "authenticated"
path = "examples/clob/authenticated.rs"
required-features = ["clob"]

[[example]]
name = "aws_authenticated"
path = "examples/clob/aws_authenticated.rs"
required-features = ["clob"]

[[example]]
name = "builder_authenticated"
path = "examples/clob/builder_authenticated.rs"
required-features = ["clob"]

[[example]]
name = "heartbeats"
path = "examples/clob/heartbeats.rs"
required-features = ["clob", "heartbeats", "tracing"]

[[example]]
name = "streaming"
path = "examples/clob/streaming.rs"
required-features = ["clob", "tracing"]

[[example]]
name = "unauthenticated"
path = "examples/clob/unauthenticated.rs"
required-features = ["clob", "tracing"]

[[example]]
name = "approvals"
path = "examples/approvals.rs"
required-features = ["tracing"]

[[example]]
name = "check_approvals"
path = "examples/check_approvals.rs"
required-features = ["tracing"]

[[example]]
name = "ctf"
path = "examples/ctf.rs"
required-features = ["ctf", "tracing"]

[[example]]
name = "data"
path = "examples/data.rs"
required-features = ["data", "tracing"]

[[example]]
name = "gamma"
path = "examples/gamma/client.rs"
required-features = ["gamma", "tracing"]

[[example]]
name = "gamma_streaming"
path = "examples/gamma/streaming.rs"
required-features = ["gamma", "tracing"]

[[example]]
name = "bridge"
path = "examples/bridge.rs"
required-features = ["bridge", "tracing"]

[[example]]
name = "websocket_orderbook"
path = "examples/clob/ws/orderbook.rs"
required-features = ["clob", "ws"]

[[example]]
name = "websocket_user"
path = "examples/clob/ws/user.rs"
required-features = ["clob", "ws"]

[[example]]
name = "rfq_quotes"
path = "examples/clob/rfq/quotes.rs"
required-features = ["clob", "rfq"]

[[example]]
name = "rfq_requests"
path = "examples/clob/rfq/requests.rs"
required-features = ["clob", "rfq"]

[[example]]
name = "rtds_crypto_prices"
path = "examples/rtds_crypto_prices.rs"
required-features = ["rtds", "tracing"]

[[example]]
name = "websocket_unsubscribe"
path = "examples/clob/ws/unsubscribe.rs"
required-features = ["clob", "ws"]

[[bench]]
name = "deserialize_clob"
harness = false
required-features = ["clob"]

[[bench]]
name = "deserialize_websocket"
harness = false
required-features = ["clob", "ws"]

[[bench]]
name = "clob_order_operations"
harness = false
required-features = ["clob"]

# https://rust-lang.github.io/rust-clippy/master/index.html?versions=lte%3A88
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }

allow_attributes = "warn"
allow_attributes_without_reason = "warn"
assertions_on_result_states = "warn"
clone_on_ref_ptr = "warn"
create_dir = "warn"
dbg_macro = "warn"
doc_include_without_cfg = "warn"
empty_enum_variants_with_brackets = "warn"
empty_structs_with_brackets = "warn"
exhaustive_enums = "warn"
exhaustive_structs = "warn"
exit = "warn"
filetype_is_file = "warn"
float_arithmetic = "warn"
get_unwrap = "warn"
if_then_some_else_none = "warn"
impl_trait_in_params = "warn"
infinite_loop = "warn"
large_include_file = "warn"
let_underscore_untyped = "warn"
map_err_ignore = "warn"
map_with_unused_argument_over_ranges = "warn"
missing_assert_message = "warn"
missing_errors_doc = "allow"
module_name_repetitions = "warn"
multiple_crate_versions = "allow"
multiple_inherent_impl = "warn"
mutex_atomic = "warn"
mutex_integer = "warn"
needless_raw_strings = "warn"
non_zero_suggestions = "warn"
pathbuf_init_then_push = "warn"
print_stderr = "warn"
print_stdout = "warn"
pub_without_shorthand = "warn"
rc_buffer = "warn"
redundant_test_prefix = "warn"
redundant_type_annotations = "warn"
ref_patterns = "warn"
renamed_function_params = "warn"
rest_pat_in_fully_bound_structs = "warn"
return_and_then = "warn"
same_name_method = "warn"
self_named_module_files = "warn"
similar_names = "allow"
single_char_lifetime_names = "warn"
str_to_string = "warn"
string_add = "warn"
string_slice = "warn"
todo = "warn"
too_many_lines = "allow"
try_err = "warn"
undocumented_unsafe_blocks = "warn"
unneeded_field_pattern = "warn"
unseparated_literal_suffix = "warn"
unused_trait_names = "warn"
unwrap_used = "warn"

[profile.bench]
lto = "thin"        # Link-Time Optimization: enables cross-crate inlining
codegen-units = 1   # Single codegen unit allows more aggressive optimizations



================================================
FILE: CHANGELOG.md
================================================
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/Polymarket/rs-clob-client/compare/v0.4.0...v0.4.1) - 2026-01-14

### Added

- *(clob)* add last_trade_price field to OrderBookSummaryResponse ([#174](https://github.com/Polymarket/rs-clob-client/pull/174))

### Fixed

- *(ws)* prevent TOCTOU race in subscription unsubscribe ([#190](https://github.com/Polymarket/rs-clob-client/pull/190))
- *(rtds)* prevent race condition in subscription check ([#191](https://github.com/Polymarket/rs-clob-client/pull/191))
- *(ws)* preserve custom_feature_enabled flag on reconnect ([#186](https://github.com/Polymarket/rs-clob-client/pull/186))
- *(clob)* usage of ampersand before and without question mark ([#189](https://github.com/Polymarket/rs-clob-client/pull/189))
- *(data)* make Activity.condition_id optional ([#173](https://github.com/Polymarket/rs-clob-client/pull/173))

### Other

- *(ws)* eliminate double JSON parsing in parse_if_interested ([#182](https://github.com/Polymarket/rs-clob-client/pull/182))
- *(clob/ws)* use channel map for laziness instead of once_cell ([#183](https://github.com/Polymarket/rs-clob-client/pull/183))
- *(cargo)* add release profile optimizations ([#180](https://github.com/Polymarket/rs-clob-client/pull/180))
- *(clob)* optimize SignedOrder serialization ([#181](https://github.com/Polymarket/rs-clob-client/pull/181))
- *(cargo)* bump alloy from 1.3.0 to 1.4.0 ([#178](https://github.com/Polymarket/rs-clob-client/pull/178))
- *(cargo)* bump bon from 3.8.1 to 3.8.2 ([#177](https://github.com/Polymarket/rs-clob-client/pull/177))
- *(cargo)* bump serde_json from 1.0.148 to 1.0.149 ([#179](https://github.com/Polymarket/rs-clob-client/pull/179))
- *(cargo)* bump url from 2.5.7 to 2.5.8 ([#176](https://github.com/Polymarket/rs-clob-client/pull/176))
- *(examples)* update WebSocket examples to use tracing ([#170](https://github.com/Polymarket/rs-clob-client/pull/170))
- *(examples)* update RFQ examples to use tracing ([#169](https://github.com/Polymarket/rs-clob-client/pull/169))
- *(examples)* update CLOB examples to use tracing ([#168](https://github.com/Polymarket/rs-clob-client/pull/168))

## [0.4.0](https://github.com/Polymarket/rs-clob-client/compare/v0.3.3...v0.4.0) - 2026-01-12

### Added

- *(clob)* add cache setter methods to prewarm market data ([#153](https://github.com/Polymarket/rs-clob-client/pull/153))
- *(bridge)* improve bridge type safety ([#151](https://github.com/Polymarket/rs-clob-client/pull/151))
- *(gamma)* convert neg_risk_market_id and neg_risk_request_id to B256 ([#143](https://github.com/Polymarket/rs-clob-client/pull/143))
- *(gamma)* convert question_id fields to B256 type ([#142](https://github.com/Polymarket/rs-clob-client/pull/142))
- *(clob)* clob typed b256 address ([#139](https://github.com/Polymarket/rs-clob-client/pull/139))
- *(clob)* add clob feature flag for optional CLOB compilation ([#135](https://github.com/Polymarket/rs-clob-client/pull/135))
- *(tracing)* add serde_path_to_error for detailed deserialization on errors ([#140](https://github.com/Polymarket/rs-clob-client/pull/140))
- *(data)* use typed Address and B256 for hex string fields, update data example ([#132](https://github.com/Polymarket/rs-clob-client/pull/132))
- *(gamma)* use typed Address and B256 for hex string fields ([#126](https://github.com/Polymarket/rs-clob-client/pull/126))
- *(ctf)* add CTF client/operations ([#82](https://github.com/Polymarket/rs-clob-client/pull/82))
- add Unknown(String) variant to all enums for forward compatibility ([#124](https://github.com/Polymarket/rs-clob-client/pull/124))
- add subscribe_last_trade_price websocket method ([#121](https://github.com/Polymarket/rs-clob-client/pull/121))
- support post-only orders ([#115](https://github.com/Polymarket/rs-clob-client/pull/115))
- *(heartbeats)* [**breaking**] add heartbeats ([#113](https://github.com/Polymarket/rs-clob-client/pull/113))

### Fixed

- *(rfq)* url path fixes ([#162](https://github.com/Polymarket/rs-clob-client/pull/162))
- *(gamma)* use repeated query params for array fields ([#148](https://github.com/Polymarket/rs-clob-client/pull/148))
- *(rtds)* serialize Chainlink filters as JSON string ([#136](https://github.com/Polymarket/rs-clob-client/pull/136)) ([#137](https://github.com/Polymarket/rs-clob-client/pull/137))
- add missing makerRebatesFeeShareBps field to Market struct ([#130](https://github.com/Polymarket/rs-clob-client/pull/130))
- add MakerRebate enum option to ActivityType ([#127](https://github.com/Polymarket/rs-clob-client/pull/127))
- suppress unused variable warnings in tracing cfg blocks ([#125](https://github.com/Polymarket/rs-clob-client/pull/125))
- add Yield enum option to ActivityType ([#122](https://github.com/Polymarket/rs-clob-client/pull/122))

### Other

- *(rtds)* [**breaking**] well-type RTDS structs ([#167](https://github.com/Polymarket/rs-clob-client/pull/167))
- *(gamma)* [**breaking**] well-type structs ([#166](https://github.com/Polymarket/rs-clob-client/pull/166))
- *(clob/rfq)* well-type structs ([#163](https://github.com/Polymarket/rs-clob-client/pull/163))
- *(data)* well-type data types ([#159](https://github.com/Polymarket/rs-clob-client/pull/159))
- *(gamma,rtds)* add Builder to non_exhaustive structs ([#160](https://github.com/Polymarket/rs-clob-client/pull/160))
- *(ctf)* add Builder to non_exhaustive response structs ([#161](https://github.com/Polymarket/rs-clob-client/pull/161))
- *(ws)* [**breaking**] well-type ws structs ([#156](https://github.com/Polymarket/rs-clob-client/pull/156))
- add benchmarks for CLOB and WebSocket types/operations ([#155](https://github.com/Polymarket/rs-clob-client/pull/155))
- *(clob)* [**breaking**] well-type requests/responses with U256 ([#150](https://github.com/Polymarket/rs-clob-client/pull/150))
- update rustdocs ([#134](https://github.com/Polymarket/rs-clob-client/pull/134))
- *(ws)* extract WsError to shared ws module ([#131](https://github.com/Polymarket/rs-clob-client/pull/131))
- update license ([#128](https://github.com/Polymarket/rs-clob-client/pull/128))
- update builder method doc comment ([#129](https://github.com/Polymarket/rs-clob-client/pull/129))

## [0.3.3](https://github.com/Polymarket/rs-clob-client/compare/v0.3.2...v0.3.3) - 2026-01-06

### Added

- *(auth)* auto derive funder address ([#99](https://github.com/Polymarket/rs-clob-client/pull/99))
- *(rfq)* add standalone RFQ API client ([#76](https://github.com/Polymarket/rs-clob-client/pull/76))
- *(types)* re-export commonly used external types for API ergonomics ([#102](https://github.com/Polymarket/rs-clob-client/pull/102))

### Fixed

- add missing cumulativeMarkets field to Event struct ([#108](https://github.com/Polymarket/rs-clob-client/pull/108))

### Other

- *(cargo)* bump reqwest from 0.12.28 to 0.13.1 ([#103](https://github.com/Polymarket/rs-clob-client/pull/103))
- *(ws)* common connection for clob ws and rtds ([#97](https://github.com/Polymarket/rs-clob-client/pull/97))
- *(cargo)* bump tokio from 1.48.0 to 1.49.0 ([#104](https://github.com/Polymarket/rs-clob-client/pull/104))
- *(examples)* improve approvals example with tracing ([#101](https://github.com/Polymarket/rs-clob-client/pull/101))
- *(examples)* improve bridge example with tracing ([#100](https://github.com/Polymarket/rs-clob-client/pull/100))
- *(examples)* improve rtds example with tracing and dynamic IDs ([#94](https://github.com/Polymarket/rs-clob-client/pull/94))
- *(examples)* improve gamma example with tracing and dynamic IDs ([#93](https://github.com/Polymarket/rs-clob-client/pull/93))

## [0.3.2](https://github.com/Polymarket/rs-clob-client/compare/v0.3.1...v0.3.2) - 2026-01-04

### Added

- add unknown field warnings for API responses ([#47](https://github.com/Polymarket/rs-clob-client/pull/47))
- *(ws)* add custom feature message types and subscription support ([#79](https://github.com/Polymarket/rs-clob-client/pull/79))

### Fixed

- *(ws)* defer WebSocket connection until first subscription ([#90](https://github.com/Polymarket/rs-clob-client/pull/90))
- *(types)* improve type handling and API compatibility ([#92](https://github.com/Polymarket/rs-clob-client/pull/92))
- add serde aliases for API response field variants ([#88](https://github.com/Polymarket/rs-clob-client/pull/88))
- *(data)* add missing fields to Position and Holder types ([#85](https://github.com/Polymarket/rs-clob-client/pull/85))
- *(gamma)* add missing fields to response types ([#87](https://github.com/Polymarket/rs-clob-client/pull/87))
- *(deser_warn)* show full JSON values in unknown field warnings ([#86](https://github.com/Polymarket/rs-clob-client/pull/86))
- handle order_type field in OpenOrderResponse ([#81](https://github.com/Polymarket/rs-clob-client/pull/81))

### Other

- update README with new features and examples ([#80](https://github.com/Polymarket/rs-clob-client/pull/80))

## [0.3.1](https://github.com/Polymarket/rs-clob-client/compare/v0.3.0...v0.3.1) - 2025-12-31

### Added

- *(ws)* add unsubscribe support with reference counting ([#70](https://github.com/Polymarket/rs-clob-client/pull/70))
- *(auth)* add secret and passphrase accessors to Credentials ([#78](https://github.com/Polymarket/rs-clob-client/pull/78))
- add RTDS (Real-Time Data Socket) client ([#56](https://github.com/Polymarket/rs-clob-client/pull/56))

### Fixed

- *(clob)* align API implementation with OpenAPI spec ([#72](https://github.com/Polymarket/rs-clob-client/pull/72))

### Other

- *(auth)* migrate from sec to secrecy crate ([#75](https://github.com/Polymarket/rs-clob-client/pull/75))
- use re-exported types ([#74](https://github.com/Polymarket/rs-clob-client/pull/74))

## [0.3.0](https://github.com/Polymarket/rs-clob-client/compare/v0.2.1...v0.3.0) - 2025-12-31

### Added

- *(auth)* add key() getter to Credentials ([#69](https://github.com/Polymarket/rs-clob-client/pull/69))
- add geographic restrictions check ([#63](https://github.com/Polymarket/rs-clob-client/pull/63))
- add bridge API client ([#55](https://github.com/Polymarket/rs-clob-client/pull/55))

### Fixed

- *(gamma)* use repeated query params for clob_token_ids ([#65](https://github.com/Polymarket/rs-clob-client/pull/65))
- correct data example required-features name ([#68](https://github.com/Polymarket/rs-clob-client/pull/68))
- *(clob)* allow market orders to supply price ([#67](https://github.com/Polymarket/rs-clob-client/pull/67))
- add CTF Exchange approval to approvals example ([#45](https://github.com/Polymarket/rs-clob-client/pull/45))

### Other

- [**breaking**] ws types ([#52](https://github.com/Polymarket/rs-clob-client/pull/52))
- consolidate request and query params ([#64](https://github.com/Polymarket/rs-clob-client/pull/64))
- [**breaking**] rescope data types and rename feature ([#62](https://github.com/Polymarket/rs-clob-client/pull/62))
- [**breaking**] rescope gamma types ([#61](https://github.com/Polymarket/rs-clob-client/pull/61))
- [**breaking**] scope clob types into request/response ([#60](https://github.com/Polymarket/rs-clob-client/pull/60))
- [**breaking**] WS cleanup ([#58](https://github.com/Polymarket/rs-clob-client/pull/58))
- [**breaking**] minor cleanup ([#57](https://github.com/Polymarket/rs-clob-client/pull/57))

## [0.2.1](https://github.com/Polymarket/rs-clob-client/compare/v0.2.0...v0.2.1) - 2025-12-29

### Added

- complete gamma client ([#40](https://github.com/Polymarket/rs-clob-client/pull/40))
- add data-api client ([#39](https://github.com/Polymarket/rs-clob-client/pull/39))

### Fixed

- use TryFrom for TickSize to avoid panic on unknown values ([#43](https://github.com/Polymarket/rs-clob-client/pull/43))

### Other

- *(cargo)* bump tracing from 0.1.41 to 0.1.44 ([#49](https://github.com/Polymarket/rs-clob-client/pull/49))
- *(cargo)* bump serde_json from 1.0.146 to 1.0.148 ([#51](https://github.com/Polymarket/rs-clob-client/pull/51))
- *(cargo)* bump alloy from 1.1.3 to 1.2.1 ([#50](https://github.com/Polymarket/rs-clob-client/pull/50))
- *(cargo)* bump reqwest from 0.12.27 to 0.12.28 ([#48](https://github.com/Polymarket/rs-clob-client/pull/48))

## [0.2.0](https://github.com/Polymarket/rs-clob-client/compare/v0.1.2...v0.2.0) - 2025-12-27

### Added

- WebSocket client for real-time market and user data ([#26](https://github.com/Polymarket/rs-clob-client/pull/26))

### Other

- [**breaking**] change from `derive_builder` to `bon` ([#41](https://github.com/Polymarket/rs-clob-client/pull/41))

## [0.1.2](https://github.com/Polymarket/rs-clob-client/compare/v0.1.1...v0.1.2) - 2025-12-23

### Added

- add optional tracing instrumentation ([#38](https://github.com/Polymarket/rs-clob-client/pull/38))
- add gamma client ([#31](https://github.com/Polymarket/rs-clob-client/pull/31))
- support share-denominated market orders ([#29](https://github.com/Polymarket/rs-clob-client/pull/29))

### Fixed

- mask salt for limit orders ([#30](https://github.com/Polymarket/rs-clob-client/pull/30))
- mask salt to 53 bits ([#27](https://github.com/Polymarket/rs-clob-client/pull/27))

### Other

- rescope clients with gamma feature ([#37](https://github.com/Polymarket/rs-clob-client/pull/37))
- Replacing `status: String` to enum ([#36](https://github.com/Polymarket/rs-clob-client/pull/36))
- *(cargo)* bump serde_json from 1.0.145 to 1.0.146 ([#34](https://github.com/Polymarket/rs-clob-client/pull/34))
- *(cargo)* bump reqwest from 0.12.26 to 0.12.27 ([#33](https://github.com/Polymarket/rs-clob-client/pull/33))
- *(gha)* bump dtolnay/rust-toolchain from 0b1efabc08b657293548b77fb76cc02d26091c7e to f7ccc83f9ed1e5b9c81d8a67d7ad1a747e22a561 ([#32](https://github.com/Polymarket/rs-clob-client/pull/32))

## [0.1.1](https://github.com/Polymarket/rs-clob-client/compare/v0.1.0...v0.1.1) - 2025-12-17

### Fixed

- remove signer from Authenticated ([#22](https://github.com/Polymarket/rs-clob-client/pull/22))

### Other

- enable release-plz ([#23](https://github.com/Polymarket/rs-clob-client/pull/23))
- add crates.io badge ([#20](https://github.com/Polymarket/rs-clob-client/pull/20))



================================================
FILE: clippy.toml
================================================
allow-unwrap-in-tests = true



================================================
FILE: LICENSE
================================================
MIT License

Copyright (c) 2025-2026 Polymarket

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.



================================================
FILE: rustfmt.toml
================================================
reorder_imports = true
reorder_modules = true

group_imports = "StdExternalCrate"



================================================
FILE: SECURITY.md
================================================
### Security

If you believe you’ve found a security vulnerability, please email security@polymarket.com. Do not open a public issue.

Include:
- A description of the issue and potential impact
- Steps to reproduce or a minimal proof of concept
- Any relevant logs or environment details

We will acknowledge receipt, investigate, and provide guidance on next steps.



================================================
FILE: .pre-commit-config.yaml
================================================
# See https://pre-commit.com for more information
# See https://pre-commit.com/hooks.html for more hooks
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v6.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-added-large-files
  - repo: https://github.com/DevinR528/cargo-sort
    rev: v2.0.2
    hooks:
      - id: cargo-sort
  - repo: local
    hooks:
      - id: fmt
        name: fmt
        entry: cargo +nightly fmt --all -- --check
        language: system
        types: [ rust ]
        pass_filenames: false
      - id: clippy
        name: clippy
        entry: cargo clippy --workspace --all-features --all-targets -- -D warnings
        language: system
        types: [ rust ]
        pass_filenames: false



================================================
FILE: benches/clob_order_operations.rs
================================================
//! Benchmarks for CLOB order creation and signing operations
//!
//! This benchmark suite focuses on the hot path operations for creating and signing orders:
//! - Limit order building (price validation, decimal conversion, order struct creation)
//! - Order signing (EIP-712 domain construction and cryptographic signing)
//! - Order serialization (converting `SignedOrder` to JSON for API submission)

use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::PrivateKeySigner;
use criterion::{Criterion, criterion_group, criterion_main};
use polymarket_client_sdk::POLYGON;
use polymarket_client_sdk::auth::Normal;
use polymarket_client_sdk::auth::state::Authenticated;
use polymarket_client_sdk::clob::Client;
use polymarket_client_sdk::clob::types::{OrderType, Side, TickSize};
use polymarket_client_sdk::types::{Decimal, U256};
use rust_decimal_macros::dec;

const TOKEN_ID: &str =
    "15871154585880608648532107628464183779895785213830018178010423617714102767076";

// Dummy private key for benchmarking (DO NOT USE IN PRODUCTION)
const BENCH_PRIVATE_KEY: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000001";

/// Helper to create an authenticated client with cached tick size and fee rate
async fn setup_client() -> (Client<Authenticated<Normal>>, PrivateKeySigner) {
    let token_id = U256::from_str(TOKEN_ID).expect("valid token ID");
    let signer = PrivateKeySigner::from_str(BENCH_PRIVATE_KEY)
        .expect("valid key")
        .with_chain_id(Some(POLYGON));

    let client = Client::default()
        .authentication_builder(&signer)
        .authenticate()
        .await
        .expect("authentication succeeds");

    // Pre-cache tick size and fee rate to avoid HTTP requests during benchmarking
    client.set_tick_size(token_id, TickSize::Hundredth);
    client.set_fee_rate_bps(token_id, 0);
    client.set_neg_risk(token_id, false);

    (client, signer)
}

/// Benchmark limit order building
fn bench_order_building(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("runtime");
    let (client, _) = runtime.block_on(setup_client());
    let token_id = U256::from_str(TOKEN_ID).expect("valid token ID");

    let mut group = c.benchmark_group("clob_order_operations/order_building");

    group.bench_function("BUY", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let order_builder = client
                    .limit_order()
                    .order_type(OrderType::GTC)
                    .token_id(token_id)
                    .side(Side::Buy)
                    .price(dec!(0.50))
                    .size(Decimal::ONE_HUNDRED);

                std::hint::black_box(order_builder.build().await.expect("build succeeds"))
            })
        });
    });

    group.bench_function("SELL", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let order_builder = client
                    .limit_order()
                    .order_type(OrderType::GTC)
                    .token_id(token_id)
                    .side(Side::Sell)
                    .price(dec!(0.50))
                    .size(Decimal::ONE_HUNDRED);

                std::hint::black_box(order_builder.build().await.expect("build succeeds"))
            })
        });
    });

    group.finish();
}

/// Benchmark order signing
fn bench_order_signing(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("runtime");
    let (client, signer) = runtime.block_on(setup_client());
    let token_id = U256::from_str(TOKEN_ID).expect("valid token ID");

    let mut group = c.benchmark_group("clob_order_operations/order_signing");

    let signable_order = runtime.block_on(async {
        client
            .limit_order()
            .token_id(token_id)
            .side(Side::Buy)
            .price(dec!(0.50))
            .size(dec!(100.0))
            .build()
            .await
            .expect("build succeeds")
    });

    group.bench_function("limit_order", |b| {
        b.iter(|| {
            runtime.block_on(async {
                std::hint::black_box(
                    client
                        .sign(&signer, signable_order.clone())
                        .await
                        .expect("sign succeeds"),
                )
            })
        });
    });

    group.finish();
}

/// Benchmark order serialization
fn bench_order_serializing(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("runtime");
    let token_id = U256::from_str(TOKEN_ID).expect("valid token ID");

    let mut group = c.benchmark_group("clob_order_operations/order_serializing");

    let signed_order = runtime.block_on(async {
        let (client, signer) = setup_client().await;

        let signable = client
            .limit_order()
            .token_id(token_id)
            .side(Side::Buy)
            .price(dec!(0.50))
            .size(dec!(100.0))
            .build()
            .await
            .expect("build succeeds");

        client.sign(&signer, signable).await.expect("sign succeeds")
    });

    group.bench_function("to_json", |b| {
        b.iter(|| {
            let json = serde_json::to_string(std::hint::black_box(&signed_order))
                .expect("serialization succeeds");
            std::hint::black_box(json)
        });
    });

    group.finish();
}

criterion_group!(
    order_operations_benches,
    bench_order_building,
    bench_order_signing,
    bench_order_serializing,
);

criterion_main!(order_operations_benches);



================================================
FILE: benches/deserialize_clob.rs
================================================
/// Comprehensive benchmarks for CLOB API deserialization.
///
/// This module benchmarks ALL deserialization types for the Central Limit Order Book API,
/// with special focus on hot trading paths: order placement, orderbook updates, trades,
/// and cancellations.
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use polymarket_client_sdk::clob::types::response::{
    ApiKeysResponse, BalanceAllowanceResponse, BanStatusResponse, CancelOrdersResponse,
    FeeRateResponse, LastTradePriceResponse, MarketResponse, MidpointResponse, NegRiskResponse,
    NotificationResponse, OpenOrderResponse, OrderBookSummaryResponse, PostOrderResponse,
    PriceHistoryResponse, PriceResponse, SpreadResponse, TickSizeResponse, TradeResponse,
};

fn bench_orderbook(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/orderbook");

    let orderbook_small = r#"{
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "timestamp": "1234567890123",
        "bids": [{"price": "0.55", "size": "100.0"}],
        "asks": [{"price": "0.56", "size": "150.0"}],
        "min_order_size": "10.0",
        "neg_risk": false,
        "tick_size": "0.01"
    }"#;

    let orderbook_medium = r#"{
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "timestamp": "1234567890123",
        "hash": "abc123def456",
        "bids": [
            {"price": "0.55", "size": "100.0"},
            {"price": "0.54", "size": "200.0"},
            {"price": "0.53", "size": "300.0"},
            {"price": "0.52", "size": "400.0"},
            {"price": "0.51", "size": "500.0"},
            {"price": "0.50", "size": "600.0"},
            {"price": "0.49", "size": "700.0"},
            {"price": "0.48", "size": "800.0"},
            {"price": "0.47", "size": "900.0"},
            {"price": "0.46", "size": "1000.0"}
        ],
        "asks": [
            {"price": "0.56", "size": "150.0"},
            {"price": "0.57", "size": "175.0"},
            {"price": "0.58", "size": "200.0"},
            {"price": "0.59", "size": "225.0"},
            {"price": "0.60", "size": "250.0"},
            {"price": "0.61", "size": "275.0"},
            {"price": "0.62", "size": "300.0"},
            {"price": "0.63", "size": "325.0"},
            {"price": "0.64", "size": "350.0"},
            {"price": "0.65", "size": "375.0"}
        ],
        "min_order_size": "10.0",
        "neg_risk": false,
        "tick_size": "0.01"
    }"#;

    // Benchmark with different orderbook depths
    for (name, json) in [
        ("small_1_level", orderbook_small),
        ("medium_10_levels", orderbook_medium),
    ] {
        group.throughput(Throughput::Bytes(json.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("OrderBookSummaryResponse", name),
            &json,
            |b, json| {
                b.iter(|| {
                    let _: OrderBookSummaryResponse =
                        serde_json::from_str(std::hint::black_box(json))
                            .expect("Deserialization should succeed");
                });
            },
        );
    }

    group.finish();
}

fn bench_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/orders");

    let post_order = r#"{
        "makingAmount": "100.5",
        "takingAmount": "55.275",
        "orderID": "0x1234567890abcdef",
        "status": "LIVE",
        "success": true,
        "transactionsHashes": ["0x0000000000000000000000000000000000000000000000000000000000000001"],
        "trade_ids": ["trade_123", "trade_456"]
    }"#;
    group.throughput(Throughput::Bytes(post_order.len() as u64));
    group.bench_function("PostOrderResponse", |b| {
        b.iter(|| {
            let _: PostOrderResponse = serde_json::from_str(std::hint::black_box(post_order))
                .expect("Deserialization should succeed");
        });
    });

    let open_order = r#"{
        "id": "0x1234567890abcdef",
        "status": "LIVE",
        "owner": "550e8400-e29b-41d4-a716-446655440000",
        "maker_address": "0x1234567890123456789012345678901234567890",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "original_size": "100.0",
        "size_matched": "25.0",
        "price": "0.55",
        "associate_trades": ["trade_123"],
        "outcome": "Yes",
        "created_at": 1234567890,
        "expiration": "1234567890",
        "order_type": "GTC"
    }"#;
    group.throughput(Throughput::Bytes(open_order.len() as u64));
    group.bench_function("OpenOrderResponse", |b| {
        b.iter(|| {
            let _: OpenOrderResponse = serde_json::from_str(std::hint::black_box(open_order))
                .expect("Deserialization should succeed");
        });
    });

    let trade = r#"{
        "id": "trade_123",
        "taker_order_id": "0xabcdef1234567890",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "size": "25.0",
        "fee_rate_bps": "25",
        "price": "0.55",
        "status": "MATCHED",
        "match_time": "1234567890",
        "last_update": "1234567891",
        "outcome": "Yes",
        "bucket_index": 5,
        "owner": "550e8400-e29b-41d4-a716-446655440000",
        "maker_address": "0x1234567890123456789012345678901234567890",
        "maker_orders": [
            {
                "order_id": "0x111",
                "owner": "550e8400-e29b-41d4-a716-446655440000",
                "maker_address": "0x1234567890123456789012345678901234567890",
                "matched_amount": "0.2",
                "price": "0.55",
                "fee_rate_bps": "1",
                "asset_id": "123456789",
                "outcome": "Yes",
                "side": "BUY"
            },
            {
                "order_id": "0x222",
                "owner": "550e8400-e29b-41d4-a716-446655440000",
                "maker_address": "0x1234567890123456789012345678901234567890",
                "matched_amount": "0.2",
                "price": "0.55",
                "fee_rate_bps": "1",
                "asset_id": "123456789",
                "outcome": "Yes",
                "side": "BUY"
            }
        ],
        "transaction_hash": "0x0000000000000000000000000000000000000000000000000000000000000abc",
        "trader_side": "TAKER"
    }"#;
    group.throughput(Throughput::Bytes(trade.len() as u64));
    group.bench_function("TradeResponse", |b| {
        b.iter(|| {
            let _: TradeResponse = serde_json::from_str(std::hint::black_box(trade))
                .expect("Deserialization should succeed");
        });
    });

    let cancel = r#"{
        "canceled": ["0x123", "0x456", "0x789"],
        "notCanceled": {
            "0xabc": "Order already filled",
            "0xdef": "Order not found"
        }
    }"#;
    group.throughput(Throughput::Bytes(cancel.len() as u64));
    group.bench_function("CancelOrdersResponse", |b| {
        b.iter(|| {
            let _: CancelOrdersResponse = serde_json::from_str(std::hint::black_box(cancel))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_market_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/market_data");

    let market = r#"{
        "enable_order_book": true,
        "active": true,
        "closed": false,
        "archived": false,
        "accepting_orders": true,
        "accepting_order_timestamp": null,
        "minimum_order_size": "1.0",
        "minimum_tick_size": "0.01",
        "condition_id": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "question_id": "0x0000000000000000000000000000000000000000000000000000000000000002",
        "question": "Will X happen?",
        "description": "Test market for benchmarking",
        "market_slug": "test-market-2024",
        "end_date_iso": "2024-12-31T23:59:59Z",
        "game_start_time": null,
        "seconds_delay": 0,
        "fpmm": "0x1234567890123456789012345678901234567890",
        "maker_base_fee": "0.001",
        "taker_base_fee": "0.002",
        "notifications_enabled": true,
        "neg_risk": false,
        "neg_risk_market_id": "",
        "neg_risk_request_id": "",
        "icon": "https://polymarket.com/icon.png",
        "image": "https://polymarket.com/image.png",
        "rewards": {"rates": [], "min_size": "0", "max_spread": "0"},
        "is_50_50_outcome": true,
        "tokens": [
            {"token_id": "123456789", "outcome": "Yes", "price": "0.55", "winner": false},
            {"token_id": "987654321", "outcome": "No", "price": "0.45", "winner": false}
        ],
        "tags": ["politics", "2024"]
    }"#;
    group.throughput(Throughput::Bytes(market.len() as u64));
    group.bench_function("MarketResponse", |b| {
        b.iter(|| {
            let _: MarketResponse = serde_json::from_str(std::hint::black_box(market))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_pricing(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/pricing");

    let midpoint = r#"{"mid": "0.55"}"#;
    group.bench_function("MidpointResponse", |b| {
        b.iter(|| {
            let _: MidpointResponse = serde_json::from_str(std::hint::black_box(midpoint))
                .expect("Deserialization should succeed");
        });
    });

    let price = r#"{"price": "0.60"}"#;
    group.bench_function("PriceResponse", |b| {
        b.iter(|| {
            let _: PriceResponse = serde_json::from_str(std::hint::black_box(price))
                .expect("Deserialization should succeed");
        });
    });

    let spread = r#"{"spread": "0.05"}"#;
    group.bench_function("SpreadResponse", |b| {
        b.iter(|| {
            let _: SpreadResponse = serde_json::from_str(std::hint::black_box(spread))
                .expect("Deserialization should succeed");
        });
    });

    let tick_size = r#"{"minimum_tick_size": "0.01"}"#;
    group.bench_function("TickSizeResponse", |b| {
        b.iter(|| {
            let _: TickSizeResponse = serde_json::from_str(std::hint::black_box(tick_size))
                .expect("Deserialization should succeed");
        });
    });

    let neg_risk = r#"{"neg_risk": false}"#;
    group.bench_function("NegRiskResponse", |b| {
        b.iter(|| {
            let _: NegRiskResponse = serde_json::from_str(std::hint::black_box(neg_risk))
                .expect("Deserialization should succeed");
        });
    });

    let fee_rate = r#"{"base_fee": 25}"#;
    group.bench_function("FeeRateResponse", |b| {
        b.iter(|| {
            let _: FeeRateResponse = serde_json::from_str(std::hint::black_box(fee_rate))
                .expect("Deserialization should succeed");
        });
    });

    let last_trade_price = r#"{"price": "0.55", "side": "BUY"}"#;
    group.bench_function("LastTradePriceResponse", |b| {
        b.iter(|| {
            let _: LastTradePriceResponse =
                serde_json::from_str(std::hint::black_box(last_trade_price))
                    .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_account_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/account");

    let balance = r#"{"balance": "1000.50", "allowance": "500.25"}"#;
    group.bench_function("BalanceAllowanceResponse", |b| {
        b.iter(|| {
            let _: BalanceAllowanceResponse = serde_json::from_str(std::hint::black_box(balance))
                .expect("Deserialization should succeed");
        });
    });

    let api_keys = r#"{"api_keys": ["key1", "key2", "key3"]}"#;
    group.bench_function("ApiKeysResponse", |b| {
        b.iter(|| {
            let _: ApiKeysResponse = serde_json::from_str(std::hint::black_box(api_keys))
                .expect("Deserialization should succeed");
        });
    });

    let ban_status = r#"{
        "closed_only": true
    }"#;
    group.bench_function("BanStatusResponse", |b| {
        b.iter(|| {
            let _: BanStatusResponse = serde_json::from_str(std::hint::black_box(ban_status))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_additional_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("clob/additional");

    let notification = r#"{
        "type": 1,
        "owner": "550e8400-e29b-41d4-a716-446655440000",
        "payload": {
            "asset_id": "123456789",
            "condition_id": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "eventSlug": "test-event",
            "icon": "https://polymarket.com/icon.png",
            "image": "https://polymarket.com/image.png",
            "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "market_slug": "test-market",
            "matched_size": "25.0",
            "order_id": "0x123",
            "original_size": "100.0",
            "outcome": "Yes",
            "outcome_index": 0,
            "owner": "550e8400-e29b-41d4-a716-446655440000",
            "price": "0.55",
            "question": "Will X happen?",
            "remaining_size": "75.0",
            "seriesSlug": "test-series",
            "side": "BUY",
            "trade_id": "trade_123",
            "transaction_hash": "0x0000000000000000000000000000000000000000000000000000000000000abc",
            "type": "GTC"
        }
    }"#;
    group.bench_function("NotificationResponse", |b| {
        b.iter(|| {
            let _: NotificationResponse = serde_json::from_str(std::hint::black_box(notification))
                .expect("Deserialization should succeed");
        });
    });

    let price_history = r#"{
        "history": [
            {"t": 1234567890000, "p": "0.55"},
            {"t": 1234567891000, "p": "0.56"},
            {"t": 1234567892000, "p": "0.54"},
            {"t": 1234567893000, "p": "0.57"},
            {"t": 1234567894000, "p": "0.55"}
        ]
    }"#;
    group.bench_function("PriceHistoryResponse", |b| {
        b.iter(|| {
            let _: PriceHistoryResponse = serde_json::from_str(std::hint::black_box(price_history))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

criterion_group!(
    clob_benches,
    bench_orderbook,
    bench_orders,
    bench_market_data,
    bench_pricing,
    bench_account_data,
    bench_additional_types
);
criterion_main!(clob_benches);



================================================
FILE: benches/deserialize_websocket.rs
================================================
/// Comprehensive benchmarks for CLOB WebSocket message deserialization.
///
/// This module benchmarks ALL WebSocket message types with special focus on the MOST CRITICAL
/// hot paths for live trading: orderbook updates, trade notifications, and order status updates.
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use polymarket_client_sdk::clob::ws::types::response::OrderBookLevel;
use polymarket_client_sdk::clob::ws::{
    BestBidAsk, BookUpdate, LastTradePrice, MakerOrder, MarketResolved, MidpointUpdate, NewMarket,
    OrderMessage, PriceChange, TickSizeChange, TradeMessage, WsMessage,
};

fn bench_ws_message(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/ws_message");

    let book_msg = r#"{
        "event_type": "book",
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "bids": [{"price": "0.55", "size": "100.0"}],
        "asks": [{"price": "0.56", "size": "150.0"}]
    }"#;
    group.throughput(Throughput::Bytes(book_msg.len() as u64));
    group.bench_function("WsMessage::Book", |b| {
        b.iter(|| {
            let _: WsMessage = serde_json::from_str(std::hint::black_box(book_msg))
                .expect("Deserialization should succeed");
        });
    });

    let price_change_msg = r#"{
        "event_type": "price_change",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "price_changes": [{
            "asset_id": "123456789",
            "price": "0.65",
            "side": "BUY"
        }]
    }"#;
    group.throughput(Throughput::Bytes(price_change_msg.len() as u64));
    group.bench_function("WsMessage::PriceChange", |b| {
        b.iter(|| {
            let _: WsMessage = serde_json::from_str(std::hint::black_box(price_change_msg))
                .expect("Deserialization should succeed");
        });
    });

    let trade_msg = r#"{
        "event_type": "trade",
        "id": "trade_123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "size": "25.0",
        "price": "0.55",
        "status": "MATCHED",
        "maker_orders": []
    }"#;
    group.throughput(Throughput::Bytes(trade_msg.len() as u64));
    group.bench_function("WsMessage::Trade", |b| {
        b.iter(|| {
            let _: WsMessage = serde_json::from_str(std::hint::black_box(trade_msg))
                .expect("Deserialization should succeed");
        });
    });

    let order_msg = r#"{
        "event_type": "order",
        "id": "0x123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "price": "0.55"
    }"#;
    group.throughput(Throughput::Bytes(order_msg.len() as u64));
    group.bench_function("WsMessage::Order", |b| {
        b.iter(|| {
            let _: WsMessage = serde_json::from_str(std::hint::black_box(order_msg))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_book_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/book_update");

    // BookUpdate - MOST CRITICAL HOT PATH
    // This is the highest frequency message in live trading
    // Deserialized on every orderbook tick (can be 10-100+ per second)

    let book_small = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "bids": [{"price": "0.55", "size": "100.0"}],
        "asks": [{"price": "0.56", "size": "150.0"}]
    }"#;

    let book_medium = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "hash": "abc123",
        "bids": [
            {"price": "0.55", "size": "100.0"},
            {"price": "0.54", "size": "200.0"},
            {"price": "0.53", "size": "300.0"},
            {"price": "0.52", "size": "400.0"},
            {"price": "0.51", "size": "500.0"}
        ],
        "asks": [
            {"price": "0.56", "size": "150.0"},
            {"price": "0.57", "size": "175.0"},
            {"price": "0.58", "size": "200.0"},
            {"price": "0.59", "size": "225.0"},
            {"price": "0.60", "size": "250.0"}
        ]
    }"#;

    let book_large = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "hash": "abc123",
        "bids": [
            {"price": "0.55", "size": "100.0"}, {"price": "0.54", "size": "200.0"},
            {"price": "0.53", "size": "300.0"}, {"price": "0.52", "size": "400.0"},
            {"price": "0.51", "size": "500.0"}, {"price": "0.50", "size": "600.0"},
            {"price": "0.49", "size": "700.0"}, {"price": "0.48", "size": "800.0"},
            {"price": "0.47", "size": "900.0"}, {"price": "0.46", "size": "1000.0"},
            {"price": "0.45", "size": "1100.0"}, {"price": "0.44", "size": "1200.0"},
            {"price": "0.43", "size": "1300.0"}, {"price": "0.42", "size": "1400.0"},
            {"price": "0.41", "size": "1500.0"}, {"price": "0.40", "size": "1600.0"},
            {"price": "0.39", "size": "1700.0"}, {"price": "0.38", "size": "1800.0"},
            {"price": "0.37", "size": "1900.0"}, {"price": "0.36", "size": "2000.0"}
        ],
        "asks": [
            {"price": "0.56", "size": "150.0"}, {"price": "0.57", "size": "175.0"},
            {"price": "0.58", "size": "200.0"}, {"price": "0.59", "size": "225.0"},
            {"price": "0.60", "size": "250.0"}, {"price": "0.61", "size": "275.0"},
            {"price": "0.62", "size": "300.0"}, {"price": "0.63", "size": "325.0"},
            {"price": "0.64", "size": "350.0"}, {"price": "0.65", "size": "375.0"},
            {"price": "0.66", "size": "400.0"}, {"price": "0.67", "size": "425.0"},
            {"price": "0.68", "size": "450.0"}, {"price": "0.69", "size": "475.0"},
            {"price": "0.70", "size": "500.0"}, {"price": "0.71", "size": "525.0"},
            {"price": "0.72", "size": "550.0"}, {"price": "0.73", "size": "575.0"},
            {"price": "0.74", "size": "600.0"}, {"price": "0.75", "size": "625.0"}
        ]
    }"#;

    for (name, json) in [
        ("1_level", book_small),
        ("5_levels", book_medium),
        ("20_levels", book_large),
    ] {
        group.throughput(Throughput::Bytes(json.len() as u64));
        group.bench_with_input(BenchmarkId::new("BookUpdate", name), &json, |b, json| {
            b.iter(|| {
                let _: BookUpdate = serde_json::from_str(std::hint::black_box(json))
                    .expect("Deserialization should succeed");
            });
        });
    }

    group.finish();
}

fn bench_user_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/user_messages");

    let trade_minimal = r#"{
        "id": "trade_123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "size": "25.0",
        "price": "0.55",
        "status": "MATCHED",
        "maker_orders": []
    }"#;

    let trade_full = r#"{
        "id": "trade_123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "size": "25.0",
        "price": "0.55",
        "status": "CONFIRMED",
        "type": "TRADE",
        "last_update": "1704110400000",
        "matchtime": "1704110400000",
        "timestamp": "1704110400000",
        "outcome": "Yes",
        "owner": "550e8400-e29b-41d4-a716-446655440000",
        "trade_owner": "550e8400-e29b-41d4-a716-446655440000",
        "taker_order_id": "0xabcdef",
        "maker_orders": [
            {
                "order_id": "0x111",
                "asset_id": "123456789",
                "outcome": "Yes",
                "price": "0.55",
                "matched_amount": "10.0",
                "owner": "550e8400-e29b-41d4-a716-446655440000"
            },
            {
                "order_id": "0x222",
                "asset_id": "123456789",
                "outcome": "Yes",
                "price": "0.55",
                "matched_amount": "15.0",
                "owner": "550e8400-e29b-41d4-a716-446655440000"
            }
        ],
        "fee_rate_bps": "25",
        "transaction_hash": "0x0000000000000000000000000000000000000000000000000000000000000abc",
        "trader_side": "TAKER"
    }"#;

    for (name, json) in [("minimal", trade_minimal), ("full", trade_full)] {
        group.throughput(Throughput::Bytes(json.len() as u64));
        group.bench_with_input(BenchmarkId::new("TradeMessage", name), &json, |b, json| {
            b.iter(|| {
                let _: TradeMessage = serde_json::from_str(std::hint::black_box(json))
                    .expect("Deserialization should succeed");
            });
        });
    }

    let order_minimal = r#"{
        "id": "0x123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "price": "0.55"
    }"#;

    let order_full = r#"{
        "id": "0x123",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "asset_id": "123456789",
        "side": "BUY",
        "price": "0.55",
        "type": "PLACEMENT",
        "outcome": "Yes",
        "owner": "550e8400-e29b-41d4-a716-446655440000",
        "order_owner": "550e8400-e29b-41d4-a716-446655440000",
        "original_size": "100.0",
        "size_matched": "25.0",
        "timestamp": "1704110400000",
        "associate_trades": ["trade_123", "trade_456"]
    }"#;

    for (name, json) in [("minimal", order_minimal), ("full", order_full)] {
        group.throughput(Throughput::Bytes(json.len() as u64));
        group.bench_with_input(BenchmarkId::new("OrderMessage", name), &json, |b, json| {
            b.iter(|| {
                let _: OrderMessage = serde_json::from_str(std::hint::black_box(json))
                    .expect("Deserialization should succeed");
            });
        });
    }

    group.finish();
}

fn bench_market_data_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/market_data");

    let price_change_single = r#"{
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "price_changes": [{
            "asset_id": "123456789",
            "price": "0.65",
            "side": "BUY"
        }]
    }"#;

    let price_change_batch = r#"{
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "price_changes": [
            {"asset_id": "123456789", "price": "0.65", "side": "BUY", "hash": "abc1", "best_bid": "0.64", "best_ask": "0.66"},
            {"asset_id": "987654321", "price": "0.35", "side": "SELL", "hash": "abc2", "best_bid": "0.34", "best_ask": "0.36"},
            {"asset_id": "555555555", "price": "0.50", "side": "BUY", "hash": "abc3", "best_bid": "0.49", "best_ask": "0.51"}
        ]
    }"#;

    for (name, json) in [
        ("single", price_change_single),
        ("batch_3", price_change_batch),
    ] {
        group.throughput(Throughput::Bytes(json.len() as u64));
        group.bench_with_input(BenchmarkId::new("PriceChange", name), &json, |b, json| {
            b.iter(|| {
                let _: PriceChange = serde_json::from_str(std::hint::black_box(json))
                    .expect("Deserialization should succeed");
            });
        });
    }

    let last_trade_price = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "price": "0.55",
        "side": "BUY"
    }"#;
    group.throughput(Throughput::Bytes(last_trade_price.len() as u64));
    group.bench_function("LastTradePrice", |b| {
        b.iter(|| {
            let _: LastTradePrice = serde_json::from_str(std::hint::black_box(last_trade_price))
                .expect("Deserialization should succeed");
        });
    });

    let best_bid_ask = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "best_bid": "0.54",
        "best_ask": "0.56",
        "spread": "0.02"
    }"#;
    group.throughput(Throughput::Bytes(best_bid_ask.len() as u64));
    group.bench_function("BestBidAsk", |b| {
        b.iter(|| {
            let _: BestBidAsk = serde_json::from_str(std::hint::black_box(best_bid_ask))
                .expect("Deserialization should succeed");
        });
    });

    let tick_size_change = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "old_tick_size": "0.01",
        "new_tick_size": "0.001",
        "timestamp": "1"
    }"#;
    group.throughput(Throughput::Bytes(tick_size_change.len() as u64));
    group.bench_function("TickSizeChange", |b| {
        b.iter(|| {
            let _: TickSizeChange = serde_json::from_str(std::hint::black_box(tick_size_change))
                .expect("Deserialization should succeed");
        });
    });

    let midpoint_update = r#"{
        "asset_id": "123456789",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "timestamp": "1234567890123",
        "midpoint": "0.55"
    }"#;
    group.throughput(Throughput::Bytes(midpoint_update.len() as u64));
    group.bench_function("MidpointUpdate", |b| {
        b.iter(|| {
            let _: MidpointUpdate = serde_json::from_str(std::hint::black_box(midpoint_update))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_market_events(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/market_events");

    let new_market = r#"{
        "id": "1",
        "question": "Will X happen?",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "slug": "test-market-2024",
        "description": "Test market for benchmarking",
        "assets_ids": ["123456789", "987654321"],
        "outcomes": ["Yes", "No"],
        "timestamp": "1704110400000"
    }"#;
    group.throughput(Throughput::Bytes(new_market.len() as u64));
    group.bench_function("NewMarket", |b| {
        b.iter(|| {
            let _: NewMarket = serde_json::from_str(std::hint::black_box(new_market))
                .expect("Deserialization should succeed");
        });
    });

    let market_resolved = r#"{
        "id": "1",
        "question": "Will X happen?",
        "market": "0x0000000000000000000000000000000000000000000000000000000000000001",
        "slug": "test-market-2024",
        "description": "Test market for benchmarking",
        "assets_ids": ["123456789", "987654321"],
        "outcomes": ["Yes", "No"],
        "winning_asset_id": "123456789",
        "winning_outcome": "Yes",
        "timestamp": "1704110400000"
    }"#;
    group.throughput(Throughput::Bytes(market_resolved.len() as u64));
    group.bench_function("MarketResolved", |b| {
        b.iter(|| {
            let _: MarketResolved = serde_json::from_str(std::hint::black_box(market_resolved))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

fn bench_orderbook_level(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket/primitives");

    let level = r#"{"price": "0.55", "size": "100.0"}"#;
    group.throughput(Throughput::Bytes(level.len() as u64));
    group.bench_function("OrderBookLevel", |b| {
        b.iter(|| {
            let _: OrderBookLevel = serde_json::from_str(std::hint::black_box(level))
                .expect("Deserialization should succeed");
        });
    });

    let maker_order = r#"{
        "order_id": "0x123",
        "asset_id": "123456789",
        "outcome": "Yes",
        "price": "0.55",
        "matched_amount": "10.0",
        "owner": "550e8400-e29b-41d4-a716-446655440000"
    }"#;
    group.throughput(Throughput::Bytes(maker_order.len() as u64));
    group.bench_function("MakerOrder", |b| {
        b.iter(|| {
            let _: MakerOrder = serde_json::from_str(std::hint::black_box(maker_order))
                .expect("Deserialization should succeed");
        });
    });

    group.finish();
}

criterion_group!(
    websocket_benches,
    bench_ws_message,
    bench_book_update,
    bench_user_messages,
    bench_market_data_updates,
    bench_market_events,
    bench_orderbook_level
);
criterion_main!(websocket_benches);



================================================
FILE: examples/approvals.rs
================================================
#![allow(clippy::exhaustive_enums, reason = "Generated by sol! macro")]
#![allow(clippy::exhaustive_structs, reason = "Generated by sol! macro")]
#![allow(clippy::unwrap_used, reason = "Examples use unwrap for brevity")]

//! Token approval example for Polymarket CLOB trading.
//!
//! This example demonstrates how to set the required token allowances for trading on Polymarket.
//! You must approve three contracts:
//!
//! 1. **CTF Exchange** (`config.exchange`) - Standard market trading
//! 2. **Neg Risk CTF Exchange** (`neg_risk_config.exchange`) - Neg-risk market trading
//! 3. **Neg Risk Adapter** (`neg_risk_config.neg_risk_adapter`) - Token minting/splitting for neg-risk
//!
//! Each contract needs two approvals:
//! - ERC-20 approval for USDC (collateral token)
//! - ERC-1155 approval for Conditional Tokens (outcome tokens)
//!
//! You only need to run these approvals once per wallet.
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example approvals --features tracing
//! ```
//!
//! Dry run (no transactions executed):
//! ```sh
//! RUST_LOG=info cargo run --example approvals --features tracing -- --dry-run
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=approvals.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example approvals --features tracing
//! ```

use std::env;
use std::fs::File;
use std::str::FromStr as _;

use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use alloy::sol;
use polymarket_client_sdk::types::{Address, address};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR, contract_config};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

const RPC_URL: &str = "https://polygon-rpc.com";

const USDC_ADDRESS: Address = address!("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174");
const TOKEN_TO_APPROVE: Address = USDC_ADDRESS;

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
    }

    #[sol(rpc)]
    interface IERC1155 {
        function setApprovalForAll(address operator, bool approved) external;
        function isApprovedForAll(address account, address operator) external view returns (bool);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|arg| arg == "--dry-run");

    let chain = POLYGON;
    let config = contract_config(chain, false).unwrap();
    let neg_risk_config = contract_config(chain, true).unwrap();

    // Collect all contracts that need approval
    let mut targets: Vec<(&str, Address)> = vec![
        ("CTF Exchange", config.exchange),
        ("Neg Risk CTF Exchange", neg_risk_config.exchange),
    ];

    // Add the Neg Risk Adapter if available
    if let Some(adapter) = neg_risk_config.neg_risk_adapter {
        targets.push(("Neg Risk Adapter", adapter));
    }

    if dry_run {
        info!(mode = "dry_run", "showing approvals without executing");
        for (name, target) in &targets {
            info!(contract = name, address = %target, "would receive approval");
        }
        info!(total = targets.len(), "contracts would be approved");
        return Ok(());
    }

    let private_key = env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(chain));

    let provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .connect(RPC_URL)
        .await?;

    let owner = signer.address();
    info!(address = %owner, "wallet loaded");

    let token = IERC20::new(TOKEN_TO_APPROVE, provider.clone());
    let ctf = IERC1155::new(config.conditional_tokens, provider.clone());

    info!(phase = "checking", "querying current allowances");

    for (name, target) in &targets {
        match check_allowance(&token, owner, *target).await {
            Ok(allowance) => info!(contract = name, usdc_allowance = %allowance),
            Err(e) => error!(contract = name, error = ?e, "failed to check USDC allowance"),
        }

        match check_approval_for_all(&ctf, owner, *target).await {
            Ok(approved) => info!(contract = name, ctf_approved = approved),
            Err(e) => error!(contract = name, error = ?e, "failed to check CTF approval"),
        }
    }

    info!(phase = "approving", "setting approvals");

    for (name, target) in &targets {
        info!(contract = name, address = %target, "approving");

        match approve(&token, *target, U256::MAX).await {
            Ok(tx_hash) => info!(contract = name, tx = %tx_hash, "USDC approved"),
            Err(e) => error!(contract = name, error = ?e, "USDC approve failed"),
        }

        match set_approval_for_all(&ctf, *target, true).await {
            Ok(tx_hash) => info!(contract = name, tx = %tx_hash, "CTF approved"),
            Err(e) => error!(contract = name, error = ?e, "CTF setApprovalForAll failed"),
        }
    }

    info!(phase = "verifying", "confirming approvals");

    for (name, target) in &targets {
        match check_allowance(&token, owner, *target).await {
            Ok(allowance) => info!(contract = name, usdc_allowance = %allowance, "verified"),
            Err(e) => error!(contract = name, error = ?e, "verification failed"),
        }

        match check_approval_for_all(&ctf, owner, *target).await {
            Ok(approved) => info!(contract = name, ctf_approved = approved, "verified"),
            Err(e) => error!(contract = name, error = ?e, "verification failed"),
        }
    }

    info!("all approvals complete");

    Ok(())
}

async fn check_allowance<P: alloy::providers::Provider>(
    token: &IERC20::IERC20Instance<P>,
    owner: Address,
    spender: Address,
) -> anyhow::Result<U256> {
    let allowance = token.allowance(owner, spender).call().await?;
    Ok(allowance)
}

async fn check_approval_for_all<P: alloy::providers::Provider>(
    ctf: &IERC1155::IERC1155Instance<P>,
    account: Address,
    operator: Address,
) -> anyhow::Result<bool> {
    let approved = ctf.isApprovedForAll(account, operator).call().await?;
    Ok(approved)
}

async fn approve<P: alloy::providers::Provider>(
    usdc: &IERC20::IERC20Instance<P>,
    spender: Address,
    amount: U256,
) -> anyhow::Result<alloy::primitives::FixedBytes<32>> {
    let tx_hash = usdc.approve(spender, amount).send().await?.watch().await?;
    Ok(tx_hash)
}

async fn set_approval_for_all<P: alloy::providers::Provider>(
    ctf: &IERC1155::IERC1155Instance<P>,
    operator: Address,
    approved: bool,
) -> anyhow::Result<alloy::primitives::FixedBytes<32>> {
    let tx_hash = ctf
        .setApprovalForAll(operator, approved)
        .send()
        .await?
        .watch()
        .await?;
    Ok(tx_hash)
}



================================================
FILE: examples/bridge.rs
================================================
//! Bridge API example demonstrating deposit and supported assets endpoints.
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example bridge --features bridge,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=bridge.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example bridge --features bridge,tracing
//! ```

use std::fs::File;

use polymarket_client_sdk::bridge::Client;
use polymarket_client_sdk::bridge::types::{DepositRequest, StatusRequest};
use polymarket_client_sdk::types::address;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();

    match client.supported_assets().await {
        Ok(response) => {
            info!(
                endpoint = "supported_assets",
                count = response.supported_assets.len()
            );
            for asset in &response.supported_assets {
                info!(
                    endpoint = "supported_assets",
                    name = %asset.token.name,
                    symbol = %asset.token.symbol,
                    chain = %asset.chain_name,
                    chain_id = asset.chain_id,
                    min_usd = %asset.min_checkout_usd
                );
            }
        }
        Err(e) => debug!(endpoint = "supported_assets", error = %e),
    }

    let request = DepositRequest::builder()
        .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
        .build();

    match client.deposit(&request).await {
        Ok(response) => {
            info!(
                endpoint = "deposit",
                evm = %response.address.evm,
                svm = %response.address.svm,
                btc = %response.address.btc,
                note = ?response.note
            );
        }
        Err(e) => debug!(endpoint = "deposit", error = %e),
    }

    let status_request = StatusRequest::builder()
        .address("bc1qs82vw5pczv9uj44n4npscldkdjgfjqu7x9mlna")
        .build();

    match client.status(&status_request).await {
        Ok(response) => {
            info!(endpoint = "status", count = response.transactions.len());
        }
        Err(e) => debug!(endpoint = "status", error = %e),
    }

    Ok(())
}



================================================
FILE: examples/check_approvals.rs
================================================
#![allow(clippy::exhaustive_enums, reason = "Generated by sol! macro")]
#![allow(clippy::exhaustive_structs, reason = "Generated by sol! macro")]
#![allow(clippy::print_stderr, reason = "Usage message to stderr")]
#![allow(clippy::unwrap_used, reason = "Examples use unwrap for brevity")]

//! Read-only example to check current token approvals for Polymarket CLOB trading.
//!
//! This example queries the blockchain to show which contracts are approved
//! for a given wallet address. No private key or gas required.
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example check_approvals --features tracing -- <WALLET_ADDRESS>
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=check_approvals.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example check_approvals --features tracing -- <WALLET_ADDRESS>
//! ```
//!
//! Example:
//! ```sh
//! RUST_LOG=info cargo run --example check_approvals --features tracing -- 0x1234...abcd
//! ```

use std::env;
use std::fs::File;

use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use alloy::sol;
use polymarket_client_sdk::types::{Address, address};
use polymarket_client_sdk::{POLYGON, contract_config};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

const RPC_URL: &str = "https://polygon-rpc.com";

const USDC_ADDRESS: Address = address!("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174");

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function allowance(address owner, address spender) external view returns (uint256);
    }

    #[sol(rpc)]
    interface IERC1155 {
        function isApprovedForAll(address account, address operator) external view returns (bool);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        debug!(
            args = args.len(),
            "invalid arguments - expected wallet address"
        );
        eprintln!("Usage: cargo run --example check_approvals -- <WALLET_ADDRESS>");
        eprintln!(
            "Example: cargo run --example check_approvals -- 0x1234567890abcdef1234567890abcdef12345678"
        );
        std::process::exit(1);
    }

    let wallet_address: Address = args[1].parse()?;

    info!(wallet = %wallet_address, chain = "Polygon Mainnet (137)", "checking approvals");

    let provider = ProviderBuilder::new().connect(RPC_URL).await?;

    let config = contract_config(POLYGON, false).unwrap();
    let neg_risk_config = contract_config(POLYGON, true).unwrap();

    let usdc = IERC20::new(USDC_ADDRESS, provider.clone());
    let ctf = IERC1155::new(config.conditional_tokens, provider.clone());

    // All contracts that need approval for full CLOB trading
    let mut targets: Vec<(&str, Address)> = vec![
        ("CTF Exchange", config.exchange),
        ("Neg Risk CTF Exchange", neg_risk_config.exchange),
    ];

    if let Some(adapter) = neg_risk_config.neg_risk_adapter {
        targets.push(("Neg Risk Adapter", adapter));
    }

    let mut all_approved = true;

    for (name, target) in &targets {
        let usdc_result = usdc.allowance(wallet_address, *target).call().await;
        let ctf_result = ctf.isApprovedForAll(wallet_address, *target).call().await;

        match (&usdc_result, &ctf_result) {
            (Ok(usdc_allowance), Ok(ctf_approved)) => {
                let usdc_ok = *usdc_allowance > U256::ZERO;
                let ctf_ok = *ctf_approved;

                if !usdc_ok || !ctf_ok {
                    all_approved = false;
                }

                info!(
                    contract = name,
                    address = %target,
                    usdc_allowance = %format_allowance(*usdc_allowance),
                    usdc_approved = usdc_ok,
                    ctf_approved = ctf_ok,
                );
            }
            (Err(e), _) => {
                debug!(contract = name, error = %e, "failed to check USDC allowance");
                all_approved = false;
            }
            (_, Err(e)) => {
                debug!(contract = name, error = %e, "failed to check CTF approval");
                all_approved = false;
            }
        }
    }

    if all_approved {
        info!(status = "ready", "all contracts properly approved");
    } else {
        info!(
            status = "incomplete",
            "some approvals missing - run: cargo run --example approvals"
        );
    }

    Ok(())
}

fn format_allowance(allowance: U256) -> String {
    if allowance == U256::MAX {
        "MAX (unlimited)".to_owned()
    } else if allowance == U256::ZERO {
        "0".to_owned()
    } else {
        // USDC has 6 decimals
        let usdc_decimals = U256::from(1_000_000);
        let whole = allowance / usdc_decimals;
        format!("{whole} USDC")
    }
}



================================================
FILE: examples/ctf.rs
================================================
#![allow(clippy::exhaustive_enums, reason = "Fine for examples")]
#![allow(clippy::exhaustive_structs, reason = "Fine for examples")]

//! CTF (Conditional Token Framework) example.
//!
//! This example demonstrates how to interact with the CTF contract to:
//! - Calculate condition IDs, collection IDs, and position IDs
//! - Split USDC collateral into outcome tokens (YES/NO)
//! - Merge outcome tokens back into USDC
//! - Redeem winning tokens after market resolution
//!
//! ## Usage
//!
//! For read-only operations (ID calculations):
//! ```sh
//! cargo run --example ctf --features ctf
//! ```
//!
//! For write operations (split, merge, redeem), you need a private key:
//! ```sh
//! export POLYMARKET_PRIVATE_KEY="your_private_key"
//! cargo run --example ctf --features ctf -- --write
//! ```

use std::env;
use std::str::FromStr as _;

use alloy::primitives::{B256, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use anyhow::Result;
use polymarket_client_sdk::ctf::Client;
use polymarket_client_sdk::ctf::types::{
    CollectionIdRequest, ConditionIdRequest, MergePositionsRequest, PositionIdRequest,
    RedeemPositionsRequest, SplitPositionRequest,
};
use polymarket_client_sdk::types::address;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tracing::{error, info};

const RPC_URL: &str = "https://polygon-rpc.com";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let write_mode = args.iter().any(|arg| arg == "--write");

    let chain = POLYGON;
    info!("=== CTF (Conditional Token Framework) Example ===");

    // For read-only operations, we don't need a wallet
    let provider = ProviderBuilder::new().connect(RPC_URL).await?;
    let client = Client::new(provider, chain)?;

    info!("Connected to Polygon {chain}");
    info!("CTF contract: 0x4D97DCd97eC945f40cF65F87097ACe5EA0476045");

    // Example: Calculate a condition ID
    info!("--- Calculating Condition ID ---");
    let oracle = address!("0x0000000000000000000000000000000000000001");
    let question_id = B256::ZERO;
    let outcome_slot_count = U256::from(2);

    let condition_req = ConditionIdRequest::builder()
        .oracle(oracle)
        .question_id(question_id)
        .outcome_slot_count(outcome_slot_count)
        .build();

    let condition_resp = client.condition_id(&condition_req).await?;
    info!("Oracle: {oracle}");
    info!("Question ID: {question_id}");
    info!("Outcome Slots: {outcome_slot_count}");
    info!("→ Condition ID: {}", condition_resp.condition_id);

    // Example: Calculate collection IDs for YES and NO tokens
    info!("--- Calculating Collection IDs ---");
    let parent_collection_id = B256::ZERO;

    // Collection ID for YES token (index set = 0b01 = 1)
    let yes_collection_req = CollectionIdRequest::builder()
        .parent_collection_id(parent_collection_id)
        .condition_id(condition_resp.condition_id)
        .index_set(U256::from(1))
        .build();

    let yes_collection_resp = client.collection_id(&yes_collection_req).await?;
    info!("YES token (index set = 1):");
    info!("→ Collection ID: {}", yes_collection_resp.collection_id);

    // Collection ID for NO token (index set = 0b10 = 2)
    let no_collection_req = CollectionIdRequest::builder()
        .parent_collection_id(parent_collection_id)
        .condition_id(condition_resp.condition_id)
        .index_set(U256::from(2))
        .build();

    let no_collection_resp = client.collection_id(&no_collection_req).await?;
    info!("NO token (index set = 2):");
    info!("→ Collection ID: {}", no_collection_resp.collection_id);

    // Example: Calculate position IDs (ERC1155 token IDs)
    info!("--- Calculating Position IDs ---");
    let usdc = address!("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174");

    let yes_position_req = PositionIdRequest::builder()
        .collateral_token(usdc)
        .collection_id(yes_collection_resp.collection_id)
        .build();

    let yes_position_resp = client.position_id(&yes_position_req).await?;
    info!(
        "YES position (ERC1155 token ID): {}",
        yes_position_resp.position_id
    );

    let no_position_req = PositionIdRequest::builder()
        .collateral_token(usdc)
        .collection_id(no_collection_resp.collection_id)
        .build();

    let no_position_resp = client.position_id(&no_position_req).await?;
    info!(
        "NO position (ERC1155 token ID): {}",
        no_position_resp.position_id
    );

    // Write operations require a wallet
    if write_mode {
        info!("--- Write Operations (requires wallet) ---");

        let private_key =
            env::var(PRIVATE_KEY_VAR).expect("Need a private key for write operations");
        let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(chain));

        let provider = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect(RPC_URL)
            .await?;

        let client = Client::new(provider, chain)?;
        let wallet_address = signer.address();

        info!("Using wallet: {wallet_address:?}");

        // Example: Split 1 USDC into YES and NO tokens (using convenience method)
        info!("--- Splitting Position (Binary Market) ---");
        info!("This will split 1 USDC into 1 YES and 1 NO token");
        info!("Note: You must approve the CTF contract to spend your USDC first!");

        // Using the convenience method for binary markets
        let split_req = SplitPositionRequest::for_binary_market(
            usdc,
            condition_resp.condition_id,
            U256::from(1_000_000), // 1 USDC (6 decimals)
        );

        match client.split_position(&split_req).await {
            Ok(split_resp) => {
                info!("✓ Split transaction successful!");
                info!("  Transaction hash: {}", split_resp.transaction_hash);
                info!("  Block number: {}", split_resp.block_number);
            }
            Err(e) => {
                error!("✗ Split failed: {e}");
                error!("  Make sure you have approved the CTF contract and have sufficient USDC");
            }
        }

        // Example: Merge YES and NO tokens back into USDC (using convenience method)
        info!("--- Merging Positions (Binary Market) ---");
        info!("This will merge 1 YES and 1 NO token back into 1 USDC");

        // Using the convenience method for binary markets
        let merge_req = MergePositionsRequest::for_binary_market(
            usdc,
            condition_resp.condition_id,
            U256::from(1_000_000), // 1 full set
        );

        match client.merge_positions(&merge_req).await {
            Ok(merge_resp) => {
                info!("✓ Merge transaction successful!");
                info!("  Transaction hash: {}", merge_resp.transaction_hash);
                info!("  Block number: {}", merge_resp.block_number);
            }
            Err(e) => {
                error!("✗ Merge failed: {e}");
                error!("  Make sure you have sufficient YES and NO tokens");
            }
        }

        // Example: Redeem winning tokens
        info!("--- Redeeming Positions ---");
        info!("This redeems winning tokens after market resolution");

        // Using the convenience method for binary markets (redeems both YES and NO tokens)
        let redeem_req =
            RedeemPositionsRequest::for_binary_market(usdc, condition_resp.condition_id);

        match client.redeem_positions(&redeem_req).await {
            Ok(redeem_resp) => {
                info!("✓ Redeem transaction successful!");
                info!("  Transaction hash: {}", redeem_resp.transaction_hash);
                info!("  Block number: {}", redeem_resp.block_number);
            }
            Err(e) => {
                error!("✗ Redeem failed: {e}");
                error!("  Make sure the condition is resolved and you have winning tokens");
            }
        }
    } else {
        info!("--- Write Operations ---");
        info!("To test write operations (split, merge, redeem), run with --write flag:");
        info!("  export POLYMARKET_PRIVATE_KEY=\"your_private_key\"");
        info!("  cargo run --example ctf --features ctf -- --write");
    }

    info!("=== Example Complete ===");

    Ok(())
}



================================================
FILE: examples/data.rs
================================================
//! Comprehensive Data API endpoint explorer.
//!
//! This example dynamically tests all Data API endpoints by:
//! 1. Fetching leaderboard data to discover real trader addresses
//! 2. Using those addresses for user-specific queries
//! 3. Extracting market IDs from positions for holder lookups
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example data --features data,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=data.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example data --features data,tracing
//! ```

use std::fs::File;

use polymarket_client_sdk::data::Client;
use polymarket_client_sdk::data::types::request::{
    ActivityRequest, BuilderLeaderboardRequest, BuilderVolumeRequest, ClosedPositionsRequest,
    HoldersRequest, LiveVolumeRequest, OpenInterestRequest, PositionsRequest, TradedRequest,
    TraderLeaderboardRequest, TradesRequest, ValueRequest,
};
use polymarket_client_sdk::data::types::{LeaderboardCategory, TimePeriod};
use polymarket_client_sdk::types::{Address, B256, address, b256};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();

    // Fallback test data when dynamic discovery fails
    let fallback_user = address!("56687bf447db6ffa42ffe2204a05edaa20f55839");
    let fallback_market = b256!("dd22472e552920b8438158ea7238bfadfa4f736aa4cee91a6b86c39ead110917");

    // Health check
    match client.health().await {
        Ok(status) => info!(endpoint = "health", status = %status.data),
        Err(e) => error!(endpoint = "health", error = %e),
    }

    // Fetch leaderboard to get real trader addresses
    let leaderboard_result = client
        .leaderboard(
            &TraderLeaderboardRequest::builder()
                .category(LeaderboardCategory::Overall)
                .time_period(TimePeriod::Week)
                .limit(10)?
                .build(),
        )
        .await;

    let user: Option<Address> = match &leaderboard_result {
        Ok(traders) => {
            info!(endpoint = "leaderboard", count = traders.len());
            if let Some(trader) = traders.first() {
                info!(
                    endpoint = "leaderboard",
                    rank = %trader.rank,
                    address = %trader.proxy_wallet,
                    pnl = %trader.pnl,
                    volume = %trader.vol
                );
                Some(trader.proxy_wallet)
            } else {
                None
            }
        }
        Err(e) => {
            warn!(endpoint = "leaderboard", error = %e, "using fallback user");
            Some(fallback_user)
        }
    };

    // Fetch positions for the discovered user
    let market_id: Option<B256> = if let Some(user) = user {
        let positions_result = client
            .positions(&PositionsRequest::builder().user(user).limit(10)?.build())
            .await;

        match &positions_result {
            Ok(positions) => {
                info!(endpoint = "positions", user = %user, count = positions.len());
                if let Some(pos) = positions.first() {
                    info!(
                        endpoint = "positions",
                        market = %pos.condition_id,
                        size = %pos.size,
                        value = %pos.current_value
                    );
                    Some(pos.condition_id)
                } else {
                    // No positions found, use fallback market
                    warn!(
                        endpoint = "positions",
                        "no positions, using fallback market"
                    );
                    Some(fallback_market)
                }
            }
            Err(e) => {
                warn!(endpoint = "positions", user = %user, error = %e, "using fallback market");
                Some(fallback_market)
            }
        }
    } else {
        debug!(endpoint = "positions", "skipped - no user address found");
        Some(fallback_market)
    };

    // Fetch holders for the discovered market
    if let Some(market) = market_id {
        match client
            .holders(
                &HoldersRequest::builder()
                    .markets(vec![market])
                    .limit(5)?
                    .build(),
            )
            .await
        {
            Ok(meta_holders) => {
                info!(endpoint = "holders", market = %market, tokens = meta_holders.len());
                if let Some(meta) = meta_holders.first() {
                    info!(
                        endpoint = "holders",
                        token = %meta.token,
                        holders_count = meta.holders.len()
                    );
                    if let Some(holder) = meta.holders.first() {
                        info!(
                            endpoint = "holders",
                            address = %holder.proxy_wallet,
                            amount = %holder.amount
                        );
                    }
                }
            }
            Err(e) => error!(endpoint = "holders", market = %market, error = %e),
        }
    }

    // User activity, value, closed positions, and traded count
    if let Some(user) = user {
        match client
            .activity(&ActivityRequest::builder().user(user).limit(5)?.build())
            .await
        {
            Ok(activities) => {
                info!(endpoint = "activity", user = %user, count = activities.len());
                if let Some(act) = activities.first() {
                    info!(
                        endpoint = "activity",
                        activity_type = ?act.activity_type,
                        transaction = %act.transaction_hash
                    );
                }
            }
            Err(e) => error!(endpoint = "activity", user = %user, error = %e),
        }

        match client
            .value(&ValueRequest::builder().user(user).build())
            .await
        {
            Ok(values) => {
                info!(endpoint = "value", user = %user, count = values.len());
                if let Some(value) = values.first() {
                    info!(
                        endpoint = "value",
                        user = %value.user,
                        total = %value.value
                    );
                }
            }
            Err(e) => error!(endpoint = "value", user = %user, error = %e),
        }

        match client
            .closed_positions(
                &ClosedPositionsRequest::builder()
                    .user(user)
                    .limit(5)?
                    .build(),
            )
            .await
        {
            Ok(positions) => {
                info!(endpoint = "closed_positions", user = %user, count = positions.len());
                if let Some(pos) = positions.first() {
                    info!(
                        endpoint = "closed_positions",
                        market = %pos.condition_id,
                        realized_pnl = %pos.realized_pnl
                    );
                }
            }
            Err(e) => error!(endpoint = "closed_positions", user = %user, error = %e),
        }

        match client
            .traded(&TradedRequest::builder().user(user).build())
            .await
        {
            Ok(traded) => {
                info!(
                    endpoint = "traded",
                    user = %user,
                    markets_traded = traded.traded
                );
            }
            Err(e) => error!(endpoint = "traded", user = %user, error = %e),
        }
    }

    // Trades - global trade feed
    match client.trades(&TradesRequest::default()).await {
        Ok(trades) => {
            info!(endpoint = "trades", count = trades.len());
            if let Some(trade) = trades.first() {
                info!(
                    endpoint = "trades",
                    market = %trade.condition_id,
                    side = ?trade.side,
                    size = %trade.size,
                    price = %trade.price
                );
            }
        }
        Err(e) => error!(endpoint = "trades", error = %e),
    }

    // Open interest
    match client.open_interest(&OpenInterestRequest::default()).await {
        Ok(oi_list) => {
            info!(endpoint = "open_interest", count = oi_list.len());
            if let Some(oi) = oi_list.first() {
                info!(
                    endpoint = "open_interest",
                    market = ?oi.market,
                    value = %oi.value
                );
            }
        }
        Err(e) => error!(endpoint = "open_interest", error = %e),
    }

    // Live volume (using event ID 1 as example)
    match client
        .live_volume(&LiveVolumeRequest::builder().id(1).build())
        .await
    {
        Ok(volumes) => {
            info!(
                endpoint = "live_volume",
                event_id = 1,
                count = volumes.len()
            );
            if let Some(vol) = volumes.first() {
                info!(
                    endpoint = "live_volume",
                    total = %vol.total,
                    markets = vol.markets.len()
                );
            }
        }
        Err(e) => error!(endpoint = "live_volume", event_id = 1, error = %e),
    }

    // Builder leaderboard
    match client
        .builder_leaderboard(
            &BuilderLeaderboardRequest::builder()
                .time_period(TimePeriod::Week)
                .limit(5)?
                .build(),
        )
        .await
    {
        Ok(builders) => {
            info!(endpoint = "builder_leaderboard", count = builders.len());
            if let Some(builder) = builders.first() {
                info!(
                    endpoint = "builder_leaderboard",
                    name = %builder.builder,
                    volume = %builder.volume,
                    rank = %builder.rank
                );
            }
        }
        Err(e) => error!(endpoint = "builder_leaderboard", error = %e),
    }

    // Builder volume time series
    match client
        .builder_volume(
            &BuilderVolumeRequest::builder()
                .time_period(TimePeriod::Week)
                .build(),
        )
        .await
    {
        Ok(volumes) => {
            info!(endpoint = "builder_volume", count = volumes.len());
            if let Some(vol) = volumes.first() {
                info!(
                    endpoint = "builder_volume",
                    builder = %vol.builder,
                    date = %vol.dt,
                    volume = %vol.volume
                );
            }
        }
        Err(e) => error!(endpoint = "builder_volume", error = %e),
    }

    Ok(())
}



================================================
FILE: examples/rtds_crypto_prices.rs
================================================
//! Comprehensive RTDS (Real-Time Data Socket) endpoint explorer.
//!
//! This example dynamically tests all RTDS streaming endpoints by:
//! 1. Subscribing to Binance crypto prices (all symbols and filtered)
//! 2. Subscribing to Chainlink price feeds
//! 3. Subscribing to comment events
//! 4. Demonstrating unsubscribe functionality
//! 5. Showing connection state and subscription count
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info cargo run --example rtds_crypto_prices --features rtds,tracing
//! ```

use std::time::Duration;

use futures::StreamExt as _;
use polymarket_client_sdk::rtds::Client;
use polymarket_client_sdk::rtds::types::response::CommentType;
use tokio::time::timeout;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::default();

    // Show connection state
    let state = client.connection_state();
    info!(endpoint = "connection_state", state = ?state);

    // Subscribe to all crypto prices from Binance
    info!(
        stream = "crypto_prices",
        "Subscribing to Binance prices (all symbols)"
    );
    match client.subscribe_crypto_prices(None) {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut count = 0;

            while let Ok(Some(result)) = timeout(Duration::from_secs(5), stream.next()).await {
                match result {
                    Ok(price) => {
                        info!(
                            stream = "crypto_prices",
                            symbol = %price.symbol.to_uppercase(),
                            value = %price.value,
                            timestamp = %price.timestamp
                        );
                        count += 1;
                        if count >= 5 {
                            break;
                        }
                    }
                    Err(e) => debug!(stream = "crypto_prices", error = %e),
                }
            }
            info!(stream = "crypto_prices", received = count);
        }
        Err(e) => debug!(stream = "crypto_prices", error = %e),
    }

    // Subscribe to specific crypto symbols
    let symbols = vec!["btcusdt".to_owned(), "ethusdt".to_owned()];
    info!(
        stream = "crypto_prices_filtered",
        symbols = ?symbols,
        "Subscribing to specific symbols"
    );
    match client.subscribe_crypto_prices(Some(symbols.clone())) {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut count = 0;

            while let Ok(Some(result)) = timeout(Duration::from_secs(5), stream.next()).await {
                match result {
                    Ok(price) => {
                        info!(
                            stream = "crypto_prices_filtered",
                            symbol = %price.symbol.to_uppercase(),
                            value = %price.value
                        );
                        count += 1;
                        if count >= 3 {
                            break;
                        }
                    }
                    Err(e) => debug!(stream = "crypto_prices_filtered", error = %e),
                }
            }
            info!(stream = "crypto_prices_filtered", received = count);
        }
        Err(e) => debug!(stream = "crypto_prices_filtered", error = %e),
    }

    // Subscribe to specific Chainlink symbol
    let chainlink_symbol = "btc/usd".to_owned();
    info!(
        stream = "chainlink_prices",
        symbol = %chainlink_symbol,
        "Subscribing to Chainlink price feed"
    );
    match client.subscribe_chainlink_prices(Some(chainlink_symbol)) {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut count = 0;

            while let Ok(Some(result)) = timeout(Duration::from_secs(5), stream.next()).await {
                match result {
                    Ok(price) => {
                        info!(
                            stream = "chainlink_prices",
                            symbol = %price.symbol,
                            value = %price.value,
                            timestamp = %price.timestamp
                        );
                        count += 1;
                        if count >= 3 {
                            break;
                        }
                    }
                    Err(e) => debug!(stream = "chainlink_prices", error = %e),
                }
            }
            info!(stream = "chainlink_prices", received = count);
        }
        Err(e) => debug!(stream = "chainlink_prices", error = %e),
    }

    // Subscribe to comments (unauthenticated)
    info!(stream = "comments", "Subscribing to comment events");
    match client.subscribe_comments(None) {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut count = 0;

            // Comments may be infrequent, use shorter timeout
            while let Ok(Some(result)) = timeout(Duration::from_secs(3), stream.next()).await {
                match result {
                    Ok(comment) => {
                        info!(
                            stream = "comments",
                            id = %comment.id,
                            parent_type = ?comment.parent_entity_type,
                            parent_id = %comment.parent_entity_id
                        );
                        count += 1;
                        if count >= 3 {
                            break;
                        }
                    }
                    Err(e) => debug!(stream = "comments", error = %e),
                }
            }
            if count > 0 {
                info!(stream = "comments", received = count);
            } else {
                debug!(stream = "comments", "no comments received within timeout");
            }
        }
        Err(e) => debug!(stream = "comments", error = %e),
    }

    // Subscribe to specific comment type
    info!(
        stream = "comments_created",
        comment_type = ?CommentType::CommentCreated,
        "Subscribing to created comments only"
    );
    match client.subscribe_comments(Some(CommentType::CommentCreated)) {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut count = 0;

            while let Ok(Some(result)) = timeout(Duration::from_secs(3), stream.next()).await {
                match result {
                    Ok(comment) => {
                        info!(
                            stream = "comments_created",
                            id = %comment.id,
                            parent_id = %comment.parent_entity_id
                        );
                        count += 1;
                        if count >= 2 {
                            break;
                        }
                    }
                    Err(e) => debug!(stream = "comments_created", error = %e),
                }
            }
            if count > 0 {
                info!(stream = "comments_created", received = count);
            } else {
                debug!(
                    stream = "comments_created",
                    "no created comments received within timeout"
                );
            }
        }
        Err(e) => debug!(stream = "comments_created", error = %e),
    }

    // Show subscription count before unsubscribe
    let sub_count = client.subscription_count();
    info!(
        endpoint = "subscription_count",
        count = sub_count,
        "Before unsubscribe"
    );

    // Demonstrate unsubscribe functionality
    info!("=== Demonstrating unsubscribe ===");

    // Unsubscribe from crypto_prices (Binance)
    info!("Unsubscribing from Binance crypto prices");
    match client.unsubscribe_crypto_prices() {
        Ok(()) => info!("Successfully unsubscribed from crypto_prices"),
        Err(e) => warn!(error = %e, "Failed to unsubscribe from crypto_prices"),
    }

    // Unsubscribe from chainlink prices
    info!("Unsubscribing from Chainlink prices");
    match client.unsubscribe_chainlink_prices() {
        Ok(()) => info!("Successfully unsubscribed from chainlink_prices"),
        Err(e) => warn!(error = %e, "Failed to unsubscribe from chainlink_prices"),
    }

    // Unsubscribe from comments (wildcard)
    info!("Unsubscribing from comments");
    match client.unsubscribe_comments(None) {
        Ok(()) => info!("Successfully unsubscribed from comments"),
        Err(e) => warn!(error = %e, "Failed to unsubscribe from comments"),
    }

    // Show final subscription count after unsubscribe
    let sub_count = client.subscription_count();
    info!(
        endpoint = "subscription_count",
        count = sub_count,
        "After unsubscribe"
    );

    Ok(())
}



================================================
FILE: examples/clob/async.rs
================================================
//! Demonstrates async concurrency patterns with the CLOB client.
//!
//! This example shows how to:
//! 1. Run multiple unauthenticated API calls concurrently
//! 2. Run multiple authenticated API calls concurrently
//! 3. Spawn background tasks that share the client
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example async --features clob,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=async.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example async --features clob,tracing
//! ```
//!
//! For authenticated endpoints, set the `POLY_PRIVATE_KEY` environment variable.

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::types::U256;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tokio::join;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let (unauthenticated, authenticated) = join!(unauthenticated(), authenticated());
    unauthenticated?;
    authenticated
}

async fn unauthenticated() -> anyhow::Result<()> {
    let client = Client::new("https://clob.polymarket.com", Config::default())?;
    let client_clone = client.clone();

    let token_id = U256::from_str(
        "42334954850219754195241248003172889699504912694714162671145392673031415571339",
    )?;

    let thread = tokio::spawn(async move {
        let (ok_result, tick_result, neg_risk_result) = join!(
            client_clone.ok(),
            client_clone.tick_size(token_id),
            client_clone.neg_risk(token_id)
        );

        match ok_result {
            Ok(s) => info!(endpoint = "ok", thread = true, result = %s),
            Err(e) => error!(endpoint = "ok", thread = true, error = %e),
        }

        match tick_result {
            Ok(t) => info!(endpoint = "tick_size", thread = true, tick_size = ?t.minimum_tick_size),
            Err(e) => error!(endpoint = "tick_size", thread = true, error = %e),
        }

        match neg_risk_result {
            Ok(n) => info!(endpoint = "neg_risk", thread = true, neg_risk = n.neg_risk),
            Err(e) => error!(endpoint = "neg_risk", thread = true, error = %e),
        }

        anyhow::Ok(())
    });

    match client.ok().await {
        Ok(s) => info!(endpoint = "ok", result = %s),
        Err(e) => error!(endpoint = "ok", error = %e),
    }

    match client.tick_size(token_id).await {
        Ok(t) => {
            info!(endpoint = "tick_size", token_id = %token_id, tick_size = ?t.minimum_tick_size);
        }
        Err(e) => error!(endpoint = "tick_size", token_id = %token_id, error = %e),
    }

    match client.neg_risk(token_id).await {
        Ok(n) => info!(endpoint = "neg_risk", token_id = %token_id, neg_risk = n.neg_risk),
        Err(e) => error!(endpoint = "neg_risk", token_id = %token_id, error = %e),
    }

    thread.await?
}

async fn authenticated() -> anyhow::Result<()> {
    let Ok(private_key) = std::env::var(PRIVATE_KEY_VAR) else {
        info!(
            endpoint = "authenticated",
            "skipped - POLY_PRIVATE_KEY not set"
        );
        return Ok(());
    };
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;
    let client_clone = client.clone();

    let thread = tokio::spawn(async move {
        let (ok_result, api_keys_result) = join!(client_clone.ok(), client_clone.api_keys());

        match ok_result {
            Ok(s) => info!(endpoint = "ok", thread = true, authenticated = true, result = %s),
            Err(e) => error!(endpoint = "ok", thread = true, authenticated = true, error = %e),
        }

        match api_keys_result {
            Ok(keys) => info!(endpoint = "api_keys", thread = true, result = ?keys),
            Err(e) => error!(endpoint = "api_keys", thread = true, error = %e),
        }

        anyhow::Ok(())
    });

    match client.ok().await {
        Ok(s) => info!(endpoint = "ok", authenticated = true, result = %s),
        Err(e) => error!(endpoint = "ok", authenticated = true, error = %e),
    }

    match client.api_keys().await {
        Ok(keys) => info!(endpoint = "api_keys", result = ?keys),
        Err(e) => error!(endpoint = "api_keys", error = %e),
    }

    thread.await?
}



================================================
FILE: examples/clob/authenticated.rs
================================================
//! Comprehensive authenticated CLOB API endpoint explorer.
//!
//! This example tests authenticated CLOB API endpoints including:
//! 1. API key management and account status
//! 2. Market and limit order creation
//! 3. Order management (fetch, cancel)
//! 4. Balance and allowance operations
//! 5. Trades and rewards queries
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example authenticated --features clob,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=authenticated.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example authenticated --features clob,tracing
//! ```
//!
//! Requires `POLY_PRIVATE_KEY` environment variable to be set.

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use chrono::{TimeDelta, Utc};
use polymarket_client_sdk::clob::types::request::{
    BalanceAllowanceRequest, OrdersRequest, TradesRequest, UpdateBalanceAllowanceRequest,
    UserRewardsEarningRequest,
};
use polymarket_client_sdk::clob::types::{Amount, OrderType, Side};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::types::{Decimal, U256};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use rust_decimal_macros::dec;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let token_id = U256::from_str(
        "15871154585880608648532107628464183779895785213830018178010423617714102767076",
    )?;

    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need POLY_PRIVATE_KEY");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let config = Config::builder().use_server_time(true).build();
    let client = Client::new("https://clob.polymarket.com", config)?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    match client.api_keys().await {
        Ok(keys) => info!(endpoint = "api_keys", result = ?keys),
        Err(e) => error!(endpoint = "api_keys", error = %e),
    }

    match client.closed_only_mode().await {
        Ok(status) => info!(
            endpoint = "closed_only_mode",
            closed_only = status.closed_only
        ),
        Err(e) => error!(endpoint = "closed_only_mode", error = %e),
    }

    // Market order
    let market_order = client
        .market_order()
        .token_id(token_id)
        .amount(Amount::usdc(Decimal::ONE_HUNDRED)?)
        .side(Side::Buy)
        .build()
        .await?;
    let signed_order = client.sign(&signer, market_order).await?;
    match client.post_order(signed_order).await {
        Ok(r) => {
            info!(endpoint = "post_order", order_type = "market", order_id = %r.order_id, success = r.success);
        }
        Err(e) => error!(endpoint = "post_order", order_type = "market", error = %e),
    }

    // Limit order
    let limit_order = client
        .limit_order()
        .token_id(token_id)
        .order_type(OrderType::GTD)
        .expiration(Utc::now() + TimeDelta::days(2))
        .price(dec!(0.5))
        .size(Decimal::ONE_HUNDRED)
        .side(Side::Buy)
        .build()
        .await?;
    let signed_order = client.sign(&signer, limit_order).await?;
    match client.post_order(signed_order).await {
        Ok(r) => {
            info!(endpoint = "post_order", order_type = "limit", order_id = %r.order_id, success = r.success);
        }
        Err(e) => error!(endpoint = "post_order", order_type = "limit", error = %e),
    }

    match client.notifications().await {
        Ok(n) => info!(endpoint = "notifications", count = n.len()),
        Err(e) => error!(endpoint = "notifications", error = %e),
    }

    match client
        .balance_allowance(BalanceAllowanceRequest::default())
        .await
    {
        Ok(b) => info!(endpoint = "balance_allowance", result = ?b),
        Err(e) => error!(endpoint = "balance_allowance", error = %e),
    }

    match client
        .update_balance_allowance(UpdateBalanceAllowanceRequest::default())
        .await
    {
        Ok(b) => info!(endpoint = "update_balance_allowance", result = ?b),
        Err(e) => error!(endpoint = "update_balance_allowance", error = %e),
    }

    let order_id = "0xa1449ec0831c7d62f887c4653d0917f2445783ff30f0ca713d99c667fef17f2c";
    match client.order(order_id).await {
        Ok(o) => info!(endpoint = "order", order_id = %order_id, status = ?o.status),
        Err(e) => error!(endpoint = "order", order_id = %order_id, error = %e),
    }

    match client.orders(&OrdersRequest::default(), None).await {
        Ok(orders) => info!(endpoint = "orders", count = orders.data.len()),
        Err(e) => error!(endpoint = "orders", error = %e),
    }

    match client.cancel_order(order_id).await {
        Ok(r) => info!(endpoint = "cancel_order", order_id = %order_id, result = ?r),
        Err(e) => error!(endpoint = "cancel_order", order_id = %order_id, error = %e),
    }

    match client.cancel_orders(&[order_id]).await {
        Ok(r) => info!(endpoint = "cancel_orders", result = ?r),
        Err(e) => error!(endpoint = "cancel_orders", error = %e),
    }

    match client.cancel_all_orders().await {
        Ok(r) => info!(endpoint = "cancel_all_orders", result = ?r),
        Err(e) => error!(endpoint = "cancel_all_orders", error = %e),
    }

    match client.orders(&OrdersRequest::default(), None).await {
        Ok(orders) => info!(
            endpoint = "orders",
            after_cancel = true,
            count = orders.data.len()
        ),
        Err(e) => error!(endpoint = "orders", after_cancel = true, error = %e),
    }

    match client.trades(&TradesRequest::default(), None).await {
        Ok(trades) => info!(endpoint = "trades", count = trades.data.len()),
        Err(e) => error!(endpoint = "trades", error = %e),
    }

    match client
        .earnings_for_user_for_day(Utc::now().date_naive(), None)
        .await
    {
        Ok(e) => info!(endpoint = "earnings_for_user_for_day", result = ?e),
        Err(e) => error!(endpoint = "earnings_for_user_for_day", error = %e),
    }

    let request = UserRewardsEarningRequest::builder()
        .date(Utc::now().date_naive() - TimeDelta::days(30))
        .build();
    match client
        .user_earnings_and_markets_config(&request, None)
        .await
    {
        Ok(e) => info!(endpoint = "user_earnings_and_markets_config", result = ?e),
        Err(e) => error!(endpoint = "user_earnings_and_markets_config", error = %e),
    }

    match client.reward_percentages().await {
        Ok(r) => info!(endpoint = "reward_percentages", result = ?r),
        Err(e) => error!(endpoint = "reward_percentages", error = %e),
    }

    match client.current_rewards(None).await {
        Ok(r) => info!(endpoint = "current_rewards", result = ?r),
        Err(e) => error!(endpoint = "current_rewards", error = %e),
    }

    let market_id = "0x5f65177b394277fd294cd75650044e32ba009a95022d88a0c1d565897d72f8f1";
    match client.raw_rewards_for_market(market_id, None).await {
        Ok(r) => info!(endpoint = "raw_rewards_for_market", market_id = %market_id, result = ?r),
        Err(e) => error!(endpoint = "raw_rewards_for_market", market_id = %market_id, error = %e),
    }

    Ok(())
}



================================================
FILE: examples/clob/aws_authenticated.rs
================================================
//! Demonstrates AWS KMS-based authentication with the CLOB client.
//!
//! This example shows how to:
//! 1. Configure AWS SDK and KMS client
//! 2. Create an `AwsSigner` using a KMS key
//! 3. Authenticate with the CLOB API using the AWS signer
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example aws_authenticated --features clob,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=aws_authenticated.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example aws_authenticated --features clob,tracing
//! ```
//!
//! Requires AWS credentials configured and a valid KMS key ID.

use std::fs::File;

use alloy::signers::Signer as _;
use alloy::signers::aws::AwsSigner;
use aws_config::BehaviorVersion;
use polymarket_client_sdk::POLYGON;
use polymarket_client_sdk::clob::{Client, Config};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&config);

    let key_id = "<your key ID>".to_owned();
    info!(endpoint = "aws_signer", key_id = %key_id, "creating AWS KMS signer");

    let alloy_signer = AwsSigner::new(kms_client, key_id, Some(POLYGON))
        .await?
        .with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&alloy_signer)
        .authenticate()
        .await?;

    match client.api_keys().await {
        Ok(keys) => info!(endpoint = "api_keys", result = ?keys),
        Err(e) => error!(endpoint = "api_keys", error = %e),
    }

    Ok(())
}



================================================
FILE: examples/clob/builder_authenticated.rs
================================================
//! Demonstrates builder API authentication with the CLOB client.
//!
//! This example shows how to:
//! 1. Authenticate as a regular user
//! 2. Create builder API credentials
//! 3. Promote the client to a builder client
//! 4. Access builder-specific endpoints
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example builder_authenticated --features clob,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=builder_authenticated.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example builder_authenticated --features clob,tracing
//! ```
//!
//! Requires `POLY_PRIVATE_KEY` environment variable to be set.

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::auth::builder::Config as BuilderConfig;
use polymarket_client_sdk::clob::types::request::TradesRequest;
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::types::U256;
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need POLY_PRIVATE_KEY");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    // Create builder credentials and promote to builder client
    let builder_credentials = client.create_builder_api_key().await?;
    info!(
        endpoint = "create_builder_api_key",
        "created builder credentials"
    );

    let config = BuilderConfig::local(builder_credentials);
    let client = client.promote_to_builder(config).await?;
    info!(
        endpoint = "promote_to_builder",
        "promoted to builder client"
    );

    match client.builder_api_keys().await {
        Ok(keys) => info!(endpoint = "builder_api_keys", count = keys.len()),
        Err(e) => error!(endpoint = "builder_api_keys", error = %e),
    }

    let token_id = U256::from_str(
        "15871154585880608648532107628464183779895785213830018178010423617714102767076",
    )?;
    let request = TradesRequest::builder().asset_id(token_id).build();

    match client.builder_trades(&request, None).await {
        Ok(trades) => {
            info!(endpoint = "builder_trades", token_id = %token_id, count = trades.data.len());
        }
        Err(e) => error!(endpoint = "builder_trades", token_id = %token_id, error = %e),
    }

    Ok(())
}



================================================
FILE: examples/clob/heartbeats.rs
================================================
//! Shows how heartbeats are sent automatically when the corresponding feature flag is enabled.
//!
//! Run with:
//! ```sh
//! RUST_LOG=debug,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example heartbeats --features heartbeats,tracing
//! ```
//!
use std::str::FromStr as _;
use std::time::Duration;

use polymarket_client_sdk::auth::{LocalSigner, Signer as _};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let config = Config::builder()
        .use_server_time(true)
        .heartbeat_interval(Duration::from_secs(1))
        .build();
    let client = Client::new("https://clob.polymarket.com", config)?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    drop(client);

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}



================================================
FILE: examples/clob/streaming.rs
================================================
//! CLOB API streaming endpoint explorer.
//!
//! This example demonstrates streaming data from CLOB API endpoints by:
//! 1. Streaming `sampling_markets` (unauthenticated) to discover market data
//! 2. Streaming trades (authenticated) if credentials are available
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example streaming --features tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=streaming.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example streaming --features tracing
//! ```
//!
//! For authenticated streaming, set the `POLY_PRIVATE_KEY` environment variable:
//! ```sh
//! POLY_PRIVATE_KEY=0x... RUST_LOG=info cargo run --example streaming --features tracing
//! ```

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use futures::{StreamExt as _, future};
use polymarket_client_sdk::clob::types::request::TradesRequest;
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tokio::join;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let (unauthenticated, authenticated) = join!(unauthenticated(), authenticated());
    unauthenticated?;
    authenticated
}

async fn unauthenticated() -> anyhow::Result<()> {
    let client = Client::new("https://clob.polymarket.com", Config::default())?;

    info!(
        stream = "sampling_markets",
        "starting unauthenticated stream"
    );

    let mut stream = client
        .stream_data(Client::sampling_markets)
        .filter_map(|d| future::ready(d.ok()))
        .boxed();

    let mut count = 0_u32;

    while let Some(market) = stream.next().await {
        count += 1;

        // Log every 100th market to avoid flooding logs
        if count % 100 == 1 {
            if let Some(cid) = &market.condition_id {
                info!(
                    stream = "sampling_markets",
                    count = count,
                    condition_id = %cid,
                    question = %market.question,
                    active = market.active
                );
            } else {
                info!(
                    stream = "sampling_markets",
                    count = count,
                    question = %market.question,
                    active = market.active
                );
            }
        }
    }

    info!(
        stream = "sampling_markets",
        total_markets = count,
        "stream completed"
    );

    Ok(())
}

async fn authenticated() -> anyhow::Result<()> {
    let Ok(private_key) = std::env::var(PRIVATE_KEY_VAR) else {
        warn!(
            stream = "trades",
            "skipping authenticated stream - {} not set", PRIVATE_KEY_VAR
        );
        return Ok(());
    };

    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    info!(stream = "trades", "starting authenticated stream");

    let request = TradesRequest::builder().build();
    let mut stream = client
        .stream_data(|c, cursor| c.trades(&request, cursor))
        .boxed();

    let mut count = 0_u32;

    while let Some(result) = stream.next().await {
        match result {
            Ok(trade) => {
                count += 1;

                // Log every 100th trade to avoid flooding logs
                if count % 100 == 1 {
                    info!(
                        stream = "trades",
                        count = count,
                        market = %trade.market,
                        side = ?trade.side,
                        size = %trade.size,
                        price = %trade.price
                    );
                }
            }
            Err(e) => {
                debug!(stream = "trades", error = %e, "stream error");
            }
        }
    }

    info!(stream = "trades", total_trades = count, "stream completed");

    Ok(())
}



================================================
FILE: examples/clob/unauthenticated.rs
================================================
//! Comprehensive CLOB API endpoint explorer (unauthenticated).
//!
//! This example dynamically tests all unauthenticated CLOB API endpoints by:
//! 1. Fetching markets to discover real token IDs and condition IDs
//! 2. Using those IDs for subsequent price, orderbook, and trade queries
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example unauthenticated --features tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=clob.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example unauthenticated --features tracing
//! ```

use std::collections::HashMap;
use std::fs::File;

use futures_util::StreamExt as _;
use polymarket_client_sdk::clob::types::Side;
use polymarket_client_sdk::clob::types::request::{
    LastTradePriceRequest, MidpointRequest, OrderBookSummaryRequest, PriceRequest, SpreadRequest,
};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::types::{B256, Decimal, U256};
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// Finds a market with an active orderbook by streaming through all markets.
///
/// Returns a tuple of (`token_id`, `condition_id`) from a market that:
/// - Has orderbook enabled (`enable_order_book` = true)
/// - Is active and not closed
/// - Is accepting orders
/// - Has tokens with non-zero prices
///
/// This ensures subsequent price/midpoint/orderbook API calls will succeed.
async fn find_market_with_orderbook(client: &Client) -> anyhow::Result<(U256, B256)> {
    info!("Searching for a market with an active orderbook...");

    let mut stream = Box::pin(client.stream_data(Client::markets));

    while let Some(maybe_market) = stream.next().await {
        match maybe_market {
            Ok(market) => {
                if market.enable_order_book
                    && market.active
                    && !market.closed
                    && !market.archived
                    && market.accepting_orders
                    && !market.tokens.is_empty()
                    && market.tokens.iter().any(|t| t.price > Decimal::ZERO)
                {
                    let condition_id = market
                        .condition_id
                        .ok_or_else(|| anyhow::anyhow!("Market missing condition_id"))?;
                    let token_id = market
                        .tokens
                        .first()
                        .map(|t| t.token_id)
                        .ok_or_else(|| anyhow::anyhow!("Market has no tokens"))?;

                    let request = MidpointRequest::builder().token_id(token_id).build();
                    if client.midpoint(&request).await.is_ok() {
                        info!(
                            condition_id = %condition_id,
                            token_id = %token_id,
                            question = %market.question,
                            "Found market with active orderbook"
                        );

                        return Ok((token_id, condition_id));
                    }
                }
            }
            Err(e) => {
                error!(error = ?e, "Error fetching market");
            }
        }
    }

    Err(anyhow::anyhow!(
        "No active markets with orderbooks found after searching all markets"
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::new("https://clob.polymarket.com", Config::default())?;

    // Health check endpoints
    match client.ok().await {
        Ok(_) => info!(endpoint = "ok", status = "healthy"),
        Err(e) => error!(endpoint = "ok", error = %e),
    }

    match client.server_time().await {
        Ok(time) => info!(endpoint = "server_time", timestamp = %time),
        Err(e) => error!(endpoint = "server_time", error = %e),
    }

    let (token_id, condition_id) = match find_market_with_orderbook(&client).await {
        Ok((tid, cid)) => (Some(tid), Some(cid)),
        Err(e) => {
            error!("Failed to find market with orderbook: {}", e);
            (None, None)
        }
    };

    if let Some(cid) = &condition_id {
        match client.market(&cid.to_string()).await {
            Ok(market) => info!(
                endpoint = "market",
                condition_id = %cid,
                question = %market.question,
                active = market.active
            ),
            Err(e) => error!(endpoint = "market", condition_id = %cid, error = %e),
        }
    }

    match client.sampling_markets(None).await {
        Ok(page) => info!(
            endpoint = "sampling_markets",
            count = page.data.len(),
            has_next = !page.next_cursor.is_empty()
        ),
        Err(e) => error!(endpoint = "sampling_markets", error = %e),
    }

    match client.simplified_markets(None).await {
        Ok(page) => info!(
            endpoint = "simplified_markets",
            count = page.data.len(),
            has_next = !page.next_cursor.is_empty()
        ),
        Err(e) => error!(endpoint = "simplified_markets", error = %e),
    }

    match client.sampling_simplified_markets(None).await {
        Ok(page) => info!(
            endpoint = "sampling_simplified_markets",
            count = page.data.len(),
            has_next = !page.next_cursor.is_empty()
        ),
        Err(e) => error!(endpoint = "sampling_simplified_markets", error = %e),
    }

    if let Some(token_id) = token_id {
        let midpoint_request = MidpointRequest::builder().token_id(token_id).build();
        match client.midpoint(&midpoint_request).await {
            Ok(midpoint) => info!(endpoint = "midpoint", token_id = %token_id, mid = %midpoint.mid),
            Err(e) => error!(endpoint = "midpoint", token_id = %token_id, error = %e),
        }

        match client.midpoints(&[midpoint_request]).await {
            Ok(midpoints) => info!(endpoint = "midpoints", count = midpoints.midpoints.len()),
            Err(e) => error!(endpoint = "midpoints", error = %e),
        }

        let buy_price_request = PriceRequest::builder()
            .token_id(token_id)
            .side(Side::Buy)
            .build();
        match client.price(&buy_price_request).await {
            Ok(price) => info!(
                endpoint = "price",
                token_id = %token_id,
                side = "buy",
                price = %price.price
            ),
            Err(e) => error!(endpoint = "price", token_id = %token_id, side = "buy", error = %e),
        }

        let sell_price_request = PriceRequest::builder()
            .token_id(token_id)
            .side(Side::Sell)
            .build();
        match client.price(&sell_price_request).await {
            Ok(price) => info!(
                endpoint = "price",
                token_id = %token_id,
                side = "sell",
                price = %price.price
            ),
            Err(e) => error!(endpoint = "price", token_id = %token_id, side = "sell", error = %e),
        }

        match client
            .prices(&[buy_price_request, sell_price_request])
            .await
        {
            Ok(prices) => info!(
                endpoint = "prices",
                count = prices.prices.as_ref().map_or(0, HashMap::len)
            ),
            Err(e) => error!(endpoint = "prices", error = %e),
        }

        let spread_request = SpreadRequest::builder().token_id(token_id).build();
        match client.spread(&spread_request).await {
            Ok(spread) => info!(
                endpoint = "spread",
                token_id = %token_id,
                spread = %spread.spread
            ),
            Err(e) => error!(endpoint = "spread", token_id = %token_id, error = %e),
        }

        match client.spreads(&[spread_request]).await {
            Ok(spreads) => info!(
                endpoint = "spreads",
                count = spreads.spreads.as_ref().map_or(0, HashMap::len)
            ),
            Err(e) => error!(endpoint = "spreads", error = %e),
        }

        match client.tick_size(token_id).await {
            Ok(tick_size) => info!(
                endpoint = "tick_size",
                token_id = %token_id,
                tick_size = %tick_size.minimum_tick_size
            ),
            Err(e) => error!(endpoint = "tick_size", token_id = %token_id, error = %e),
        }

        match client.neg_risk(token_id).await {
            Ok(neg_risk) => info!(
                endpoint = "neg_risk",
                token_id = %token_id,
                neg_risk = neg_risk.neg_risk
            ),
            Err(e) => error!(endpoint = "neg_risk", token_id = %token_id, error = %e),
        }

        match client.fee_rate_bps(token_id).await {
            Ok(fee_rate) => info!(
                endpoint = "fee_rate_bps",
                token_id = %token_id,
                base_fee = fee_rate.base_fee
            ),
            Err(e) => error!(endpoint = "fee_rate_bps", token_id = %token_id, error = %e),
        }

        let order_book_request = OrderBookSummaryRequest::builder()
            .token_id(token_id)
            .build();
        match client.order_book(&order_book_request).await {
            Ok(book) => {
                let hash = book.hash().unwrap_or_default();
                info!(
                    endpoint = "order_book",
                    token_id = %token_id,
                    bids = book.bids.len(),
                    asks = book.asks.len(),
                    hash = %hash
                );
            }
            Err(e) => error!(endpoint = "order_book", token_id = %token_id, error = %e),
        }

        match client.order_books(&[order_book_request]).await {
            Ok(books) => info!(endpoint = "order_books", count = books.len()),
            Err(e) => error!(endpoint = "order_books", error = %e),
        }

        let last_trade_request = LastTradePriceRequest::builder().token_id(token_id).build();
        match client.last_trade_price(&last_trade_request).await {
            Ok(last_trade) => info!(
                endpoint = "last_trade_price",
                token_id = %token_id,
                price = %last_trade.price
            ),
            Err(e) => error!(endpoint = "last_trade_price", token_id = %token_id, error = %e),
        }

        match client.last_trades_prices(&[last_trade_request]).await {
            Ok(prices) => info!(endpoint = "last_trade_prices", count = prices.len()),
            Err(e) => error!(endpoint = "last_trade_prices", error = %e),
        }
    } else {
        warn!(
            endpoint = "price_queries",
            "skipped - no token_id discovered"
        );
    }

    Ok(())
}



================================================
FILE: examples/clob/rfq/quotes.rs
================================================
//! Demonstrates fetching RFQ quotes from the CLOB API.
//!
//! This example shows how to:
//! 1. Authenticate with the CLOB API
//! 2. Build an RFQ quotes request with filters
//! 3. Fetch and display paginated quote results
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example rfq_quotes --features clob,rfq,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=rfq_quotes.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example rfq_quotes --features clob,rfq,tracing
//! ```
//!
//! Requires `POLY_PRIVATE_KEY` environment variable to be set.

#![cfg(feature = "rfq")]

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::clob::types::{RfqQuotesRequest, RfqSortBy, RfqSortDir, RfqState};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need POLY_PRIVATE_KEY");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let request = RfqQuotesRequest::builder()
        .state(RfqState::Active)
        .limit(10)
        .offset("MA==")
        .sort_by(RfqSortBy::Price)
        .sort_dir(RfqSortDir::Asc)
        .build();

    match client.quotes(&request, None).await {
        Ok(quotes) => {
            info!(
                endpoint = "quotes",
                count = quotes.count,
                data_len = quotes.data.len(),
                next_cursor = %quotes.next_cursor
            );
            for quote in &quotes.data {
                debug!(endpoint = "quotes", quote = ?quote);
            }
        }
        Err(e) => error!(endpoint = "quotes", error = %e),
    }

    Ok(())
}



================================================
FILE: examples/clob/rfq/requests.rs
================================================
//! Demonstrates fetching RFQ requests from the CLOB API.
//!
//! This example shows how to:
//! 1. Authenticate with the CLOB API
//! 2. Build an RFQ requests query with filters
//! 3. Fetch and display paginated request results
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example rfq_requests --features clob,rfq,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=rfq_requests.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example rfq_requests --features clob,rfq,tracing
//! ```
//!
//! Requires `POLY_PRIVATE_KEY` environment variable to be set.

#![cfg(feature = "rfq")]

use std::fs::File;
use std::str::FromStr as _;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use polymarket_client_sdk::clob::types::{RfqRequestsRequest, RfqSortBy, RfqSortDir, RfqState};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need POLY_PRIVATE_KEY");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));

    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let request = RfqRequestsRequest::builder()
        .state(RfqState::Active)
        .limit(10)
        .offset("MA==")
        .sort_by(RfqSortBy::Created)
        .sort_dir(RfqSortDir::Desc)
        .build();

    match client.requests(&request, None).await {
        Ok(requests) => {
            info!(
                endpoint = "requests",
                count = requests.count,
                data_len = requests.data.len(),
                next_cursor = %requests.next_cursor
            );
            for req in &requests.data {
                debug!(endpoint = "requests", request = ?req);
            }
        }
        Err(e) => error!(endpoint = "requests", error = %e),
    }

    Ok(())
}



================================================
FILE: examples/clob/ws/orderbook.rs
================================================
//! Demonstrates subscribing to real-time orderbook updates via WebSocket.
//!
//! This example shows how to:
//! 1. Connect to the CLOB WebSocket API
//! 2. Subscribe to orderbook updates for multiple assets
//! 3. Process and display bid/ask updates in real-time
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_orderbook --features ws,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=websocket_orderbook.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_orderbook --features ws,tracing
//! ```

use std::fs::File;
use std::str::FromStr as _;

use futures::StreamExt as _;
use polymarket_client_sdk::clob::ws::Client;
use polymarket_client_sdk::types::U256;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();
    info!(endpoint = "websocket", "connected to CLOB WebSocket API");

    let asset_ids = vec![
        U256::from_str(
            "92703761682322480664976766247614127878023988651992837287050266308961660624165",
        )?,
        U256::from_str(
            "34551606549875928972193520396544368029176529083448203019529657908155427866742",
        )?,
    ];

    let stream = client.subscribe_orderbook(asset_ids.clone())?;
    let mut stream = Box::pin(stream);
    info!(
        endpoint = "subscribe_orderbook",
        asset_count = asset_ids.len(),
        "subscribed to orderbook updates"
    );

    while let Some(book_result) = stream.next().await {
        match book_result {
            Ok(book) => {
                info!(
                    endpoint = "orderbook",
                    asset_id = %book.asset_id,
                    market = %book.market,
                    timestamp = %book.timestamp,
                    bids = book.bids.len(),
                    asks = book.asks.len()
                );

                for (i, bid) in book.bids.iter().take(5).enumerate() {
                    debug!(
                        endpoint = "orderbook",
                        side = "bid",
                        rank = i + 1,
                        size = %bid.size,
                        price = %bid.price
                    );
                }

                for (i, ask) in book.asks.iter().take(5).enumerate() {
                    debug!(
                        endpoint = "orderbook",
                        side = "ask",
                        rank = i + 1,
                        size = %ask.size,
                        price = %ask.price
                    );
                }

                if let Some(hash) = &book.hash {
                    debug!(endpoint = "orderbook", hash = %hash);
                }
            }
            Err(e) => error!(endpoint = "orderbook", error = %e),
        }
    }

    Ok(())
}



================================================
FILE: examples/clob/ws/unsubscribe.rs
================================================
//! Demonstrates WebSocket subscribe/unsubscribe and multiplexing behavior.
//!
//! This example shows how to:
//! 1. Subscribe multiple streams to the same asset (multiplexing)
//! 2. Unsubscribe streams while others remain active
//! 3. Verify reference counting works correctly
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_unsubscribe --features ws,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=websocket_unsubscribe.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_unsubscribe --features ws,tracing
//! ```
//!
//! With debug level, you can see subscribe/unsubscribe wire messages:
//! ```sh
//! RUST_LOG=debug,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_unsubscribe --features ws,tracing
//! ```

use std::fs::File;
use std::str::FromStr as _;
use std::time::Duration;

use futures::StreamExt as _;
use polymarket_client_sdk::clob::ws::Client;
use polymarket_client_sdk::types::U256;
use tokio::time::timeout;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();
    info!(endpoint = "websocket", "connected to CLOB WebSocket API");

    let asset_ids = vec![U256::from_str(
        "92703761682322480664976766247614127878023988651992837287050266308961660624165",
    )?];

    // === FIRST SUBSCRIPTION ===
    info!(
        step = 1,
        "first subscription - should send 'subscribe' to server"
    );
    let stream1 = client.subscribe_orderbook(asset_ids.clone())?;
    let mut stream1 = Box::pin(stream1);

    match timeout(Duration::from_secs(10), stream1.next()).await {
        Ok(Some(Ok(book))) => {
            info!(
                step = 1,
                endpoint = "orderbook",
                bids = book.bids.len(),
                asks = book.asks.len(),
                "received update on stream1"
            );
        }
        Ok(Some(Err(e))) => error!(step = 1, error = %e),
        Ok(None) => error!(step = 1, "stream ended"),
        Err(_) => error!(step = 1, "timeout"),
    }

    // === SECOND SUBSCRIPTION (same asset - should multiplex) ===
    info!(
        step = 2,
        "second subscription (same asset) - should NOT send message (multiplexing)"
    );
    let stream2 = client.subscribe_orderbook(asset_ids.clone())?;
    let mut stream2 = Box::pin(stream2);

    match timeout(Duration::from_secs(10), stream2.next()).await {
        Ok(Some(Ok(book))) => {
            info!(
                step = 2,
                endpoint = "orderbook",
                bids = book.bids.len(),
                asks = book.asks.len(),
                "received update on stream2"
            );
        }
        Ok(Some(Err(e))) => error!(step = 2, error = %e),
        Ok(None) => error!(step = 2, "stream ended"),
        Err(_) => error!(step = 2, "timeout"),
    }

    // === FIRST UNSUBSCRIBE ===
    info!(
        step = 3,
        "first unsubscribe - should NOT send message (refcount still 1)"
    );
    client.unsubscribe_orderbook(&asset_ids)?;
    drop(stream1);
    info!(step = 3, "stream1 unsubscribed and dropped");

    // stream2 should still work
    match timeout(Duration::from_secs(10), stream2.next()).await {
        Ok(Some(Ok(book))) => {
            info!(
                step = 3,
                endpoint = "orderbook",
                bids = book.bids.len(),
                asks = book.asks.len(),
                "stream2 still receiving updates"
            );
        }
        Ok(Some(Err(e))) => error!(step = 3, error = %e),
        Ok(None) => error!(step = 3, "stream ended"),
        Err(_) => error!(step = 3, "timeout"),
    }

    // === SECOND UNSUBSCRIBE ===
    info!(
        step = 4,
        "second unsubscribe - should send 'unsubscribe' (refcount now 0)"
    );
    client.unsubscribe_orderbook(&asset_ids)?;
    drop(stream2);
    info!(step = 4, "stream2 unsubscribed and dropped");

    // === RE-SUBSCRIBE (proves unsubscribe worked) ===
    info!(
        step = 5,
        "re-subscribe - should send 'subscribe' (proves unsubscribe worked)"
    );
    let stream3 = client.subscribe_orderbook(asset_ids)?;
    let mut stream3 = Box::pin(stream3);

    match timeout(Duration::from_secs(10), stream3.next()).await {
        Ok(Some(Ok(book))) => {
            info!(
                step = 5,
                endpoint = "orderbook",
                bids = book.bids.len(),
                asks = book.asks.len(),
                "stream3 receiving updates"
            );
        }
        Ok(Some(Err(e))) => error!(step = 5, error = %e),
        Ok(None) => error!(step = 5, "stream ended"),
        Err(_) => error!(step = 5, "timeout"),
    }

    info!("example complete");
    debug!(
        "with debug logging, you should see subscribe/unsubscribe wire messages at steps 1, 4, and 5"
    );

    Ok(())
}



================================================
FILE: examples/clob/ws/user.rs
================================================
//! Demonstrates subscribing to authenticated user WebSocket channels.
//!
//! This example shows how to:
//! 1. Build credentials for authenticated WebSocket access
//! 2. Subscribe to user-specific order and trade events
//! 3. Process real-time order updates and trade notifications
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_user --features ws,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=websocket_user.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example websocket_user --features ws,tracing
//! ```
//!
//! Requires the following environment variables:
//! - `POLYMARKET_API_KEY`
//! - `POLYMARKET_API_SECRET`
//! - `POLYMARKET_API_PASSPHRASE`
//! - `POLYMARKET_ADDRESS`

use std::fs::File;
use std::str::FromStr as _;

use futures::StreamExt as _;
use polymarket_client_sdk::auth::Credentials;
use polymarket_client_sdk::clob::ws::{Client, WsMessage};
use polymarket_client_sdk::types::{Address, B256};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let api_key = Uuid::parse_str(&std::env::var("POLYMARKET_API_KEY")?)?;
    let api_secret = std::env::var("POLYMARKET_API_SECRET")?;
    let api_passphrase = std::env::var("POLYMARKET_API_PASSPHRASE")?;
    let address = Address::from_str(&std::env::var("POLYMARKET_ADDRESS")?)?;

    let credentials = Credentials::new(api_key, api_secret, api_passphrase);

    let client = Client::default().authenticate(credentials, address)?;
    info!(
        endpoint = "websocket",
        authenticated = true,
        "connected to authenticated WebSocket"
    );

    // Provide specific market IDs, or leave empty for all events
    let markets: Vec<B256> = Vec::new();
    let mut stream = std::pin::pin!(client.subscribe_user_events(markets)?);
    info!(
        endpoint = "subscribe_user_events",
        "subscribed to user events"
    );

    while let Some(event) = stream.next().await {
        match event {
            Ok(WsMessage::Order(order)) => {
                info!(
                    endpoint = "user_events",
                    event_type = "order",
                    order_id = %order.id,
                    market = %order.market,
                    msg_type = ?order.msg_type,
                    side = ?order.side,
                    price = %order.price
                );
                if let Some(size) = &order.original_size {
                    debug!(endpoint = "user_events", original_size = %size);
                }
                if let Some(matched) = &order.size_matched {
                    debug!(endpoint = "user_events", size_matched = %matched);
                }
            }
            Ok(WsMessage::Trade(trade)) => {
                info!(
                    endpoint = "user_events",
                    event_type = "trade",
                    trade_id = %trade.id,
                    market = %trade.market,
                    status = ?trade.status,
                    side = ?trade.side,
                    size = %trade.size,
                    price = %trade.price
                );
                if let Some(trader_side) = &trade.trader_side {
                    debug!(endpoint = "user_events", trader_side = ?trader_side);
                }
            }
            Ok(other) => {
                debug!(endpoint = "user_events", event = ?other);
            }
            Err(e) => {
                error!(endpoint = "user_events", error = %e);
                break;
            }
        }
    }

    Ok(())
}



================================================
FILE: examples/gamma/client.rs
================================================
//! Comprehensive Gamma API endpoint explorer.
//!
//! This example dynamically tests all Gamma API endpoints by:
//! 1. Fetching lists first (events, markets, tags, etc.)
//! 2. Extracting real IDs/slugs from responses
//! 3. Using those IDs for subsequent lookups
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example gamma --features gamma,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=gamma.log RUST_LOG=info,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off cargo run --example gamma --features gamma,tracing
//! ```

use std::fs::File;

use polymarket_client_sdk::gamma::Client;
use polymarket_client_sdk::gamma::types::ParentEntityType;
use polymarket_client_sdk::gamma::types::request::{
    CommentsByIdRequest, CommentsByUserAddressRequest, CommentsRequest, EventByIdRequest,
    EventBySlugRequest, EventTagsRequest, EventsRequest, MarketByIdRequest, MarketBySlugRequest,
    MarketTagsRequest, MarketsRequest, PublicProfileRequest, RelatedTagsByIdRequest,
    RelatedTagsBySlugRequest, SearchRequest, SeriesByIdRequest, SeriesListRequest, TagByIdRequest,
    TagBySlugRequest, TagsRequest, TeamsRequest,
};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();

    match client.status().await {
        Ok(s) => info!(endpoint = "status", result = %s),
        Err(e) => debug!(endpoint = "status", error = %e),
    }

    match client.sports().await {
        Ok(v) => info!(endpoint = "sports", count = v.len()),
        Err(e) => debug!(endpoint = "sports", error = %e),
    }

    match client.sports_market_types().await {
        Ok(v) => info!(
            endpoint = "sports_market_types",
            count = v.market_types.len()
        ),
        Err(e) => debug!(endpoint = "sports_market_types", error = %e),
    }

    match client
        .teams(&TeamsRequest::builder().limit(5).build())
        .await
    {
        Ok(v) => info!(endpoint = "teams", count = v.len()),
        Err(e) => debug!(endpoint = "teams", error = %e),
    }

    let tags_result = client.tags(&TagsRequest::builder().limit(10).build()).await;
    match &tags_result {
        Ok(v) => info!(endpoint = "tags", count = v.len()),
        Err(e) => debug!(endpoint = "tags", error = %e),
    }

    // Use "politics" tag - known to have related tags
    let tag_slug = "politics";
    let tag_result = client
        .tag_by_slug(&TagBySlugRequest::builder().slug(tag_slug).build())
        .await;
    let tag_id = match &tag_result {
        Ok(tag) => {
            info!(endpoint = "tag_by_slug", slug = tag_slug, id = %tag.id);
            Some(tag.id.clone())
        }
        Err(e) => {
            debug!(endpoint = "tag_by_slug", slug = tag_slug, error = %e);
            None
        }
    };

    if let Some(id) = &tag_id {
        match client
            .tag_by_id(&TagByIdRequest::builder().id(id).build())
            .await
        {
            Ok(_) => info!(endpoint = "tag_by_id", id = %id),
            Err(e) => debug!(endpoint = "tag_by_id", id = %id, error = %e),
        }

        match client
            .related_tags_by_id(&RelatedTagsByIdRequest::builder().id(id).build())
            .await
        {
            Ok(v) => info!(endpoint = "related_tags_by_id", id = %id, count = v.len()),
            Err(e) => debug!(endpoint = "related_tags_by_id", id = %id, error = %e),
        }

        match client
            .tags_related_to_tag_by_id(&RelatedTagsByIdRequest::builder().id(id).build())
            .await
        {
            Ok(v) => info!(endpoint = "tags_related_to_tag_by_id", id = %id, count = v.len()),
            Err(e) => debug!(endpoint = "tags_related_to_tag_by_id", id = %id, error = %e),
        }
    }

    match client
        .related_tags_by_slug(&RelatedTagsBySlugRequest::builder().slug(tag_slug).build())
        .await
    {
        Ok(v) => info!(
            endpoint = "related_tags_by_slug",
            slug = tag_slug,
            count = v.len()
        ),
        Err(e) => debug!(endpoint = "related_tags_by_slug", slug = tag_slug, error = %e),
    }

    match client
        .tags_related_to_tag_by_slug(&RelatedTagsBySlugRequest::builder().slug(tag_slug).build())
        .await
    {
        Ok(v) => info!(
            endpoint = "tags_related_to_tag_by_slug",
            slug = tag_slug,
            count = v.len()
        ),
        Err(e) => debug!(endpoint = "tags_related_to_tag_by_slug", slug = tag_slug, error = %e),
    }

    let events_result = client
        .events(
            &EventsRequest::builder()
                .active(true)
                .limit(20)
                .order(vec!["volume".to_owned()])
                .ascending(false)
                .build(),
        )
        .await;

    // Find an event with comments
    let (event_with_comments, any_event) = match &events_result {
        Ok(events) => {
            info!(endpoint = "events", count = events.len());
            let with_comments = events
                .iter()
                .find(|e| e.comment_count.unwrap_or(0) > 0)
                .map(|e| (e.id.clone(), e.slug.clone(), e.comment_count.unwrap_or(0)));
            let any = events.first().map(|e| (e.id.clone(), e.slug.clone()));
            (with_comments, any)
        }
        Err(e) => {
            debug!(endpoint = "events", error = %e);
            (None, None)
        }
    };

    if let Some((event_id, event_slug)) = &any_event {
        match client
            .event_by_id(&EventByIdRequest::builder().id(event_id).build())
            .await
        {
            Ok(_) => info!(endpoint = "event_by_id", id = %event_id),
            Err(e) => debug!(endpoint = "event_by_id", id = %event_id, error = %e),
        }

        match client
            .event_tags(&EventTagsRequest::builder().id(event_id).build())
            .await
        {
            Ok(v) => info!(endpoint = "event_tags", id = %event_id, count = v.len()),
            Err(e) => debug!(endpoint = "event_tags", id = %event_id, error = %e),
        }

        if let Some(slug) = event_slug {
            match client
                .event_by_slug(&EventBySlugRequest::builder().slug(slug).build())
                .await
            {
                Ok(_) => info!(endpoint = "event_by_slug", slug = %slug),
                Err(e) => debug!(endpoint = "event_by_slug", slug = %slug, error = %e),
            }
        }
    }

    let markets_result = client
        .markets(&MarketsRequest::builder().closed(false).limit(10).build())
        .await;

    let (market_id, market_slug) = match &markets_result {
        Ok(markets) => {
            info!(endpoint = "markets", count = markets.len());
            markets
                .first()
                .map_or((None, None), |m| (Some(m.id.clone()), m.slug.clone()))
        }
        Err(e) => {
            debug!(endpoint = "markets", error = %e);
            (None, None)
        }
    };

    // Test multiple slugs - verifies repeated query params work (issue #147)
    if let Ok(markets) = &markets_result {
        let slugs: Vec<String> = markets
            .iter()
            .filter_map(|m| m.slug.clone())
            .take(3)
            .collect();

        if slugs.len() >= 2 {
            match client
                .markets(&MarketsRequest::builder().slug(slugs.clone()).build())
                .await
            {
                Ok(v) => info!(
                    endpoint = "markets_multiple_slugs",
                    slugs = ?slugs,
                    count = v.len(),
                    "verified repeated query params work"
                ),
                Err(e) => debug!(endpoint = "markets_multiple_slugs", slugs = ?slugs, error = %e),
            }
        }
    }

    if let Some(id) = &market_id {
        match client
            .market_by_id(&MarketByIdRequest::builder().id(id).build())
            .await
        {
            Ok(_) => info!(endpoint = "market_by_id", id = %id),
            Err(e) => debug!(endpoint = "market_by_id", id = %id, error = %e),
        }

        match client
            .market_tags(&MarketTagsRequest::builder().id(id).build())
            .await
        {
            Ok(v) => info!(endpoint = "market_tags", id = %id, count = v.len()),
            Err(e) => debug!(endpoint = "market_tags", id = %id, error = %e),
        }
    }

    if let Some(slug) = &market_slug {
        match client
            .market_by_slug(&MarketBySlugRequest::builder().slug(slug).build())
            .await
        {
            Ok(_) => info!(endpoint = "market_by_slug", slug = %slug),
            Err(e) => debug!(endpoint = "market_by_slug", slug = %slug, error = %e),
        }
    }

    let series_result = client
        .series(
            &SeriesListRequest::builder()
                .limit(10)
                .order("volume".to_owned())
                .ascending(false)
                .build(),
        )
        .await;

    let series_id = match &series_result {
        Ok(series) => {
            info!(endpoint = "series", count = series.len());
            series.first().map(|s| s.id.clone())
        }
        Err(e) => {
            debug!(endpoint = "series", error = %e);
            None
        }
    };

    if let Some(id) = &series_id {
        match client
            .series_by_id(&SeriesByIdRequest::builder().id(id).build())
            .await
        {
            Ok(_) => info!(endpoint = "series_by_id", id = %id),
            Err(e) => debug!(endpoint = "series_by_id", id = %id, error = %e),
        }
    }

    let (comment_id, user_address) = if let Some((event_id, _, comment_count)) =
        &event_with_comments
    {
        let comments_result = client
            .comments(
                &CommentsRequest::builder()
                    .parent_entity_type(ParentEntityType::Event)
                    .parent_entity_id(event_id)
                    .limit(10)
                    .build(),
            )
            .await;

        match &comments_result {
            Ok(comments) => {
                info!(endpoint = "comments", event_id = %event_id, expected = comment_count, count = comments.len());
                comments
                    .first()
                    .map_or((None, None), |c| (Some(c.id.clone()), c.user_address))
            }
            Err(e) => {
                debug!(endpoint = "comments", event_id = %event_id, error = %e);
                (None, None)
            }
        }
    } else {
        debug!(
            endpoint = "comments",
            "skipped - no event with comments found"
        );
        (None, None)
    };

    if let Some(id) = &comment_id {
        match client
            .comments_by_id(&CommentsByIdRequest::builder().id(id).build())
            .await
        {
            Ok(v) => info!(endpoint = "comments_by_id", id = %id, count = v.len()),
            Err(e) => debug!(endpoint = "comments_by_id", id = %id, error = %e),
        }
    }

    if let Some(addr) = user_address {
        match client
            .comments_by_user_address(
                &CommentsByUserAddressRequest::builder()
                    .user_address(addr)
                    .limit(5)
                    .build(),
            )
            .await
        {
            Ok(v) => info!(endpoint = "comments_by_user_address", address = %addr, count = v.len()),
            Err(e) => debug!(endpoint = "comments_by_user_address", address = %addr, error = %e),
        }
    }

    // Use the user_address from comments if available
    if let Some(profile_address) = user_address {
        match client
            .public_profile(
                &PublicProfileRequest::builder()
                    .address(profile_address)
                    .build(),
            )
            .await
        {
            Ok(p) => {
                let name = p.pseudonym.as_deref().unwrap_or("anonymous");
                info!(endpoint = "public_profile", address = %profile_address, name = %name);
            }
            Err(e) => debug!(endpoint = "public_profile", address = %profile_address, error = %e),
        }
    }

    let query = "trump";
    match client
        .search(&SearchRequest::builder().q(query).build())
        .await
    {
        Ok(r) => {
            let events = r.events.map_or(0, |e| e.len());
            let tags = r.tags.map_or(0, |t| t.len());
            let profiles = r.profiles.map_or(0, |p| p.len());
            info!(
                endpoint = "search",
                query = query,
                events = events,
                tags = tags,
                profiles = profiles
            );
        }
        Err(e) => debug!(endpoint = "search", query = query, error = %e),
    }

    Ok(())
}



================================================
FILE: examples/gamma/streaming.rs
================================================
//! Gamma API streaming endpoint explorer.
//!
//! This example demonstrates streaming data from Gamma API endpoints using offset-based
//! pagination and single-call endpoints. It covers all response types:
//!
//! Run with tracing enabled:
//! ```sh
//! RUST_LOG=info cargo run --example gamma_streaming --features gamma,tracing
//! ```
//!
//! Optionally log to a file:
//! ```sh
//! LOG_FILE=gamma_streaming.log RUST_LOG=info cargo run --example gamma_streaming --features gamma,tracing
//! ```

use std::fs::File;

use futures::StreamExt as _;
use polymarket_client_sdk::gamma::{
    Client,
    types::request::{EventsRequest, MarketsRequest},
};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("LOG_FILE") {
        let file = File::create(path)?;
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let client = Client::default();

    stream_events(&client).await?;
    stream_markets(&client).await?;

    Ok(())
}

/// Streams events from the Gamma API.
async fn stream_events(client: &Client) -> anyhow::Result<()> {
    info!(stream = "events", "starting stream");

    let mut stream = client
        .stream_data(
            |c, limit, offset| {
                let request = EventsRequest::builder()
                    .active(true)
                    .limit(limit)
                    .offset(offset)
                    .build();
                async move { c.events(&request).await }
            },
            100,
        )
        .take(100)
        .boxed();

    let mut count = 0_u32;

    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => {
                count += 1;
                info!(stream = "events", count, "{event:?}");
            }
            Err(e) => {
                warn!(stream = "events", error = %e, "stream error");
                break;
            }
        }
    }

    info!(stream = "events", total = count, "stream completed");
    Ok(())
}

/// Streams markets from the Gamma API.
async fn stream_markets(client: &Client) -> anyhow::Result<()> {
    info!(stream = "markets", "starting stream");

    let mut stream = client
        .stream_data(
            |c, limit, offset| {
                let request = MarketsRequest::builder()
                    .closed(false)
                    .limit(limit)
                    .offset(offset)
                    .build();
                async move { c.markets(&request).await }
            },
            100,
        )
        .take(100)
        .boxed();

    let mut count = 0_u32;

    while let Some(result) = stream.next().await {
        match result {
            Ok(market) => {
                count += 1;
                info!(stream = "markets", count, "{market:?}");
            }
            Err(e) => {
                warn!(stream = "markets", error = %e, "stream error");
                break;
            }
        }
    }

    info!(stream = "markets", total = count, "stream completed");
    Ok(())
}



================================================
FILE: src/auth.rs
================================================
// Re-exported types for public API convenience
/// The [`Signer`] trait from alloy for signing operations.
/// Implement this trait or use provided signers like [`LocalSigner`] or AWS KMS signers.
pub use alloy::signers::Signer;
/// Local wallet signer for signing with a private key.
/// This is the most common signer implementation.
pub use alloy::signers::local::LocalSigner;
use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE;
use hmac::{Hmac, Mac as _};
use reqwest::header::HeaderMap;
use reqwest::{Body, Request};
/// Secret string types that redact values in debug output for security.
pub use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sha2::Sha256;
/// UUID type used for API keys and identifiers.
pub use uuid::Uuid;

use crate::{Result, Timestamp};

/// Type alias for API keys, which are UUIDs.
pub type ApiKey = Uuid;

/// Generic set of credentials used to authenticate to the Polymarket API. These credentials are
/// returned when calling [`crate::clob::Client::create_or_derive_api_key`], [`crate::clob::Client::derive_api_key`], or
/// [`crate::clob::Client::create_api_key`]. They are used by the [`state::Authenticated`] client to
/// sign the [`Request`] when making calls to the API.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Credentials {
    #[serde(alias = "apiKey")]
    pub(crate) key: ApiKey,
    pub(crate) secret: SecretString,
    pub(crate) passphrase: SecretString,
}

impl Credentials {
    #[must_use]
    pub fn new(key: Uuid, secret: String, passphrase: String) -> Self {
        Self {
            key,
            secret: SecretString::from(secret),
            passphrase: SecretString::from(passphrase),
        }
    }

    /// Returns the API key.
    #[must_use]
    pub fn key(&self) -> ApiKey {
        self.key
    }

    /// Returns the secret.
    #[must_use]
    pub fn secret(&self) -> &SecretString {
        &self.secret
    }

    /// Returns the passphrase.
    #[must_use]
    pub fn passphrase(&self) -> &SecretString {
        &self.passphrase
    }
}

/// Each client can exist in one state at a time, i.e. [`state::Unauthenticated`] or
/// [`state::Authenticated`].
pub mod state {
    use crate::auth::{Credentials, Kind};
    use crate::types::Address;

    /// The initial state of the client
    #[non_exhaustive]
    #[derive(Clone, Debug)]
    pub struct Unauthenticated;

    /// The elevated state of the client. For example, calling [`crate::clob::Client::authentication_builder`]
    /// will return an [`crate::clob::client::AuthenticationBuilder`], which can be turned into
    /// an authenticated clob via [`crate::clob::client::AuthenticationBuilder::authenticate`].
    ///
    /// See `examples/authenticated.rs` for more context.
    #[non_exhaustive]
    #[derive(Clone, Debug)]
    #[cfg_attr(
        not(feature = "clob"),
        expect(dead_code, reason = "Fields used by clob module when feature enabled")
    )]
    pub struct Authenticated<K: Kind> {
        /// The signer's address that created the credentials
        pub(crate) address: Address,
        /// The [`Credentials`]'s `secret` is used to generate an [`crate::signer::hmac`] which is
        /// passed in the L2 headers ([`super::HeaderMap`]) `POLY_SIGNATURE` field.
        pub(crate) credentials: Credentials,
        /// The [`Kind`] that this [`Authenticated`] exhibits. Used to generate additional headers
        /// for different types of authentication, e.g. Builder.
        pub(crate) kind: K,
    }

    /// The clob state can only be [`Unauthenticated`] or [`Authenticated`].
    pub trait State: sealed::Sealed {}

    impl State for Unauthenticated {}
    impl sealed::Sealed for Unauthenticated {}

    impl<K: Kind> State for Authenticated<K> {}
    impl<K: Kind> sealed::Sealed for Authenticated<K> {}

    mod sealed {
        pub trait Sealed {}
    }
}

/// Asynchronous authentication enricher
///
/// This trait is used to apply extra headers to authenticated requests. For example, in the case
/// of [`builder::Builder`] authentication, Builder headers are added in addition to the [`Normal`]
/// L2 headers.
#[async_trait]
pub trait Kind: sealed::Sealed + Clone + Send + Sync + 'static {
    async fn extra_headers(&self, request: &Request, timestamp: Timestamp) -> Result<HeaderMap>;
}

/// Non-special, generic authentication. Sometimes referred to as L2 authentication.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Normal;

#[async_trait]
impl Kind for Normal {
    async fn extra_headers(&self, _request: &Request, _timestamp: Timestamp) -> Result<HeaderMap> {
        Ok(HeaderMap::new())
    }
}

impl sealed::Sealed for Normal {}

#[async_trait]
impl Kind for builder::Builder {
    async fn extra_headers(&self, request: &Request, timestamp: Timestamp) -> Result<HeaderMap> {
        self.create_headers(request, timestamp).await
    }
}

impl sealed::Sealed for builder::Builder {}

mod sealed {
    pub trait Sealed {}
}

#[cfg(feature = "clob")]
pub(crate) mod l1 {
    use std::borrow::Cow;

    use alloy::core::sol;
    use alloy::dyn_abi::Eip712Domain;
    use alloy::hex::ToHexExt as _;
    use alloy::primitives::{ChainId, U256};
    use alloy::signers::Signer;
    use alloy::sol_types::SolStruct as _;
    use reqwest::header::HeaderMap;

    use crate::{Result, Timestamp};

    pub(crate) const POLY_ADDRESS: &str = "POLY_ADDRESS";
    pub(crate) const POLY_NONCE: &str = "POLY_NONCE";
    pub(crate) const POLY_SIGNATURE: &str = "POLY_SIGNATURE";
    pub(crate) const POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

    sol! {
        #[non_exhaustive]
        struct ClobAuth {
            address address;
            string  timestamp;
            uint256 nonce;
            string  message;
        }
    }

    /// Returns the [`HeaderMap`] needed to obtain [`Credentials`] .
    pub(crate) async fn create_headers<S: Signer>(
        signer: &S,
        chain_id: ChainId,
        timestamp: Timestamp,
        nonce: Option<u32>,
    ) -> Result<HeaderMap> {
        let naive_nonce = nonce.unwrap_or(0);

        let auth = ClobAuth {
            address: signer.address(),
            timestamp: timestamp.to_string(),
            nonce: U256::from(naive_nonce),
            message: "This message attests that I control the given wallet".to_owned(),
        };

        let domain = Eip712Domain {
            name: Some(Cow::Borrowed("ClobAuthDomain")),
            version: Some(Cow::Borrowed("1")),
            chain_id: Some(U256::from(chain_id)),
            ..Eip712Domain::default()
        };

        let hash = auth.eip712_signing_hash(&domain);
        let signature = signer.sign_hash(&hash).await?;

        let mut map = HeaderMap::new();
        map.insert(
            POLY_ADDRESS,
            signer.address().encode_hex_with_prefix().parse()?,
        );
        map.insert(POLY_NONCE, naive_nonce.to_string().parse()?);
        map.insert(POLY_SIGNATURE, signature.to_string().parse()?);
        map.insert(POLY_TIMESTAMP, timestamp.to_string().parse()?);

        Ok(map)
    }
}

#[cfg(feature = "clob")]
pub(crate) mod l2 {
    use alloy::hex::ToHexExt as _;
    use reqwest::Request;
    use reqwest::header::HeaderMap;
    use secrecy::ExposeSecret as _;

    use crate::auth::state::Authenticated;
    use crate::auth::{Kind, hmac, to_message};
    use crate::{Result, Timestamp};

    pub(crate) const POLY_ADDRESS: &str = "POLY_ADDRESS";
    pub(crate) const POLY_API_KEY: &str = "POLY_API_KEY";
    pub(crate) const POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
    pub(crate) const POLY_SIGNATURE: &str = "POLY_SIGNATURE";
    pub(crate) const POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

    /// Returns the [`Headers`] needed to interact with any authenticated endpoints.
    pub(crate) async fn create_headers<K: Kind>(
        state: &Authenticated<K>,
        request: &Request,
        timestamp: Timestamp,
    ) -> Result<HeaderMap> {
        let credentials = &state.credentials;
        let signature = hmac(&credentials.secret, &to_message(request, timestamp))?;

        let mut map = HeaderMap::new();

        map.insert(
            POLY_ADDRESS,
            state.address.encode_hex_with_prefix().parse()?,
        );
        map.insert(POLY_API_KEY, state.credentials.key.to_string().parse()?);
        map.insert(
            POLY_PASSPHRASE,
            state.credentials.passphrase.expose_secret().parse()?,
        );
        map.insert(POLY_SIGNATURE, signature.parse()?);
        map.insert(POLY_TIMESTAMP, timestamp.to_string().parse()?);

        let extra_headers = state.kind.extra_headers(request, timestamp).await?;

        map.extend(extra_headers);

        Ok(map)
    }
}

/// Specific structs and methods used in configuring and authenticating the Builder flow
pub mod builder {
    use reqwest::header::HeaderMap;
    use reqwest::{Client, Request};
    use secrecy::ExposeSecret as _;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    /// URL type for remote builder host configuration.
    pub use url::Url;

    use crate::auth::{Credentials, body_to_string, hmac, to_message};
    use crate::{Result, Timestamp};

    pub(crate) const POLY_BUILDER_API_KEY: &str = "POLY_BUILDER_API_KEY";
    pub(crate) const POLY_BUILDER_PASSPHRASE: &str = "POLY_BUILDER_PASSPHRASE";
    pub(crate) const POLY_BUILDER_SIGNATURE: &str = "POLY_BUILDER_SIGNATURE";
    pub(crate) const POLY_BUILDER_TIMESTAMP: &str = "POLY_BUILDER_TIMESTAMP";

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[expect(
        clippy::struct_field_names,
        reason = "Have to prefix `poly_builder` for serde"
    )]
    struct HeaderPayload {
        poly_builder_api_key: String,
        poly_builder_timestamp: String,
        poly_builder_passphrase: String,
        poly_builder_signature: String,
    }

    /// Configuration used to authenticate as a [Builder](https://docs.polymarket.com/developers/builders/builder-intro). Can either be [`Config::local`]
    /// or [`Config::remote`]. Local uses locally accessible Builder credentials to generate builder headers. Remote obtains them from a signing server
    #[non_exhaustive]
    #[derive(Clone, Debug)]
    pub enum Config {
        Local(Credentials),
        Remote { host: Url, token: Option<String> },
    }

    impl Config {
        #[must_use]
        pub fn local(credentials: Credentials) -> Self {
            Config::Local(credentials)
        }

        pub fn remote(host: &str, token: Option<String>) -> Result<Self> {
            let host = Url::parse(host)?;
            Ok(Config::Remote { host, token })
        }
    }

    /// Used to generate the Builder headers
    #[non_exhaustive]
    #[derive(Clone, Debug)]
    pub struct Builder {
        pub(crate) config: Config,
        pub(crate) client: Client,
    }

    impl Builder {
        pub(crate) async fn create_headers(
            &self,
            request: &Request,
            timestamp: Timestamp,
        ) -> Result<HeaderMap> {
            match &self.config {
                Config::Local(credentials) => {
                    let signature = hmac(&credentials.secret, &to_message(request, timestamp))?;

                    let mut map = HeaderMap::new();

                    map.insert(POLY_BUILDER_API_KEY, credentials.key.to_string().parse()?);
                    map.insert(
                        POLY_BUILDER_PASSPHRASE,
                        credentials.passphrase.expose_secret().parse()?,
                    );
                    map.insert(POLY_BUILDER_SIGNATURE, signature.parse()?);
                    map.insert(POLY_BUILDER_TIMESTAMP, timestamp.to_string().parse()?);

                    Ok(map)
                }
                Config::Remote { host, token } => {
                    let payload = json!({
                        "method": request.method().as_str(),
                        "path": request.url().path(),
                        "body": &request.body().and_then(body_to_string).unwrap_or_default(),
                        "timestamp": timestamp,
                    });

                    let mut headers = HeaderMap::new();
                    if let Some(token) = token {
                        headers.insert("Authorization", format!("Bearer {token}").parse()?);
                    }

                    let response = self
                        .client
                        .post(host.to_string())
                        .headers(headers)
                        .json(&payload)
                        .send()
                        .await?;

                    let remote_headers: HeaderPayload = response.error_for_status()?.json().await?;

                    let mut map = HeaderMap::new();

                    map.insert(
                        POLY_BUILDER_SIGNATURE,
                        remote_headers.poly_builder_signature.parse()?,
                    );
                    map.insert(
                        POLY_BUILDER_TIMESTAMP,
                        remote_headers.poly_builder_timestamp.parse()?,
                    );
                    map.insert(
                        POLY_BUILDER_API_KEY,
                        remote_headers.poly_builder_api_key.parse()?,
                    );
                    map.insert(
                        POLY_BUILDER_PASSPHRASE,
                        remote_headers.poly_builder_passphrase.parse()?,
                    );

                    Ok(map)
                }
            }
        }
    }
}

#[must_use]
fn to_message(request: &Request, timestamp: Timestamp) -> String {
    let method = request.method();
    let body = request.body().and_then(body_to_string).unwrap_or_default();
    let path = request.url().path();

    format!("{timestamp}{method}{path}{body}")
}

#[must_use]
fn body_to_string(body: &Body) -> Option<String> {
    body.as_bytes()
        .map(String::from_utf8_lossy)
        .map(|b| b.replace('\'', "\""))
}

fn hmac(secret: &SecretString, message: &str) -> Result<String> {
    let decoded_secret = URL_SAFE.decode(secret.expose_secret())?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&decoded_secret)?;
    mac.update(message.as_bytes());

    let result = mac.finalize().into_bytes();
    Ok(URL_SAFE.encode(result))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    #[cfg(feature = "clob")]
    use alloy::signers::local::LocalSigner;
    use reqwest::{Client, Method, RequestBuilder};
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::auth::builder::Config;
    #[cfg(feature = "clob")]
    use crate::auth::state::Authenticated;
    #[cfg(feature = "clob")]
    use crate::types::address;
    #[cfg(feature = "clob")]
    use crate::{AMOY, Result};

    // publicly known private key
    #[cfg(feature = "clob")]
    const PRIVATE_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[cfg(feature = "clob")]
    #[tokio::test]
    async fn l1_headers_should_succeed() -> anyhow::Result<()> {
        let signer = LocalSigner::from_str(PRIVATE_KEY)?.with_chain_id(Some(AMOY));

        let headers = l1::create_headers(&signer, AMOY, 10_000_000, Some(23)).await?;

        assert_eq!(
            signer.address(),
            address!("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266")
        );
        assert_eq!(
            headers[l1::POLY_ADDRESS],
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        );
        assert_eq!(headers[l1::POLY_NONCE], "23");
        assert_eq!(
            headers[l1::POLY_SIGNATURE],
            "0xf62319a987514da40e57e2f4d7529f7bac38f0355bd88bb5adbb3768d80de6c1682518e0af677d5260366425f4361e7b70c25ae232aff0ab2331e2b164a1aedc1b"
        );
        assert_eq!(headers[l1::POLY_TIMESTAMP], "10000000");

        Ok(())
    }

    #[cfg(feature = "clob")]
    #[tokio::test]
    async fn l2_headers_should_succeed() -> anyhow::Result<()> {
        let signer = LocalSigner::from_str(PRIVATE_KEY)?;

        let authenticated = Authenticated {
            address: signer.address(),
            credentials: Credentials {
                key: Uuid::nil(),
                passphrase: SecretString::from(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
                ),
                secret: SecretString::from(
                    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_owned(),
                ),
            },
            kind: Normal,
        };

        let request = Request::new(Method::GET, Url::parse("http://localhost/")?);
        let headers = l2::create_headers(&authenticated, &request, 1).await?;

        assert_eq!(
            headers[l2::POLY_ADDRESS],
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        );
        assert_eq!(
            headers[l2::POLY_PASSPHRASE],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(headers[l2::POLY_API_KEY], Uuid::nil().to_string());
        assert_eq!(
            headers[l2::POLY_SIGNATURE],
            "eHaylCwqRSOa2LFD77Nt_SaTpbsxzN8eTEI3LryhEj4="
        );
        assert_eq!(headers[l2::POLY_TIMESTAMP], "1");

        Ok(())
    }

    #[tokio::test]
    async fn builder_headers_should_succeed() -> Result<()> {
        let credentials = Credentials {
            key: Uuid::nil(),
            passphrase: SecretString::from(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            ),
            secret: SecretString::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_owned()),
        };
        let config = Config::local(credentials);
        let request = Request::new(Method::GET, Url::parse("http://localhost/")?);
        let timestamp = 1;

        let builder = builder::Builder {
            config,
            client: Client::default(),
        };

        let headers = builder.create_headers(&request, timestamp).await?;

        assert_eq!(
            headers[builder::POLY_BUILDER_API_KEY],
            Uuid::nil().to_string()
        );
        assert_eq!(
            headers[builder::POLY_BUILDER_PASSPHRASE],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(headers[builder::POLY_BUILDER_TIMESTAMP], "1");

        Ok(())
    }

    #[test]
    fn request_args_should_succeed() -> Result<()> {
        let request = Request::new(Method::POST, Url::parse("http://localhost/path")?);
        let request = RequestBuilder::from_parts(Client::new(), request)
            .json(&json!({"foo": "bar"}))
            .build()?;

        let timestamp = 1;

        assert_eq!(
            to_message(&request, timestamp),
            r#"1POST/path{"foo":"bar"}"#
        );

        Ok(())
    }

    #[test]
    fn hmac_succeeds() -> Result<()> {
        let json = json!({
            "hash": "0x123"
        });

        let method = Method::from_str("test-sign")
            .expect("To avoid needing an error variant just for one test");
        let request = Request::new(method, Url::parse("http://localhost/orders")?);
        let request = RequestBuilder::from_parts(Client::new(), request)
            .json(&json)
            .build()?;

        let message = to_message(&request, 1_000_000);
        let signature = hmac(
            &SecretString::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_owned()),
            &message,
        )?;

        assert_eq!(message, r#"1000000test-sign/orders{"hash":"0x123"}"#);
        assert_eq!(signature, "4gJVbox-R6XlDK4nlaicig0_ANVL1qdcahiL8CXfXLM=");

        Ok(())
    }

    #[test]
    fn credentials_key_returns_api_key() {
        let key = Uuid::new_v4();
        let credentials = Credentials::new(key, "secret".to_owned(), "passphrase".to_owned());
        assert_eq!(credentials.key(), key);
    }

    #[test]
    fn debug_does_not_expose_secrets() {
        let secret_value = "my_super_secret_value_12345";
        let passphrase_value = "my_super_secret_passphrase_67890";
        let credentials = Credentials::new(
            Uuid::nil(),
            secret_value.to_owned(),
            passphrase_value.to_owned(),
        );

        let debug_output = format!("{credentials:?}");

        // Verify that the secret values are NOT present in the debug output
        assert!(
            !debug_output.contains(secret_value),
            "Debug output should NOT contain the secret value. Got: {debug_output}"
        );
        assert!(
            !debug_output.contains(passphrase_value),
            "Debug output should NOT contain the passphrase value. Got: {debug_output}"
        );
    }
}



================================================
FILE: src/error.rs
================================================
use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt;

use alloy::primitives::ChainId;
use alloy::primitives::ruint::ParseError;
use hmac::digest::InvalidLength;
/// HTTP method type, re-exported for use with error inspection.
pub use reqwest::Method;
/// HTTP status code type, re-exported for use with error inspection.
pub use reqwest::StatusCode;
use reqwest::header;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Error related to non-successful HTTP call
    Status,
    /// Error related to invalid state within polymarket-client-sdk
    Validation,
    /// Error related to synchronization of authenticated clients logging in and out
    Synchronization,
    /// Internal error from dependencies
    Internal,
    /// Error related to WebSocket connections
    WebSocket,
    /// Error related to geographic restrictions blocking access
    Geoblock,
}

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    backtrace: Backtrace,
}

impl Error {
    pub fn with_source<S: StdError + Send + Sync + 'static>(kind: Kind, source: S) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    pub fn inner(&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    pub fn downcast_ref<E: StdError + 'static>(&self) -> Option<&E> {
        let e = self.source.as_deref()?;
        e.downcast_ref::<E>()
    }

    pub fn validation<S: Into<String>>(message: S) -> Self {
        Validation {
            reason: message.into(),
        }
        .into()
    }

    pub fn status<S: Into<String>>(
        status_code: StatusCode,
        method: Method,
        path: String,
        message: S,
    ) -> Self {
        Status {
            status_code,
            method,
            path,
            message: message.into(),
        }
        .into()
    }

    #[must_use]
    pub fn missing_contract_config(chain_id: ChainId, neg_risk: bool) -> Self {
        MissingContractConfig { chain_id, neg_risk }.into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(src) => write!(f, "{:?}: {}", self.kind, src),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|e| e as &(dyn StdError + 'static))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Status {
    pub status_code: StatusCode,
    pub method: Method,
    pub path: String,
    pub message: String,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error({}) making {} call to {} with {}",
            self.status_code, self.method, self.path, self.message
        )
    }
}

impl StdError for Status {}

#[non_exhaustive]
#[derive(Debug)]
pub struct Validation {
    pub reason: String,
}

impl fmt::Display for Validation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid: {}", self.reason)
    }
}

impl StdError for Validation {}

#[non_exhaustive]
#[derive(Debug)]
pub struct Synchronization;

impl fmt::Display for Synchronization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "synchronization error: multiple threads are attempting to log in or log out"
        )
    }
}

impl StdError for Synchronization {}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct MissingContractConfig {
    pub chain_id: ChainId,
    pub neg_risk: bool,
}

impl fmt::Display for MissingContractConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "missing contract config for chain id {} with neg_risk = {}",
            self.chain_id, self.neg_risk,
        )
    }
}

impl std::error::Error for MissingContractConfig {}

impl From<MissingContractConfig> for Error {
    fn from(err: MissingContractConfig) -> Self {
        Error::with_source(Kind::Internal, err)
    }
}

/// Error indicating that the user is blocked from accessing Polymarket due to geographic
/// restrictions.
///
/// This error contains information about the user's detected location.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Geoblock {
    /// The detected IP address
    pub ip: String,
    /// ISO 3166-1 alpha-2 country code
    pub country: String,
    /// Region/state code
    pub region: String,
}

impl fmt::Display for Geoblock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "access blocked from country: {}, region: {}, ip: {}",
            self.country, self.region, self.ip
        )
    }
}

impl StdError for Geoblock {}

impl From<Geoblock> for Error {
    fn from(err: Geoblock) -> Self {
        Error::with_source(Kind::Geoblock, err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<header::InvalidHeaderValue> for Error {
    fn from(e: header::InvalidHeaderValue) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<InvalidLength> for Error {
    fn from(e: InvalidLength) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<alloy::signers::Error> for Error {
    fn from(e: alloy::signers::Error) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::with_source(Kind::Internal, e)
    }
}

impl From<Validation> for Error {
    fn from(err: Validation) -> Self {
        Error::with_source(Kind::Validation, err)
    }
}

impl From<Status> for Error {
    fn from(err: Status) -> Self {
        Error::with_source(Kind::Status, err)
    }
}

impl From<Synchronization> for Error {
    fn from(err: Synchronization) -> Self {
        Error::with_source(Kind::Synchronization, err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geoblock_display_should_succeed() {
        let geoblock = Geoblock {
            ip: "192.168.1.1".to_owned(),
            country: "US".to_owned(),
            region: "NY".to_owned(),
        };

        assert_eq!(
            geoblock.to_string(),
            "access blocked from country: US, region: NY, ip: 192.168.1.1"
        );
    }

    #[test]
    fn geoblock_into_error_should_succeed() {
        let geoblock = Geoblock {
            ip: "10.0.0.1".to_owned(),
            country: "CU".to_owned(),
            region: "HAV".to_owned(),
        };

        let error: Error = geoblock.into();

        assert_eq!(error.kind(), Kind::Geoblock);
        assert!(error.to_string().contains("CU"));
    }
}



================================================
FILE: src/lib.rs
================================================
#![cfg_attr(doc, doc = include_str!("../README.md"))]

pub mod auth;
#[cfg(feature = "bridge")]
pub mod bridge;
#[cfg(feature = "clob")]
pub mod clob;
#[cfg(feature = "ctf")]
pub mod ctf;
#[cfg(feature = "data")]
pub mod data;
pub mod error;
#[cfg(feature = "gamma")]
pub mod gamma;
#[cfg(feature = "rtds")]
pub mod rtds;
pub(crate) mod serde_helpers;
pub mod types;
#[cfg(any(feature = "ws", feature = "rtds"))]
pub mod ws;

use std::fmt::Write as _;

use alloy::primitives::ChainId;
use alloy::primitives::{B256, b256, keccak256};
use phf::phf_map;
#[cfg(any(
    feature = "bridge",
    feature = "clob",
    feature = "data",
    feature = "gamma"
))]
use reqwest::{Request, StatusCode, header::HeaderMap};
use serde::Serialize;
#[cfg(any(
    feature = "bridge",
    feature = "clob",
    feature = "data",
    feature = "gamma"
))]
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::types::{Address, address};

pub type Result<T> = std::result::Result<T, Error>;

/// [`ChainId`] for Polygon mainnet
pub const POLYGON: ChainId = 137;

/// [`ChainId`] for Polygon testnet <https://polygon.technology/blog/introducing-the-amoy-testnet-for-polygon-pos>
pub const AMOY: ChainId = 80002;

pub const PRIVATE_KEY_VAR: &str = "POLYMARKET_PRIVATE_KEY";

/// Timestamp in seconds since [`std::time::UNIX_EPOCH`]
pub(crate) type Timestamp = i64;

static CONFIG: phf::Map<ChainId, ContractConfig> = phf_map! {
    137_u64 => ContractConfig {
        exchange: address!("0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"),
        collateral: address!("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"),
        conditional_tokens: address!("0x4D97DCd97eC945f40cF65F87097ACe5EA0476045"),
        neg_risk_adapter: None,
    },
    80002_u64 => ContractConfig {
        exchange: address!("0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40"),
        collateral: address!("0x9c4e1703476e875070ee25b56a58b008cfb8fa78"),
        conditional_tokens: address!("0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB"),
        neg_risk_adapter: None,
    },
};

static NEG_RISK_CONFIG: phf::Map<ChainId, ContractConfig> = phf_map! {
    137_u64 => ContractConfig {
        exchange: address!("0xC5d563A36AE78145C45a50134d48A1215220f80a"),
        collateral: address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174"),
        conditional_tokens: address!("0x4D97DCd97eC945f40cF65F87097ACe5EA0476045"),
        neg_risk_adapter: Some(address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296")),
    },
    80002_u64 => ContractConfig {
        exchange: address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296"),
        collateral: address!("0x9c4e1703476e875070ee25b56a58b008cfb8fa78"),
        conditional_tokens: address!("0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB"),
        neg_risk_adapter: Some(address!("0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296")),
    },
};

// Wallet contract configurations for CREATE2 address derivation
// Source: https://github.com/Polymarket/builder-relayer-client
static WALLET_CONFIG: phf::Map<ChainId, WalletContractConfig> = phf_map! {
    137_u64 => WalletContractConfig {
        proxy_factory: Some(address!("0xaB45c5A4B0c941a2F231C04C3f49182e1A254052")),
        safe_factory: address!("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"),
    },
    80002_u64 => WalletContractConfig {
        // Proxy factory unsupported on Amoy testnet
        proxy_factory: None,
        safe_factory: address!("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"),
    },
};

/// Init code hash for Polymarket Proxy wallets (EIP-1167 minimal proxy)
const PROXY_INIT_CODE_HASH: B256 =
    b256!("0xd21df8dc65880a8606f09fe0ce3df9b8869287ab0b058be05aa9e8af6330a00b");

/// Init code hash for Gnosis Safe wallets
const SAFE_INIT_CODE_HASH: B256 =
    b256!("0x2bce2127ff07fb632d16c8347c4ebf501f4841168bed00d9e6ef715ddb6fcecf");

/// Helper struct to group the relevant deployed contract addresses
#[non_exhaustive]
#[derive(Debug)]
pub struct ContractConfig {
    pub exchange: Address,
    pub collateral: Address,
    pub conditional_tokens: Address,
    /// The Neg Risk Adapter contract address. Only present for neg-risk market configs.
    /// Users must approve this contract for token transfers to trade in neg-risk markets.
    pub neg_risk_adapter: Option<Address>,
}

/// Wallet contract configuration for CREATE2 address derivation
#[non_exhaustive]
#[derive(Debug)]
pub struct WalletContractConfig {
    /// Factory contract for Polymarket Proxy wallets (Magic/email wallets).
    /// Not available on all networks (e.g., Amoy testnet).
    pub proxy_factory: Option<Address>,
    /// Factory contract for Gnosis Safe wallets.
    pub safe_factory: Address,
}

/// Given a `chain_id` and `is_neg_risk`, return the relevant [`ContractConfig`]
#[must_use]
pub fn contract_config(chain_id: ChainId, is_neg_risk: bool) -> Option<&'static ContractConfig> {
    if is_neg_risk {
        NEG_RISK_CONFIG.get(&chain_id)
    } else {
        CONFIG.get(&chain_id)
    }
}

/// Returns the wallet contract configuration for the given chain ID.
#[must_use]
pub fn wallet_contract_config(chain_id: ChainId) -> Option<&'static WalletContractConfig> {
    WALLET_CONFIG.get(&chain_id)
}

/// Derives the Polymarket Proxy wallet address for an EOA using CREATE2.
///
/// This is the deterministic address of the EIP-1167 minimal proxy wallet
/// that Polymarket deploys for Magic/email wallet users.
///
/// # Arguments
/// * `eoa_address` - The externally owned account (EOA) address
/// * `chain_id` - The chain ID (e.g., 137 for Polygon mainnet)
///
/// # Returns
/// * `Some(Address)` - The derived proxy wallet address
/// * `None` - If the chain doesn't support proxy wallets or config is missing
#[must_use]
pub fn derive_proxy_wallet(eoa_address: Address, chain_id: ChainId) -> Option<Address> {
    let config = wallet_contract_config(chain_id)?;
    let factory = config.proxy_factory?;

    // Salt is keccak256(encodePacked(address)) - address is 20 bytes, no padding
    let salt = keccak256(eoa_address);

    Some(factory.create2(salt, PROXY_INIT_CODE_HASH))
}

/// Derives the Gnosis Safe wallet address for an EOA using CREATE2.
///
/// This is the deterministic address of the 1-of-1 Gnosis Safe multisig
/// that Polymarket deploys for browser wallet users.
///
/// # Arguments
/// * `eoa_address` - The externally owned account (EOA) address
/// * `chain_id` - The chain ID (e.g., 137 for Polygon mainnet)
///
/// # Returns
/// * `Some(Address)` - The derived Safe wallet address
/// * `None` - If the chain config is missing
#[must_use]
pub fn derive_safe_wallet(eoa_address: Address, chain_id: ChainId) -> Option<Address> {
    let config = wallet_contract_config(chain_id)?;
    let factory = config.safe_factory;

    // Salt is keccak256(encodeAbiParameters(address)) - address padded to 32 bytes
    // ABI encoding pads address to 32 bytes (left-padded with zeros)
    let mut padded = [0_u8; 32];
    padded[12..].copy_from_slice(eoa_address.as_slice());
    let salt = keccak256(padded);

    Some(factory.create2(salt, SAFE_INIT_CODE_HASH))
}

/// Trait for converting request types to URL query parameters.
///
/// This trait is automatically implemented for all types that implement [`Serialize`].
/// It uses [`serde_html_form`] to serialize the struct fields into a query string.
/// Arrays are serialized as repeated keys (`key=val1&key=val2`).
pub trait ToQueryParams: Serialize {
    /// Converts the request to a URL query string.
    ///
    /// Returns an empty string if no parameters are set, otherwise returns
    /// a string starting with `?` followed by URL-encoded key-value pairs.
    /// Also uses an optional cursor as a parameter, if provided.
    fn query_params(&self, next_cursor: Option<&str>) -> String {
        let mut params = serde_html_form::to_string(self)
            .inspect_err(|e| {
                #[cfg(feature = "tracing")]
                tracing::error!("Unable to convert to URL-encoded string {e:?}");
                #[cfg(not(feature = "tracing"))]
                let _: &serde_html_form::ser::Error = e;
            })
            .unwrap_or_default();

        if let Some(cursor) = next_cursor {
            if !params.is_empty() {
                params.push('&');
            }
            let _ = write!(params, "next_cursor={cursor}");
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{params}")
        }
    }
}

impl<T: Serialize> ToQueryParams for T {}

#[cfg(any(
    feature = "bridge",
    feature = "clob",
    feature = "data",
    feature = "gamma"
))]
#[cfg_attr(
    feature = "tracing",
    tracing::instrument(
        level = "debug",
        skip(client, request, headers),
        fields(
            method = %request.method(),
            path = request.url().path(),
            status_code
        )
    )
)]
async fn request<Response: DeserializeOwned>(
    client: &reqwest::Client,
    mut request: Request,
    headers: Option<HeaderMap>,
) -> Result<Response> {
    let method = request.method().clone();
    let path = request.url().path().to_owned();

    if let Some(h) = headers {
        *request.headers_mut() = h;
    }

    let response = client.execute(request).await?;
    let status_code = response.status();

    #[cfg(feature = "tracing")]
    tracing::Span::current().record("status_code", status_code.as_u16());

    if !status_code.is_success() {
        let message = response.text().await.unwrap_or_default();

        #[cfg(feature = "tracing")]
        tracing::warn!(
            status = %status_code,
            method = %method,
            path = %path,
            message = %message,
            "API request failed"
        );

        return Err(Error::status(status_code, method, path, message));
    }

    let json_value = response.json::<serde_json::Value>().await?;
    let response_data: Option<Response> = serde_helpers::deserialize_with_warnings(json_value)?;

    if let Some(response) = response_data {
        Ok(response)
    } else {
        #[cfg(feature = "tracing")]
        tracing::warn!(method = %method, path = %path, "API resource not found");
        Err(Error::status(
            StatusCode::NOT_FOUND,
            method,
            path,
            "Unable to find requested resource",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_contains_80002() {
        let cfg = contract_config(AMOY, false).expect("missing config");
        assert_eq!(
            cfg.exchange,
            address!("0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40")
        );
    }

    #[test]
    fn config_contains_80002_neg() {
        let cfg = contract_config(AMOY, true).expect("missing config");
        assert_eq!(
            cfg.exchange,
            address!("0xd91e80cf2e7be2e162c6513ced06f1dd0da35296")
        );
    }

    #[test]
    fn wallet_contract_config_polygon() {
        let cfg = wallet_contract_config(POLYGON).expect("missing config");
        assert_eq!(
            cfg.proxy_factory,
            Some(address!("0xaB45c5A4B0c941a2F231C04C3f49182e1A254052"))
        );
        assert_eq!(
            cfg.safe_factory,
            address!("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b")
        );
    }

    #[test]
    fn wallet_contract_config_amoy() {
        let cfg = wallet_contract_config(AMOY).expect("missing config");
        // Proxy factory not supported on Amoy
        assert_eq!(cfg.proxy_factory, None);
        assert_eq!(
            cfg.safe_factory,
            address!("0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b")
        );
    }

    #[test]
    fn derive_safe_wallet_polygon() {
        // Test address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (Foundry/Anvil test key)
        let eoa = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        let safe_addr = derive_safe_wallet(eoa, POLYGON).expect("derivation failed");

        // This is the deterministic Safe address for this EOA on Polygon
        assert_eq!(
            safe_addr,
            address!("0xd93b25Cb943D14d0d34FBAf01fc93a0F8b5f6e47")
        );
    }

    #[test]
    fn derive_proxy_wallet_polygon() {
        // Test address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (Foundry/Anvil test key)
        let eoa = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        let proxy_addr = derive_proxy_wallet(eoa, POLYGON).expect("derivation failed");

        // This is the deterministic Proxy address for this EOA on Polygon
        assert_eq!(
            proxy_addr,
            address!("0x365f0cA36ae1F641E02Fe3b7743673DA42A13a70")
        );
    }

    #[test]
    fn derive_proxy_wallet_amoy_not_supported() {
        let eoa = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        // Proxy wallet derivation should fail on Amoy (no proxy factory)
        assert!(derive_proxy_wallet(eoa, AMOY).is_none());
    }

    #[test]
    fn derive_safe_wallet_amoy() {
        let eoa = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        // Safe wallet derivation should work on Amoy
        let safe_addr = derive_safe_wallet(eoa, AMOY).expect("derivation failed");

        // Same Safe factory on both networks, so same derived address
        assert_eq!(
            safe_addr,
            address!("0xd93b25Cb943D14d0d34FBAf01fc93a0F8b5f6e47")
        );
    }

    #[test]
    fn derive_wallet_unsupported_chain() {
        let eoa = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        // Unsupported chain should return None
        assert!(derive_proxy_wallet(eoa, 1).is_none());
        assert!(derive_safe_wallet(eoa, 1).is_none());
    }
}



================================================
FILE: src/serde_helpers.rs
================================================
//! Serde helpers for flexible deserialization.
//!
//! When the `tracing` feature is enabled, this module also logs warnings for any
//! unknown fields encountered during deserialization, helping detect API changes.

#[cfg(any(
    feature = "bridge",
    feature = "clob",
    feature = "data",
    feature = "gamma",
))]
use {serde::de::DeserializeOwned, serde_json::Value};

/// A `serde_as` type that deserializes strings or integers as `String`.
///
/// Use with `#[serde_as(as = "StringFromAny")]` for `String` fields
/// or `#[serde_as(as = "Option<StringFromAny>")]` for `Option<String>`.
#[cfg(any(feature = "clob", feature = "gamma"))]
pub struct StringFromAny;

#[cfg(any(feature = "clob", feature = "gamma"))]
impl<'de> serde_with::DeserializeAs<'de, String> for StringFromAny {
    fn deserialize_as<D>(deserializer: D) -> std::result::Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct StringOrNumberVisitor;

        impl Visitor<'_> for StringOrNumberVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or integer")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.to_owned())
            }

            fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.to_string())
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.to_string())
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

#[cfg(any(feature = "clob", feature = "gamma"))]
impl serde_with::SerializeAs<String> for StringFromAny {
    fn serialize_as<S>(source: &String, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(source)
    }
}

/// Deserialize JSON with unknown field warnings.
///
/// This function deserializes JSON to a target type while detecting and logging
/// any fields that are not captured by the type definition.
///
/// # Arguments
///
/// * `value` - The JSON value to deserialize
///
/// # Returns
///
/// The deserialized value, or an error if deserialization fails.
/// Unknown fields trigger warnings but do not cause deserialization to fail.
///
/// # Example
///
/// ```ignore
/// let json = serde_json::json!({
///     "known_field": "value",
///     "unknown_field": "extra"
/// });
/// let result: MyType = deserialize_with_warnings(json)?;
/// // Logs: WARN Unknown field "unknown_field" with value "extra" in MyType
/// ```
#[cfg(all(
    feature = "tracing",
    any(
        feature = "bridge",
        feature = "clob",
        feature = "data",
        feature = "gamma"
    )
))]
pub fn deserialize_with_warnings<T: DeserializeOwned>(value: Value) -> crate::Result<T> {
    use std::any::type_name;

    tracing::trace!(
        type_name = %type_name::<T>(),
        json = %value,
        "deserializing JSON"
    );

    // Clone the value so we can look up unknown field values later
    let original = value.clone();

    // Collect unknown field paths during deserialization
    let mut unknown_paths: Vec<String> = Vec::new();

    let result: T = serde_ignored::deserialize(value, |path| {
        unknown_paths.push(path.to_string());
    })
    .inspect_err(|_| {
        // Re-deserialize with serde_path_to_error to get the error path
        let json_str = original.to_string();
        let jd = &mut serde_json::Deserializer::from_str(&json_str);
        let path_result: Result<T, _> = serde_path_to_error::deserialize(jd);
        if let Err(path_err) = path_result {
            let path = path_err.path().to_string();
            let inner_error = path_err.inner();
            let value_at_path = lookup_value(&original, &path);
            let value_display = format_value(value_at_path);

            tracing::error!(
                type_name = %type_name::<T>(),
                path = %path,
                value = %value_display,
                error = %inner_error,
                "deserialization failed"
            );
        }
    })?;

    // Log warnings for unknown fields with their values
    if !unknown_paths.is_empty() {
        let type_name = type_name::<T>();
        for path in unknown_paths {
            let field_value = lookup_value(&original, &path);
            let value_display = format_value(field_value);

            tracing::warn!(
                type_name = %type_name,
                field = %path,
                value = %value_display,
                "unknown field in API response"
            );
        }
    }

    Ok(result)
}

/// Pass-through deserialization when tracing is disabled.
#[cfg(all(
    not(feature = "tracing"),
    any(
        feature = "bridge",
        feature = "clob",
        feature = "data",
        feature = "gamma"
    )
))]
pub fn deserialize_with_warnings<T: DeserializeOwned>(value: Value) -> crate::Result<T> {
    Ok(serde_json::from_value(value)?)
}

/// Look up a value in a JSON structure by path.
///
/// Handles paths from both `serde_ignored` and `serde_path_to_error`:
/// - `?` for Option wrappers (skipped, as JSON has no Option representation)
/// - Numeric indices for arrays: `items.0` or `items[0]`
/// - Field names for objects: `foo.bar` or `foo.bar[0].baz`
///
/// Returns `None` if the path doesn't exist or traverses a non-container value.
#[cfg(feature = "tracing")]
fn lookup_value<'value>(value: &'value Value, path: &str) -> Option<&'value Value> {
    if path.is_empty() {
        return Some(value);
    }

    let mut current = value;

    // Parse path segments, handling both dot notation and bracket notation
    // e.g., "data[15].condition_id" -> ["data", "15", "condition_id"]
    let segments = parse_path_segments(path);

    for segment in segments {
        if segment.is_empty() || segment == "?" {
            continue;
        }

        match current {
            Value::Object(map) => {
                current = map.get(&segment)?;
            }
            Value::Array(arr) => {
                let index: usize = segment.parse().ok()?;
                current = arr.get(index)?;
            }
            _ => return None,
        }
    }

    Some(current)
}

/// Parse a path string into segments, handling both dot and bracket notation.
///
/// Examples:
/// - `"foo.bar"` -> `["foo", "bar"]`
/// - `"data[15].condition_id"` -> `["data", "15", "condition_id"]`
/// - `"items[0][1].value"` -> `["items", "0", "1", "value"]`
#[cfg(feature = "tracing")]
fn parse_path_segments(path: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();

    let mut chars = path.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
            }
            '[' => {
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
                // Collect until closing bracket
                for inner in chars.by_ref() {
                    if inner == ']' {
                        break;
                    }
                    current.push(inner);
                }
                if !current.is_empty() {
                    segments.push(std::mem::take(&mut current));
                }
            }
            ']' => {
                // Shouldn't happen if well-formed, but handle gracefully
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        segments.push(current);
    }

    segments
}

/// Format a JSON value for logging.
#[cfg(feature = "tracing")]
fn format_value(value: Option<&Value>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "<unable to retrieve>".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    // Imports for tracing-gated tests in the outer module
    #[cfg(feature = "tracing")]
    use serde_json::Value;

    #[cfg(feature = "tracing")]
    use super::{format_value, lookup_value};

    // ========== deserialize_with_warnings tests ==========
    #[cfg(any(
        feature = "bridge",
        feature = "clob",
        feature = "data",
        feature = "gamma"
    ))]
    mod deserialize_with_warnings_tests {
        use serde::Deserialize;

        use super::super::deserialize_with_warnings;

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            known_field: String,
            #[serde(default)]
            optional_field: Option<i32>,
        }

        #[test]
        fn deserialize_known_fields_only() {
            let json = serde_json::json!({
                "known_field": "value",
                "optional_field": 42
            });

            let result: TestStruct =
                deserialize_with_warnings(json).expect("deserialization failed");
            assert_eq!(result.known_field, "value");
            assert_eq!(result.optional_field, Some(42));
        }

        #[test]
        fn deserialize_with_unknown_fields() {
            let json = serde_json::json!({
                "known_field": "value",
                "unknown_field": "extra",
                "another_unknown": 123
            });

            // Should succeed - extra fields are logged but not an error
            let result: TestStruct =
                deserialize_with_warnings(json).expect("deserialization failed");
            assert_eq!(result.known_field, "value");
            assert_eq!(result.optional_field, None);
        }

        #[test]
        fn deserialize_missing_required_field_fails() {
            let json = serde_json::json!({
                "optional_field": 42
            });

            let result: crate::Result<TestStruct> = deserialize_with_warnings(json);
            result.unwrap_err();
        }

        #[test]
        fn deserialize_array() {
            let json = serde_json::json!([1, 2, 3]);

            let result: Vec<i32> = deserialize_with_warnings(json).expect("deserialization failed");
            assert_eq!(result, vec![1, 2, 3]);
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct NestedStruct {
            outer: String,
            inner: InnerStruct,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct InnerStruct {
            value: i32,
        }

        #[test]
        fn deserialize_nested_unknown_fields() {
            let json = serde_json::json!({
                "outer": "test",
                "inner": {
                    "value": 42,
                    "nested_unknown": "surprise"
                }
            });

            let result: NestedStruct =
                deserialize_with_warnings(json).expect("deserialization failed");
            assert_eq!(result.outer, "test");
            assert_eq!(result.inner.value, 42);
        }

        /// Test that verifies warnings are actually emitted for unknown fields.
        /// This test captures tracing output to prove the feature works.
        #[cfg(feature = "tracing")]
        #[test]
        fn warning_is_emitted_for_unknown_fields() {
            use std::sync::{Arc, Mutex};

            use tracing_subscriber::layer::SubscriberExt as _;

            // Capture warnings in a buffer
            let warnings: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
            let warnings_clone = Arc::clone(&warnings);

            // Custom layer that captures warn events
            let layer = tracing_subscriber::fmt::layer()
                .with_writer(move || {
                    struct CaptureWriter(Arc<Mutex<Vec<String>>>);
                    impl std::io::Write for CaptureWriter {
                        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                            if let Ok(s) = std::str::from_utf8(buf) {
                                self.0.lock().expect("lock").push(s.to_owned());
                            }
                            Ok(buf.len())
                        }
                        fn flush(&mut self) -> std::io::Result<()> {
                            Ok(())
                        }
                    }
                    CaptureWriter(Arc::clone(&warnings_clone))
                })
                .with_ansi(false);

            let subscriber = tracing_subscriber::registry().with(layer);

            // Run the deserialization with our subscriber
            tracing::subscriber::with_default(subscriber, || {
                let json = serde_json::json!({
                    "known_field": "value",
                    "secret_new_field": "surprise!",
                    "another_unknown": 42
                });

                let result: TestStruct =
                    deserialize_with_warnings(json).expect("deserialization should succeed");
                assert_eq!(result.known_field, "value");
            });

            // Check that warnings were captured
            let captured = warnings.lock().expect("lock");
            let all_output = captured.join("");

            assert!(
                all_output.contains("unknown field"),
                "Expected 'unknown field' in output, got: {all_output}"
            );
            assert!(
                all_output.contains("secret_new_field"),
                "Expected 'secret_new_field' in output, got: {all_output}"
            );
        }
    }

    // ========== StringFromAny tests ==========
    #[cfg(any(feature = "clob", feature = "gamma"))]
    mod string_from_any_tests {
        use serde::Deserialize;

        use super::super::StringFromAny;

        #[derive(Debug, Deserialize, PartialEq, serde::Serialize)]
        struct StringFromAnyStruct {
            #[serde(with = "serde_with::As::<StringFromAny>")]
            id: String,
        }

        #[derive(Debug, Deserialize, PartialEq, serde::Serialize)]
        struct OptionalStringFromAny {
            #[serde(with = "serde_with::As::<Option<StringFromAny>>")]
            id: Option<String>,
        }

        #[test]
        fn string_from_any_deserialize_string() {
            let json = serde_json::json!({ "id": "hello" });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, "hello");
        }

        #[test]
        fn string_from_any_deserialize_positive_integer() {
            let json = serde_json::json!({ "id": 12345 });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, "12345");
        }

        #[test]
        fn string_from_any_deserialize_negative_integer() {
            let json = serde_json::json!({ "id": -42 });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, "-42");
        }

        #[test]
        fn string_from_any_deserialize_zero() {
            let json = serde_json::json!({ "id": 0 });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, "0");
        }

        #[test]
        fn string_from_any_deserialize_large_u64() {
            // Test u64 max value
            let json = serde_json::json!({ "id": u64::MAX });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, u64::MAX.to_string());
        }

        #[test]
        fn string_from_any_deserialize_large_negative_i64() {
            // Test i64 min value
            let json = serde_json::json!({ "id": i64::MIN });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, i64::MIN.to_string());
        }

        #[test]
        fn string_from_any_serialize_back_to_string() {
            let obj = StringFromAnyStruct {
                id: "12345".to_owned(),
            };
            let json = serde_json::to_value(&obj).expect("serialization failed");
            assert_eq!(json, serde_json::json!({ "id": "12345" }));
        }

        #[test]
        fn string_from_any_roundtrip_from_string() {
            let json = serde_json::json!({ "id": "hello" });
            let obj: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            let back = serde_json::to_value(&obj).expect("serialization failed");
            assert_eq!(back, serde_json::json!({ "id": "hello" }));
        }

        #[test]
        fn string_from_any_roundtrip_from_integer() {
            let json = serde_json::json!({ "id": 42 });
            let obj: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            // After roundtrip, integer becomes string
            let back = serde_json::to_value(&obj).expect("serialization failed");
            assert_eq!(back, serde_json::json!({ "id": "42" }));
        }

        #[test]
        fn string_from_any_option_some_string() {
            let json = serde_json::json!({ "id": "hello" });
            let result: OptionalStringFromAny =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, Some("hello".to_owned()));
        }

        #[test]
        fn string_from_any_option_some_integer() {
            let json = serde_json::json!({ "id": 123 });
            let result: OptionalStringFromAny =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, Some("123".to_owned()));
        }

        #[test]
        fn string_from_any_option_none() {
            let json = serde_json::json!({ "id": null });
            let result: OptionalStringFromAny =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, None);
        }

        #[test]
        fn string_from_any_option_serialize_some() {
            let obj = OptionalStringFromAny {
                id: Some("test".to_owned()),
            };
            let json = serde_json::to_value(&obj).expect("serialization failed");
            assert_eq!(json, serde_json::json!({ "id": "test" }));
        }

        #[test]
        fn string_from_any_option_serialize_none() {
            let obj = OptionalStringFromAny { id: None };
            let json = serde_json::to_value(&obj).expect("serialization failed");
            assert_eq!(json, serde_json::json!({ "id": null }));
        }

        #[test]
        fn string_from_any_empty_string() {
            let json = serde_json::json!({ "id": "" });
            let result: StringFromAnyStruct =
                serde_json::from_value(json).expect("deserialization failed");
            assert_eq!(result.id, "");
        }
    }

    // ========== lookup_value tests ==========

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_simple_path() {
        let json = serde_json::json!({
            "foo": "bar"
        });

        let result = lookup_value(&json, "foo");
        assert_eq!(result, Some(&Value::String("bar".to_owned())));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_nested_path() {
        let json = serde_json::json!({
            "outer": {
                "inner": "value"
            }
        });

        let result = lookup_value(&json, "outer.inner");
        assert_eq!(result, Some(&Value::String("value".to_owned())));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_array_index() {
        let json = serde_json::json!({
            "items": ["a", "b", "c"]
        });

        let result = lookup_value(&json, "items.1");
        assert_eq!(result, Some(&Value::String("b".to_owned())));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_empty_path_returns_root() {
        let json = serde_json::json!({"foo": "bar"});
        let result = lookup_value(&json, "");
        assert_eq!(result, Some(&json));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_consecutive_dots_handled() {
        let json = serde_json::json!({"foo": {"bar": "value"}});
        // Path "foo..bar" should skip the empty segment and find "foo.bar"
        let result = lookup_value(&json, "foo..bar");
        assert_eq!(result, Some(&Value::String("value".to_owned())));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_leading_dot_handled() {
        let json = serde_json::json!({"foo": "bar"});
        // Path ".foo" should skip the leading empty segment
        let result = lookup_value(&json, ".foo");
        assert_eq!(result, Some(&Value::String("bar".to_owned())));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_invalid_array_index_returns_none() {
        let json = serde_json::json!({"items": [1, 2, 3]});
        let result = lookup_value(&json, "items.abc");
        assert_eq!(result, None);
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_array_out_of_bounds_returns_none() {
        let json = serde_json::json!({"items": [1, 2, 3]});
        let result = lookup_value(&json, "items.100");
        assert_eq!(result, None);
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_through_primitive_returns_none() {
        let json = serde_json::json!({"foo": "bar"});
        // Can't traverse through a string
        let result = lookup_value(&json, "foo.baz");
        assert_eq!(result, None);
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn format_shows_full_string() {
        let long_string = "a".repeat(300);
        let value = Value::String(long_string.clone());

        let formatted = format_value(Some(&value));
        // Full JSON string with quotes
        assert_eq!(formatted, format!("\"{long_string}\""));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn format_array_shows_full_json() {
        let value = serde_json::json!([1, 2, 3, 4, 5]);

        let formatted = format_value(Some(&value));
        assert_eq!(formatted, "[1,2,3,4,5]");
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn format_object_shows_full_json() {
        let value = serde_json::json!({"a": 1, "b": 2});

        let formatted = format_value(Some(&value));
        // JSON object serialization order may vary, check both keys present
        assert!(formatted.contains("\"a\":1"));
        assert!(formatted.contains("\"b\":2"));
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn format_none_shows_placeholder() {
        let formatted = format_value(None);
        assert_eq!(formatted, "<unable to retrieve>");
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn lookup_option_marker_skipped() {
        // serde_ignored uses '?' for Option wrappers
        let json = serde_json::json!({"outer": {"inner": "value"}});
        // Path "?.outer.?.inner" should skip ? markers
        let result = lookup_value(&json, "?.outer.?.inner");
        assert_eq!(result, Some(&Value::String("value".to_owned())));
    }
}



================================================
FILE: src/types.rs
================================================
//! Re-exported types from external crates for convenience.
//!
//! These types are commonly used in this SDK and are re-exported here
//! so users don't need to add these dependencies to their `Cargo.toml`.

/// Ethereum address type and the [`address!`] macro for compile-time address literals.
/// [`ChainId`] is a type alias for `u64` representing EVM chain IDs.
/// [`Signature`] represents cryptographic signatures for signed orders.
/// [`B256`] is a 256-bit fixed-size byte array type used for condition IDs and hashes.
/// [`U256`] is a 256-bit integer
pub use alloy::primitives::{Address, B256, ChainId, Signature, U256, address, b256};
/// Date and time types for timestamps in API responses and order expiration.
pub use chrono::{DateTime, NaiveDate, Utc};
/// Arbitrary precision decimal type for prices, sizes, and amounts.
pub use rust_decimal::Decimal;
/// Macro for creating [`Decimal`] literals at compile time.
///
/// # Example
/// ```
/// use polymarket_client_sdk::types::dec;
/// let price = dec!(0.55);
/// ```
pub use rust_decimal_macros::dec;



================================================
FILE: src/bridge/client.rs
================================================
use reqwest::{
    Client as ReqwestClient, Method,
    header::{HeaderMap, HeaderValue},
};
use url::Url;

use super::types::{
    DepositRequest, DepositResponse, StatusRequest, StatusResponse, SupportedAssetsResponse,
};
use crate::Result;

/// Client for the Polymarket Bridge API.
///
/// The Bridge API enables bridging assets from various chains (EVM, Solana, Bitcoin)
/// to USDC.e on Polygon for trading on Polymarket.
///
/// # Example
///
/// ```no_run
/// use polymarket_client_sdk::types::address;
/// use polymarket_client_sdk::bridge::{Client, types::DepositRequest};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::default();
///
/// // Get deposit addresses
/// let request = DepositRequest::builder()
///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
///     .build();
/// let response = client.deposit(&request).await?;
///
/// // Get supported assets
/// let assets = client.supported_assets().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    host: Url,
    client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Client::new("https://bridge.polymarket.com")
            .expect("Client with default endpoint should succeed")
    }
}

impl Client {
    /// Creates a new Bridge API client with a custom host.
    ///
    /// # Errors
    ///
    /// Returns an error if the host URL is invalid or the HTTP client fails to build.
    pub fn new(host: &str) -> Result<Client> {
        let mut headers = HeaderMap::new();

        headers.insert("User-Agent", HeaderValue::from_static("rs_clob_client"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let client = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self {
            host: Url::parse(host)?,
            client,
        })
    }

    /// Returns the host URL for the client.
    #[must_use]
    pub fn host(&self) -> &Url {
        &self.host
    }

    #[must_use]
    fn client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Create deposit addresses for a Polymarket wallet.
    ///
    /// Generates unique deposit addresses for bridging assets to Polymarket.
    /// Returns addresses for EVM-compatible chains, Solana, and Bitcoin.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polymarket_client_sdk::types::address;
    /// use polymarket_client_sdk::bridge::{Client, types::DepositRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    /// let request = DepositRequest::builder()
    ///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
    ///     .build();
    ///
    /// let response = client.deposit(&request).await?;
    /// println!("EVM: {}", response.address.evm);
    /// println!("SVM: {}", response.address.svm);
    /// println!("BTC: {}", response.address.btc);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn deposit(&self, request: &DepositRequest) -> Result<DepositResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}deposit", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get all supported chains and tokens for deposits.
    ///
    /// Returns information about which assets can be deposited and their
    /// minimum deposit amounts in USD.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polymarket_client_sdk::bridge::Client;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    /// let response = client.supported_assets().await?;
    ///
    /// for asset in response.supported_assets {
    ///     println!(
    ///         "{} ({}) on {} - min: ${:.2}",
    ///         asset.token.name,
    ///         asset.token.symbol,
    ///         asset.chain_name,
    ///         asset.min_checkout_usd
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn supported_assets(&self) -> Result<SupportedAssetsResponse> {
        let request = self
            .client()
            .request(Method::GET, format!("{}supported-assets", self.host()))
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get the transaction status for all deposits associated with a given deposit address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use polymarket_client_sdk::bridge::{Client, types::StatusRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    ///
    /// let request = StatusRequest::builder()
    ///     .address("56687bf447db6ffa42ffe2204a05edaa20f55839")
    ///     .build();
    /// let response = client.status(&request).await?;
    ///
    /// for tx in response.transactions {
    ///     println!(
    ///         "Sent {} amount of token {} on chainId {} to destination chainId {} with status {:?}",
    ///         tx.from_amount_base_unit,
    ///         tx.from_token_address,
    ///         tx.from_chain_id,
    ///         tx.to_chain_id,
    ///         tx.status
    ///     );
    /// }
    /// # Ok(())
    /// # }
    ///
    /// ```
    pub async fn status(&self, request: &StatusRequest) -> Result<StatusResponse> {
        let request = self
            .client()
            .request(
                Method::GET,
                format!("{}status/{}", self.host(), request.address),
            )
            .build()?;

        crate::request(&self.client, request, None).await
    }
}



================================================
FILE: src/bridge/mod.rs
================================================
//! Polymarket Bridge API client and types.
//!
//! **Feature flag:** `bridge` (required to use this module)
//!
//! This module provides a client for interacting with the Polymarket Bridge API,
//! which enables bridging assets from various chains (EVM, Solana, Bitcoin) to
//! USDC.e on Polygon for trading on Polymarket.
//!
//! # Overview
//!
//! The Bridge API is a read/write HTTP API that provides:
//! - Deposit address generation for multi-chain asset bridging
//! - Supported asset and chain information
//!
//! ## Available Endpoints
//!
//! | Endpoint | Method | Description |
//! |----------|--------|-------------|
//! | `/deposit` | POST | Create deposit addresses for a wallet |
//! | `/supported-assets` | GET | Get supported chains and tokens |
//!
//! # Example
//!
//! ```no_run
//! use polymarket_client_sdk::types::address;
//! use polymarket_client_sdk::bridge::{Client, types::DepositRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client with the default endpoint
//! let client = Client::default();
//!
//! // Get deposit addresses for a wallet
//! let request = DepositRequest::builder()
//!     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
//!     .build();
//!
//! let response = client.deposit(&request).await?;
//! println!("EVM: {}", response.address.evm);
//! println!("SVM: {}", response.address.svm);
//! println!("BTC: {}", response.address.btc);
//! # Ok(())
//! # }
//! ```
//!
//! # API Base URL
//!
//! The default API endpoint is `https://bridge.polymarket.com`.

pub mod client;
pub mod types;

pub use client::Client;



================================================
FILE: src/bridge/types/mod.rs
================================================
mod request;
mod response;

pub use request::*;
pub use response::*;



================================================
FILE: src/bridge/types/request.rs
================================================
use bon::Builder;
use serde::Serialize;

use crate::types::Address;

/// Request to create deposit addresses for a Polymarket wallet.
///
/// # Example
///
/// ```
/// use polymarket_client_sdk::types::address;
/// use polymarket_client_sdk::bridge::types::DepositRequest;
///
/// let request = DepositRequest::builder()
///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
///     .build();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Builder)]
pub struct DepositRequest {
    /// The Polymarket wallet address to generate deposit addresses for.
    pub address: Address,
}

/// Request to get deposit statuses for a given deposit address.
///
/// ### Note: This doesn't use the alloy Address type, since it supports Solana and Bitcoin addresses.
///
/// # Example
///
/// ```
/// use polymarket_client_sdk::bridge::types::StatusRequest;
///
/// let request = StatusRequest::builder().address("0x9cb12Ec30568ab763ae5891ce4b8c5C96CeD72C9").build();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Builder)]
#[builder(on(String, into))]
pub struct StatusRequest {
    pub address: String,
}



================================================
FILE: src/bridge/types/response.rs
================================================
use alloy::primitives::U256;
use bon::Builder;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::types::{Address, ChainId, Decimal};

/// Response containing deposit addresses for different blockchain networks.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
pub struct DepositResponse {
    /// Deposit addresses for different blockchain networks.
    pub address: DepositAddresses,
    /// Additional information about supported chains.
    pub note: Option<String>,
}

/// Deposit addresses for different blockchain networks.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct DepositAddresses {
    /// EVM-compatible deposit address (Ethereum, Polygon, Arbitrum, Base, etc.).
    pub evm: Address,
    /// Solana Virtual Machine deposit address.
    pub svm: String,
    /// Bitcoin deposit address.
    pub btc: String,
}

/// Response containing all supported assets for deposits.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SupportedAssetsResponse {
    /// List of supported assets with minimum deposit amounts.
    pub supported_assets: Vec<SupportedAsset>,
    /// Additional information about supported chains and assets.
    pub note: Option<String>,
}

/// A supported asset with chain and token information.
#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SupportedAsset {
    /// Blockchain chain ID (e.g., 1 for Ethereum mainnet, 137 for Polygon).
    /// Deserialized from JSON string representation (e.g., `"137"`).
    #[serde_as(as = "DisplayFromStr")]
    pub chain_id: ChainId,
    /// Human-readable chain name.
    pub chain_name: String,
    /// Token information.
    pub token: Token,
    /// Minimum deposit amount in USD.
    pub min_checkout_usd: Decimal,
}

/// Token information for a supported asset.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct Token {
    /// Full token name.
    pub name: String,
    /// Token symbol.
    pub symbol: String,
    /// Token contract address.
    pub address: String,
    /// Token decimals.
    pub decimals: u8,
}

/// Transaction status for all deposits associated with a given deposit address.
#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    /// List of transactions for the given address
    pub transactions: Vec<DepositTransaction>,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct DepositTransaction {
    /// Source chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub from_chain_id: ChainId,
    /// Source token contract address
    pub from_token_address: String,
    /// Amount in base units (without decimals)
    #[serde_as(as = "DisplayFromStr")]
    pub from_amount_base_unit: U256,
    /// Destination chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    /// Destination chain ID
    pub to_token_address: Address,
    /// Current status of the transaction
    pub status: DepositTransactionStatus,
    /// Transaction hash (only available when status is Completed)
    pub tx_hash: Option<String>,
    /// Unix timestamp in milliseconds when transaction was created (missing when status is `DepositDetected`)
    pub created_time_ms: Option<u64>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepositTransactionStatus {
    DepositDetected,
    Processing,
    OriginTxConfirmed,
    Submitted,
    Completed,
    Failed,
}



================================================
FILE: src/clob/mod.rs
================================================
//! Polymarket CLOB (Central Limit Order Book) API client and types.
//!
//! **Feature flag:** None (this is the core module, always available)
//!
//! This module provides the primary client for interacting with the Polymarket CLOB API,
//! which handles all trading operations including order placement, cancellation, market
//! data queries, and account management.
//!
//! # Overview
//!
//! The CLOB API is the main trading interface for Polymarket. It supports both
//! authenticated and unauthenticated operations:
//!
//! - **Unauthenticated**: Market data, pricing, orderbooks, health checks
//! - **Authenticated**: Order placement/cancellation, balances, API keys, rewards
//! - **Builder Authentication**: Special endpoints for market makers and builders
//!
//! ## Public Endpoints (No Authentication Required)
//!
//! | Endpoint | Description |
//! |----------|-------------|
//! | `/` | Health check - returns "OK" |
//! | `/time` | Current server timestamp |
//! | `/midpoint` | Mid-market price for a token |
//! | `/midpoints` | Batch midpoint prices |
//! | `/price` | Best bid or ask price |
//! | `/prices` | Batch best prices |
//! | `/spread` | Bid-ask spread |
//! | `/spreads` | Batch spreads |
//! | `/last-trade-price` | Most recent trade price |
//! | `/last-trades-prices` | Batch last trade prices |
//! | `/prices-all` | All token prices |
//! | `/tick-size` | Minimum price increment (cached) |
//! | `/neg-risk` | `NegRisk` adapter flag (cached) |
//! | `/fee-rate-bps` | Trading fee in basis points (cached) |
//! | `/book` | Full orderbook depth |
//! | `/books` | Batch orderbooks |
//! | `/market` | Single market details |
//! | `/markets` | All markets (paginated) |
//! | `/sampling-markets` | Sampling program markets |
//! | `/simplified-markets` | Markets with reduced detail |
//! | `/sampling-simplified-markets` | Simplified sampling markets |
//! | `/data/price-history` | Historical price data |
//! | `/geoblock` | Geographic restriction check |
//!
//! ## Authenticated Endpoints
//!
//! | Endpoint | Description |
//! |----------|-------------|
//! | `/order` | Place a new order |
//! | `/cancel` | Cancel an order |
//! | `/cancel-market-orders` | Cancel all orders in a market |
//! | `/cancel-all` | Cancel all orders |
//! | `/orders` | Get user's orders |
//! | `/trades` | Get user's trade history |
//! | `/balances` | Get USDC balances and allowances |
//! | `/api-keys` | List API keys |
//! | `/create-api-key` | Create new API key |
//! | `/delete-api-key` | Delete an API key |
//! | `/notifications` | Get notifications |
//! | `/mark-notifications-as-read` | Mark notifications read |
//! | `/drop-notifications` | Delete notifications |
//! | `/rewards/current` | Current rewards info |
//! | `/rewards/percentages` | Rewards percentages |
//! | `/order-scoring` | Order score for rewards |
//! | `/ban` | Check ban status |
//!
//! # Examples
//!
//! ## Unauthenticated Client
//!
//! ```rust,no_run
//! use std::str::FromStr as _;
//!
//! use polymarket_client_sdk::clob::{Client, Config};
//! use polymarket_client_sdk::clob::types::request::MidpointRequest;
//! use polymarket_client_sdk::types::U256;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an unauthenticated client
//! let client = Client::new("https://clob.polymarket.com", Config::default())?;
//!
//! // Check API health
//! let status = client.ok().await?;
//! println!("Status: {status}");
//!
//! // Get midpoint price for a token
//! let request = MidpointRequest::builder()
//!     .token_id(U256::from_str("15871154585880608648532107628464183779895785213830018178010423617714102767076")?)
//!     .build();
//! let midpoint = client.midpoint(&request).await?;
//! println!("Midpoint: {}", midpoint.mid);
//! # Ok(())
//! # }
//! ```
//!
//! ## Authenticated Client
//!
//! ```rust,no_run
//! use std::str::FromStr as _;
//!
//! use alloy::signers::Signer;
//! use alloy::signers::local::LocalSigner;
//! use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};
//! use polymarket_client_sdk::clob::{Client, Config};
//! use polymarket_client_sdk::clob::types::{Side, SignedOrder};
//! use polymarket_client_sdk::types::{dec, Decimal, U256};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create signer from private key
//! let private_key = std::env::var(PRIVATE_KEY_VAR)?;
//! let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
//!
//! let client = Client::new("https://clob.polymarket.com", Config::default())?
//!     .authentication_builder(&signer)
//!     .authenticate()
//!     .await?;
//!
//! let order = client
//!     .limit_order()
//!     .token_id(U256::from_str("15871154585880608648532107628464183779895785213830018178010423617714102767076")?)
//!     .side(Side::Buy)
//!     .price(dec!(0.5))
//!     .size(Decimal::TEN)
//!     .build()
//!     .await?;
//!
//! let signed_order = client.sign(&signer, order).await?;
//! let response = client.post_order(signed_order).await?;
//! println!("Order ID: {}", response.order_id);
//! # Ok(())
//! # }
//! ```
//!
//! # Optional Features
//!
//! - **`ws`**: Enables WebSocket support for real-time orderbook and trade streams
//! - **`heartbeats`**: Enables automatic heartbeat mechanism for authenticated sessions
//! - **`tracing`**: Enables detailed request/response tracing
//! - **`rfq`**: Enables RFQ (Request for Quote) endpoints for institutional trading
//!
//! # API Base URL
//!
//! The default API endpoint is `https://clob.polymarket.com`.

pub mod client;
pub mod order_builder;
pub mod types;
#[cfg(feature = "ws")]
pub mod ws;

pub use client::{Client, Config};



================================================
FILE: src/clob/order_builder.rs
================================================
use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

use alloy::primitives::U256;
use chrono::{DateTime, Utc};
use rand::Rng as _;
use rust_decimal::prelude::ToPrimitive as _;

use crate::Result;
use crate::auth::Kind as AuthKind;
use crate::auth::state::Authenticated;
use crate::clob::Client;
use crate::clob::types::request::OrderBookSummaryRequest;
use crate::clob::types::{
    Amount, AmountInner, Order, OrderType, Side, SignableOrder, SignatureType,
};
use crate::error::Error;
use crate::types::{Address, Decimal};

pub(crate) const USDC_DECIMALS: u32 = 6;

/// Maximum number of decimal places for `size`
pub(crate) const LOT_SIZE_SCALE: u32 = 2;

/// Placeholder type for compile-time checks on limit order builders
#[non_exhaustive]
#[derive(Debug)]
pub struct Limit;

/// Placeholder type for compile-time checks on market order builders
#[non_exhaustive]
#[derive(Debug)]
pub struct Market;

/// Used to create an order iteratively and ensure validity with respect to its order kind.
#[derive(Debug)]
pub struct OrderBuilder<OrderKind, K: AuthKind> {
    pub(crate) client: Client<Authenticated<K>>,
    pub(crate) signer: Address,
    pub(crate) signature_type: SignatureType,
    pub(crate) salt_generator: fn() -> u64,
    pub(crate) token_id: Option<U256>,
    pub(crate) price: Option<Decimal>,
    pub(crate) size: Option<Decimal>,
    pub(crate) amount: Option<Amount>,
    pub(crate) side: Option<Side>,
    pub(crate) nonce: Option<u64>,
    pub(crate) expiration: Option<DateTime<Utc>>,
    pub(crate) taker: Option<Address>,
    pub(crate) order_type: Option<OrderType>,
    pub(crate) post_only: Option<bool>,
    pub(crate) funder: Option<Address>,
    pub(crate) _kind: PhantomData<OrderKind>,
}

impl<OrderKind, K: AuthKind> OrderBuilder<OrderKind, K> {
    /// Sets the `token_id` for this builder. This is a required field.
    #[must_use]
    pub fn token_id(mut self, token_id: U256) -> Self {
        self.token_id = Some(token_id);
        self
    }

    /// Sets the [`Side`] for this builder. This is a required field.
    #[must_use]
    pub fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    /// Sets the nonce for this builder.
    #[must_use]
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    #[must_use]
    pub fn expiration(mut self, expiration: DateTime<Utc>) -> Self {
        self.expiration = Some(expiration);
        self
    }

    #[must_use]
    pub fn taker(mut self, taker: Address) -> Self {
        self.taker = Some(taker);
        self
    }

    #[must_use]
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Sets the `postOnly` flag for this builder.
    #[must_use]
    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }
}

impl<K: AuthKind> OrderBuilder<Limit, K> {
    /// Sets the price for this limit builder. This is a required field.
    #[must_use]
    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    /// Sets the size for this limit builder. This is a required field.
    #[must_use]
    pub fn size(mut self, size: Decimal) -> Self {
        self.size = Some(size);
        self
    }

    /// Validates and transforms this limit builder into a [`SignableOrder`]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), err(level = "warn"))
    )]
    pub async fn build(self) -> Result<SignableOrder> {
        let Some(token_id) = self.token_id else {
            return Err(Error::validation(
                "Unable to build Order due to missing token ID",
            ));
        };

        let Some(side) = self.side else {
            return Err(Error::validation(
                "Unable to build Order due to missing token side",
            ));
        };

        let Some(price) = self.price else {
            return Err(Error::validation(
                "Unable to build Order due to missing price",
            ));
        };

        if price.is_sign_negative() {
            return Err(Error::validation(format!(
                "Unable to build Order due to negative price {price}"
            )));
        }

        let fee_rate = self.client.fee_rate_bps(token_id).await?;
        let minimum_tick_size = self
            .client
            .tick_size(token_id)
            .await?
            .minimum_tick_size
            .as_decimal();

        let decimals = minimum_tick_size.scale();

        if price.scale() > minimum_tick_size.scale() {
            return Err(Error::validation(format!(
                "Unable to build Order: Price {price} has {} decimal places. Minimum tick size \
                {minimum_tick_size} has {} decimal places. Price decimal places <= minimum tick size decimal places",
                price.scale(),
                minimum_tick_size.scale()
            )));
        }

        if price < minimum_tick_size || price > Decimal::ONE - minimum_tick_size {
            return Err(Error::validation(format!(
                "Price {price} is too small or too large for the minimum tick size {minimum_tick_size}"
            )));
        }

        let Some(size) = self.size else {
            return Err(Error::validation(
                "Unable to build Order due to missing size",
            ));
        };

        if size.scale() > LOT_SIZE_SCALE {
            return Err(Error::validation(format!(
                "Unable to build Order: Size {size} has {} decimal places. Maximum lot size is {LOT_SIZE_SCALE}",
                size.scale()
            )));
        }

        if size.is_zero() || size.is_sign_negative() {
            return Err(Error::validation(format!(
                "Unable to build Order due to negative size {size}"
            )));
        }

        let nonce = self.nonce.unwrap_or(0);
        let expiration = self.expiration.unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
        let taker = self.taker.unwrap_or(Address::ZERO);
        let order_type = self.order_type.unwrap_or(OrderType::GTC);
        let post_only = Some(self.post_only.unwrap_or(false));

        if !matches!(order_type, OrderType::GTD) && expiration > DateTime::<Utc>::UNIX_EPOCH {
            return Err(Error::validation(
                "Only GTD orders may have a non-zero expiration",
            ));
        }

        if post_only == Some(true) && !matches!(order_type, OrderType::GTC | OrderType::GTD) {
            return Err(Error::validation(
                "postOnly is only supported for GTC and GTD orders",
            ));
        }

        // When buying `YES` tokens, the user will "make" `size` * `price` USDC and "take"
        // `size` `YES` tokens, and vice versa for sells. We have to truncate the notional values
        // to the combined precision of the tick size _and_ the lot size. This is to ensure that
        // this order will "snap" to the precision of resting orders on the book. The returned
        // values are quantized to `USDC_DECIMALS`.
        //
        // e.g. User submits a limit order to buy 100 `YES` tokens at $0.34.
        // This means they will take/receive 100 `YES` tokens, make/give up 34 USDC. This means that
        // the `taker_amount` is `100000000` and the `maker_amount` of `34000000`.
        let (taker_amount, maker_amount) = match side {
            Side::Buy => (
                size,
                (size * price).trunc_with_scale(decimals + LOT_SIZE_SCALE),
            ),
            Side::Sell => (
                (size * price).trunc_with_scale(decimals + LOT_SIZE_SCALE),
                size,
            ),
            side => return Err(Error::validation(format!("Invalid side: {side}"))),
        };

        let salt = to_ieee_754_int((self.salt_generator)());

        let order = Order {
            salt: U256::from(salt),
            maker: self.funder.unwrap_or(self.signer),
            taker,
            tokenId: token_id,
            makerAmount: U256::from(to_fixed_u128(maker_amount)),
            takerAmount: U256::from(to_fixed_u128(taker_amount)),
            side: side as u8,
            feeRateBps: U256::from(fee_rate.base_fee),
            nonce: U256::from(nonce),
            signer: self.signer,
            expiration: U256::from(expiration.timestamp().to_u64().ok_or(Error::validation(
                format!("Unable to represent expiration {expiration} as a u64"),
            ))?),
            signatureType: self.signature_type as u8,
        };

        #[cfg(feature = "tracing")]
        tracing::debug!(token_id = %token_id, side = ?side, price = %price, size = %size, "limit order built");

        Ok(SignableOrder {
            order,
            order_type,
            post_only,
        })
    }
}

impl<K: AuthKind> OrderBuilder<Market, K> {
    /// Sets the price for this market builder. This is an optional field.
    #[must_use]
    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    /// Sets the [`Amount`] for this market order. This is a required field.
    #[must_use]
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    // Attempts to calculate the market price from the top of the book for the particular token.
    // - Uses an orderbook depth search to find the cutoff price:
    //   - BUY + USDC: walk asks until notional >= USDC
    //   - BUY + Shares: walk asks until shares >= N
    //   - SELL + Shares: walk bids until shares >= N
    async fn calculate_price(&self, order_type: OrderType) -> Result<Decimal> {
        let token_id = self
            .token_id
            .expect("Token ID was already validated in `build`");
        let side = self.side.expect("Side was already validated in `build`");
        let amount = self
            .amount
            .as_ref()
            .expect("Amount was already validated in `build`");

        let book = self
            .client
            .order_book(&OrderBookSummaryRequest {
                token_id,
                side: None,
            })
            .await?;

        if !matches!(order_type, OrderType::FAK | OrderType::FOK) {
            return Err(Error::validation(
                "Cannot set an order type other than FAK/FOK for a market order",
            ));
        }

        let (levels, amount) = match side {
            Side::Buy => (book.asks, amount.0),
            Side::Sell => match amount.0 {
                a @ AmountInner::Shares(_) => (book.bids, a),
                AmountInner::Usdc(_) => {
                    return Err(Error::validation(
                        "Sell Orders must specify their `amount`s in shares",
                    ));
                }
            },

            side => return Err(Error::validation(format!("Invalid side: {side}"))),
        };

        let first = levels.first().ok_or(Error::validation(format!(
            "No opposing orders for {token_id} which means there is no market price"
        )))?;

        let mut sum = Decimal::ZERO;
        let cutoff_price = levels.iter().rev().find_map(|level| {
            match amount {
                AmountInner::Usdc(_) => sum += level.size * level.price,
                AmountInner::Shares(_)