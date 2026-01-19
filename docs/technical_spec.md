# Bankai Terminal Technical Specification

## 1. System Overview
*   **Core Purpose:** A high-frequency quantitative trading engine designed to arbitrage price discrepancies between real-world asset spot prices (Binance) and prediction market probabilities (Polymarket). It utilizes the Allora Network for AI-driven trend confirmation to filter false arbitrage signals.
*   **Value Proposition:** Maximizes "Capital Velocity" by compounding small edges ($50 -> $5,000) through immediate post-resolution capital recycling and fee-aware execution strategies.
*   **System Architecture:**
    *   **Host Language:** Rust (Tokio/Tungstenite) for sub-millisecond I/O and execution.
    *   **Logic Layer:** Python 3.11 (via PyO3) for statistical modeling and meta-optimization.
    *   **State Management:** Redis (Hot/Ephemeral) for live order books; TimescaleDB (Cold/Persistent) for analytics.
    *   **Blockchain Interaction:** Dual-rail execution (HTTP Relayer + Direct RPC) on Polygon PoS.

## 2. Project Structure
```text
bankai-terminal/
├── Cargo.toml                 # Rust dependencies (tokio, ethers, pyo3, redis, sqlx)
├── build.rs                   # PyO3 linking configuration
├── config/
│   ├── strategies.json        # Dynamic params (kelly_fraction, snipe_threshold)
│   └── secrets.enc            # AES-256 encrypted keystore (Private Key, API Keys)
├── abi/
│   ├── CTFExchange.json       # Polymarket Exchange Artifact
│   └── ConditionalTokens.json # Gnosis CTF Artifact
├── src/
│   ├── main.rs                # Entry point, Runtime Bootstrap, TUI init
│   ├── oracle/
│   │   ├── binance.rs         # WSS (bookTicker + aggTrade)
│   │   ├── allora.rs          # RPC Poller (GetLatestAvailableNetworkInferences)
│   │   └── polymarket.rs      # WSS (OrderBook) + Gamma API (Discovery)
│   ├── engine/
│   │   ├── risk.rs            # Volatility & Staleness Guards
│   │   └── strategy.rs        # EV Calculation & Signal Alignment
│   ├── execution/
│   │   ├── relayer.rs         # Rail A: HTTP API Client
│   │   ├── direct.rs          # Rail B: Ethers-rs Contract Interaction
│   │   └── nonce.rs           # Redis-backed Atomic Nonce Manager
│   ├── storage/
│   │   ├── redis.rs           # ZSET/HASH abstractions for OrderBook
│   │   └── database.rs        # TimescaleDB Connection Pool
│   └── ui/                    # Ratatui TUI modules
├── python/                    # Embedded Python Environment
│   ├── models/                # ML Models
│   └── meta_brain.py          # 6h Optimization Loop
├── tests/
│   ├── simulation/            # Market replay tests
│   └── fork/                  # Anvil/Foundry Mainnet Fork tests
└── docker-compose.yml         # Redis, TimescaleDB services
```

## 3. Feature Specification

### 3.1 Oracle Module (Data Ingestion)
*   **User Story:** As the system, I need zero-latency access to Binance prices and reliable access to Allora predictions to form a valid trade signal.
*   **Implementation Steps:**
    1.  **Binance:** Connect WSS to `wss://stream.binance.com:9443/ws`.
        *   Subscribe to `<symbol>@bookTicker` for best bid/ask (Immediate EV calculation).
        *   Subscribe to `<symbol>@aggTrade` for volume/volatility calculations.
        *   *Calculation:* `Rolling_Vol_1m` = StdDev of price changes over last 60s window.
    2.  **Allora:** Poll RPC endpoint `https://allora-rpc.testnet.allora.network` (or Mainnet) every 2 seconds (block time).
        *   Method: `allora.emissions.v1.Query/GetLatestAvailableNetworkInferences`.
        *   *Staleness Guard:* Calculate `Time_Decay = (Now - Inference.timestamp) / Topic.EpochLength`. If `Time_Decay > 0.05` (5%), return `Signal::Invalid`.
    3.  **Polymarket:**
        *   **Snapshot:** On startup/reconnect, call `GET /book?token_id={id}` to seed Redis.
        *   **Stream:** Connect WSS `wss://ws-live-data.polymarket.com`. Subscribe to `price_change` (Sept 2025 Schema).
        *   *Updates:* Apply changes to Redis Order Book.

### 3.2 Velocity Execution Engine
*   **User Story:** I need to select the optimal order type (Maker vs. Taker) based on the magnitude of the edge and current market volatility.
*   **Implementation Steps:**
    1.  **Event Loop:** Triggered on any Oracle update.
    2.  **Safety Check:**
        *   If `Binance_Vol > Config.Max_Vol` AND `Allora_Signal == Neutral`: **HALT**.
        *   If `Polymarket.Liquidity < $2000`: **SKIP**.
    3.  **EV Calculation:**
        *   `Implied_Prob = 1 / Decimal_Odds`
        *   `True_Prob` derived from Binance Price + Allora Confidence Interval.
        *   `Edge = True_Prob - Implied_Prob`.
    4.  **Mode Selector:**
        *   **Snipe (Taker):** If `Edge > (Taker_Fee_Bps + Gas_Cost_Eq + Snipe_Min_Edge)`:
            *   Generate `FOK` (Fill-Or-Kill) order targeting the best Ask.
        *   **Ladder (Maker):** If `Edge > 0` but below Snipe threshold:
            *   Generate 3 `GTC` Limit orders at `MidPrice - Spread_Offset`.
            *   *Constraint:* Must not cross the spread.

### 3.3 Dual-Rail Execution
*   **User Story:** I must execute trades even if the Polymarket Relayer is congested or down.
*   **Implementation Steps:**
    1.  **Nonce Management:** Fetch `transactionCount` from chain on boot. Store in Redis `nonce:{address}`. Use `INCR` for every attempt.
    2.  **Rail A (Relayer):**
        *   Construct JSON payload matching `POST /order`.
        *   Sign EIP-712 Typed Data (`Order` struct).
        *   Send HTTP Request. Timeout: 500ms.
    3.  **Rail B (Direct - Failover):**
        *   Triggered on Rail A timeout or 5xx error.
        *   Construct Raw Transaction calling `CTFExchange.fillOrders()`.
        *   Sign with local Private Key.
        *   Broadcast via Premium RPC (e.g., Alchemy/Infura) to Polygon Mainnet.
        *   *Optimization:* Use Flashbots Protect or similar private RPC to avoid mempool front-running if possible.

## 4. Database Schema

### 4.1 Redis (Hot Storage)
Used for state that changes millisecond-to-millisecond.

*   **Order Book (Price Ordering):**
    *   `book:{token_id}:bids` -> `ZSET<Price_Float, Price_String>`
    *   `book:{token_id}:asks` -> `ZSET<Price_Float, Price_String>`
    *   *Constraint:* Scores are floats for sorting. Members are strings to prevent precision loss.
*   **Order Book (Liquidity):**
    *   `book:{token_id}:depth` -> `HASH<Price_String, Quantity_Float>`
*   **System State:**
    *   `sys:volatility:{asset}` -> `Float`
    *   `sys:nonce:{wallet_address}` -> `Integer`
    *   `sys:bankroll:usdc` -> `Float` (Updated via Event Listener)

### 4.2 TimescaleDB (Cold Storage)
Used for training the "Meta-Brain".

*   **Table: `market_ticks`** (Hypertable)
    *   `time` (TIMESTAMPTZ, NOT NULL)
    *   `market_id` (TEXT)
    *   `binance_price` (DECIMAL)
    *   `poly_best_bid` (DECIMAL)
    *   `poly_best_ask` (DECIMAL)
    *   `allora_pred` (DECIMAL)

*   **Table: `trade_execution`**
    *   `id` (UUID, PK)
    *   `time` (TIMESTAMPTZ)
    *   `rail` (ENUM: 'RELAYER', 'DIRECT')
    *   `mode` (ENUM: 'SNIPE', 'LADDER')
    *   `expected_ev` (DECIMAL)
    *   `actual_pnl` (DECIMAL, Nullable)
    *   `fees_paid` (DECIMAL)

## 5. Server Actions (Async Tasks)

### 5.1 Redemption Listener
*   **Description:** Listens for market resolution to recycle capital.
*   **Trigger:** `ConditionResolution` event on Gnosis CTF Contract.
*   **Action:**
    1.  Verify local holding of winning outcome.
    2.  Construct `redeemPositions` transaction.
    3.  Send via **Direct Rail** (Rail B) with aggressive gas settings to ensure immediate inclusion.
    4.  On confirmation, update `sys:bankroll:usdc`.

### 5.2 Meta-Brain Optimization (Python)
*   **Trigger:** Cron every 6 hours.
*   **Action:**
    1.  Load `trade_execution` logs from TimescaleDB.
    2.  Calculate Sharpe Ratio per Strategy (Snipe vs Ladder).
    3.  If `Snipe_Sharpe < 1.0`, increment `Snipe_Min_Edge` in `config.json`.
    4.  If `Ladder_Fill_Rate < 5%`, decrement `Spread_Offset` in `config.json`.

## 6. Design System (TUI)

### 6.1 Visual Style
*   **Library:** `ratatui` (Rust).
*   **Theme:** "Cyberpunk Terminal".
    *   Background: Black/Dark Gray.
    *   Text: Green (Profits), Red (Losses/Errors), Cyan (Info).
    *   Borders: Double lines for main sections.

### 6.2 Components
*   **Status Bar (Top):** `Bankai v1.0 | uptime: 12h | ETH: $2450 (Vol: LOW) | Allora: ONLINE`
*   **Market Grid (Left):**
    *   Columns: `Ticker | Gap% | EV | Mode | Action`
    *   Example: `BTC-24H | -1.2% | +0.04 | SNIPE | FOK_SENT`
*   **Log Stream (Bottom Right):** Rolling logs of decision engine output.
*   **Wallet (Top Right):** `USDC: $1,240 | Active: $4,500 | 24h PnL: +12%`

## 7. Component Architecture

### 7.1 Rust (Core)
*   **`OracleManager`**: Spawns Tokio tasks for WS/RPC connections. Broadcasts `MarketUpdate` enum on an internal bus.
*   **`Engine`**: Subscribes to `MarketUpdate`. Holds `PyObject` (Python VM). outputs `TradeIntent`.
*   **`Executor`**: Consumes `TradeIntent`. Handles Rails A/B and Nonce logic.

### 7.2 Python (Logic)
*   **`strategy.py`**:
    *   `def calculate_kelly(win_prob: float, odds: float) -> float`: Returns position size.
    *   `def validate_signal(inference: dict, current_price: float) -> bool`: Returns true if signal aligns with price action.

## 8. Authentication & Authorization
*   **Key Storage:** Keys are stored in `config/secrets.enc` using AES-256-GCM.
*   **Runtime Decryption:**
    *   App prompts for password on `stdin` at startup.
    *   Keys are decrypted into protected memory (`secrecy` crate).
    *   Keys are dropped from memory on `SIGINT`.
*   **No Network Auth:** The terminal acts as a standalone client. No external login servers.

## 9. Data Flow
1.  **Binance** pushes `bookTicker` -> **Rust Oracle** normalizes to `Decimal`.
2.  **Allora** poll returns `Inference` -> **Rust Oracle** updates state.
3.  **Engine** detects `Price_Binance != Price_Poly`.
4.  **Engine** calls Python `validate_signal()`.
5.  **Engine** emits `TradeIntent(Snipe, FOK)`.
6.  **Executor** checks Redis Nonce -> Signs EIP-712 -> Posts to Relayer.
7.  **Executor** (if Relayer fails) -> Signs Raw Tx -> Sends to Alchemy.

## 10. Analytics
*   **PostHog:** *Not Used.*
*   **Internal:** TimescaleDB stores all decision metrics.
*   **Visualization:** Grafana connected to TimescaleDB for post-mortem analysis of trade performance.

## 11. Testing Strategy
*   **Unit Tests (Rust):**
    *   `test_ev_calc`: Verify math accuracy for arbitrage spreads.
    *   `test_staleness`: Ensure signals > 5% candle time are rejected.
*   **Integration Tests:**
    *   **Mock Oracle:** Feed pre-recorded CSV data into the Oracle module. Verify correct `TradeIntent` generation.
*   **Fork Testing (Crucial):**
    *   Use `anvil` (Foundry) to fork Polygon Mainnet.
    *   Impersonate a whale wallet with USDC.
    *   Simulate `approve` CTF and `fillOrders` on the real Exchange contract bytecode.
    *   Simulate `redeemPositions` to ensure gas estimation logic is correct.