# Implementation Plan

## Phase 1: Foundation, Config, and Safety
- [X] Step 1: Project Initialization & Environment Setup
  - **Task**: Initialize the Rust workspace and project structure. Set up Docker Compose for Redis (Hot Storage) and TimescaleDB (Cold Storage). Define `Cargo.toml` with essential dependencies (`tokio`, `tracing`, `serde`, `sqlx`, `redis`). Create the basic directory structure for Python scripts (`python/`) and ABI files (`abi/`). Implement basic structured logging setup in `main.rs`.
  - **Files**:
    - `Cargo.toml`: Workspace definition and dependencies.
    - `docker-compose.yml`: Redis and TimescaleDB service definitions.
    - `src/main.rs`: Entry point with `tracing-subscriber` setup.
    - `.env.example`: Template for environment variables.
  - **Step Dependencies**: None
  - **User Instructions**: Install Rust, Docker, and Python 3.11. Run `cargo build` to verify the environment.

- [X] Step 2: Configuration Module with Hot Reload
  - **Task**: Implement the `config` module. Define the configuration schema (API keys, endpoints, trading thresholds, strategy parameters) in a struct. Implement loading from a canonical `config/config.json`. Add a thread/task to watch this file for changes and trigger a hot reload of parameters (atomic swap of `Arc<Config>`).
  - **Files**:
    - `src/config/mod.rs`: Configuration loading and watching logic.
    - `config/config.json`: Default configuration file.
    - `src/error.rs`: Global error types.
  - **Step Dependencies**: Step 1
  - **User Instructions**: Populate `config/config.json` with initial API endpoints and threshold values.

- [X] Step 3: Security & Key Management
  - **Task**: Implement the `security` module. Create a CLI utility (`src/bin/keytool.rs`) to encrypt secrets (Private Keys, API Tokens) using AES-256-GCM into a `secrets.enc` file. Implement the runtime logic to prompt for a password, decrypt keys into memory using the `secrecy` crate, and ensure they are dropped on exit.
  - **Files**:
    - `src/security/mod.rs`: AES-256-GCM encryption/decryption logic.
    - `src/bin/keytool.rs`: CLI utility for managing the keystore.
  - **Step Dependencies**: Step 1
  - **User Instructions**: Run `cargo run --bin keytool` to generate `secrets.enc` with your credentials.

- [X] Step 4: Safety Monitors (Kill Switch & Clock Sync)
  - **Task**: Implement the `Health` monitor. Add checks for System Clock Drift (using NTP/Chrony checks or an external time API comparison). Implement the "Kill Switch" logic that monitors global state flags (`Latency`, `Consecutive_Losses`). If thresholds are breached, set a global `HALT` flag that the Engine respects.
  - **Files**:
    - `src/telemetry/health.rs`: Clock drift and system health checks.
    - `src/engine/risk.rs`: Kill switch state management.
  - **Step Dependencies**: Step 2

## Phase 2: Storage and Observability
- [X] Step 5: Storage Layer (Redis & TimescaleDB)
  - **Task**: Implement `RedisManager` for hot state (order books, nonces, volatility cache) and `DatabaseManager` for persistent storage. Create SQL migrations for `trade_logs` (execution details) and `regret_metrics` (for meta-brain).
  - **Files**:
    - `src/storage/redis.rs`: Redis connection pool and ZSET/HASH helpers.
    - `src/storage/database.rs`: `sqlx` connection pool for TimescaleDB.
    - `migrations/20240101000000_init.sql`: SQL schema definition.
  - **Step Dependencies**: Step 1
  - **User Instructions**: Install `sqlx-cli` and run `sqlx migrate run`.

- [X] Step 6: Metrics and Structured Logging
  - **Task**: enhance telemetry. Implement a metrics module to track critical KPIs: `latency_ms`, `order_fill_rate`, `rail_failover_count`. Ensure all logs include correlation IDs (e.g., `trace_id` for a trade lifecycle).
  - **Files**:
    - `src/telemetry/metrics.rs`: Metric definitions and recording (e.g., using `metrics` crate).
    - `src/telemetry/logging.rs`: Structured logging extensions.
  - **Step Dependencies**: Step 1

## Phase 3: Python Integration & Logic Layer
- [X] Step 7: Python Embedding via PyO3
  - **Task**: Configure `build.rs` to link Python. Create `src/engine/python_host.rs` to initialize the Python interpreter. Define the Rust traits that expose necessary data to Python and the Python interface for strategy functions.
  - **Files**:
    - `build.rs`: PyO3 linker config.
    - `src/engine/python_host.rs`: Python VM wrapper.
    - `python/requirements.txt`: Dependencies (numpy, pandas).
  - **Step Dependencies**: Step 1
  - **User Instructions**: Run `pip install -r python/requirements.txt`.

- [X] Step 8: Core Logic Implementation (Python)
  - **Task**: Implement `python/strategy.py`. Write `calculate_kelly` (Fractional Kelly sizing), `validate_signal` (Allora vs Binance alignment), and `calculate_staleness` (time decay logic). Include unit tests in Python.
  - **Files**:
    - `python/strategy.py`: Math and logic functions.
    - `python/tests/test_strategy.py`: Pytest suite.
  - **Step Dependencies**: Step 7
  - **User Instructions**: Run `pytest` in the `python/` directory to verify logic.

## Phase 4: Oracle Module (Data Ingestion)
- [X] Step 9: Binance Oracle ("The Truth")
  - **Task**: Implement `src/oracle/binance.rs`. Connect to WSS streams for `aggTrade` (volatility calc) and `bookTicker` (immediate price). Implement the Rolling 1-minute Volatility calculation and Distance From Open (DFO) logic aligned to the Polymarket candle start time.
  - **Files**:
    - `src/oracle/binance.rs`: WebSocket handler.
    - `src/engine/types.rs`: Shared market data types.
  - **Step Dependencies**: Step 5

- [X] Step 10: Allora Oracle ("The Signal")
  - **Task**: Implement `src/oracle/allora.rs`. Poll the Allora RPC for `Combined Inference`. Implement the Staleness Ratio calculation: `(Now - Signal_Timestamp) / Candle_Time_Remaining`. Reject signals where ratio > 5%.
  - **Files**:
    - `src/oracle/allora.rs`: HTTP polling client.
    - `src/engine/risk.rs`: Add staleness check functions.
  - **Step Dependencies**: Step 5

- [X] Step 11: Polymarket Discovery (Gamma API)
  - **Task**: Implement `src/oracle/polymarket_discovery.rs`. Poll Gamma API every 30s (with jitter). Filter markets: Liquidity > $2k, exclude "Augmented Negative Risk". Fetch and store `feeRateBps` and `minTickSize` in Redis.
  - **Files**:
    - `src/oracle/polymarket_discovery.rs`: Gamma API client.
    - `src/storage/redis.rs`: methods for market metadata.
  - **Step Dependencies**: Step 5

- [X] Step 12: Polymarket RTDS (Order Book)
  - **Task**: Implement `src/oracle/polymarket_rtds.rs`. Connect to `wss://ws-live-data.polymarket.com`. Fetch initial snapshot via REST, then apply `price_change` updates to maintain a local ZSET order book in Redis.
  - **Files**:
    - `src/oracle/polymarket_rtds.rs`: WebSocket and Snapshot client.
    - `src/storage/orderbook.rs`: Redis ZSET maintenance logic.
  - **Step Dependencies**: Step 5

## Phase 5: Velocity Execution Engine
- [X] Step 13: Core Engine Event Loop
  - **Task**: Create `src/engine/core.rs`. Implement the main `tokio::select!` loop that consumes messages from all oracles. Integrate the `Risk` module (Step 4) to halt trading if volatility is high or the kill switch is active.
  - **Files**:
    - `src/engine/core.rs`: Main orchestrator loop.
    - `src/main.rs`: Wire up Oracles to Engine.
  - **Step Dependencies**: Step 9, Step 10, Step 12

- [X] Step 14: Trade Opportunity Analysis
  - **Task**: Implement `src/engine/analysis.rs`. Calculate Implied Probabilities vs True Probabilities (Binance + Allora). Compute `Edge`. Determine `TradeIntent` (Snipe vs Ladder) based on thresholds and fees. Enforce: `Edge > (Taker_Fee + Gas + 2%)` for Snipes.
  - **Files**:
    - `src/engine/analysis.rs`: EV and decision logic.
    - `src/engine/types.rs`: Add `TradeIntent` struct.
  - **Step Dependencies**: Step 8, Step 13

- [X] Step 15: Order Lifecycle Management
  - **Task**: Implement logic to track active orders. For "Ladder" orders, enforce a cancellation policy (e.g., cancel within 250ms if conditions degrade). Handle FOK emulation if native FOK isn't reliable (Immediate-Or-Cancel or check fill status immediately).
  - **Files**:
    - `src/engine/orders.rs`: Order tracking and lifecycle logic.
  - **Step Dependencies**: Step 13

## Phase 6: Dual-Rail Execution
- [X] Step 16: Signer & Nonce Management
  - **Task**: Implement `src/execution/signer.rs` for EIP-712 signing. Implement `src/execution/nonce.rs` for atomic nonce management using Redis Lua scripts/Mutex. Support "Replace-By-Fee" logic (tracking nonces to bump gas if stuck).
  - **Files**:
    - `src/execution/signer.rs`: Ethers-rs signing logic.
    - `src/execution/nonce.rs`: Redis nonce manager.
  - **Step Dependencies**: Step 3, Step 5

- [X] Step 17: Rail A - The Relayer
  - **Task**: Implement `src/execution/relayer.rs` to POST orders to the Polymarket Relayer API. Implement latency tracking and specific error handling (detecting congestion/5xx errors) to trigger failover.
  - **Files**:
    - `src/execution/relayer.rs`: HTTP Relayer client.
  - **Step Dependencies**: Step 16

- [X] Step 18: Rail B - Direct Contract
  - **Task**: Implement `src/execution/direct.rs`. Load `CTFExchange` ABI. Construct raw transactions calling `fillOrders`. Use a trusted RPC provider (e.g., Alchemy) config. Support Flashbots/Private RPC headers if configured.
  - **Files**:
    - `src/execution/direct.rs`: Direct blockchain interaction.
    - `abi/CTFExchange.json`: Contract ABI.
  - **Step Dependencies**: Step 16

- [X] Step 19: Executor Orchestrator
  - **Task**: Create `src/execution/orchestrator.rs`. Listen for `TradeIntent`. Attempt Rail A; if it times out (>500ms) or fails, execute Rail B. Log the execution path and outcome to TimescaleDB.
  - **Files**:
    - `src/execution/orchestrator.rs`: Routing logic.
    - `src/storage/database.rs`: Trade logging method.
  - **Step Dependencies**: Step 17, Step 18

## Phase 7: Resolution, Recovery, and Accounting
- [X] Step 20: Resolution Listener & Redemption
  - **Task**: Implement `src/accounting/redemption.rs`. Listen for `ConditionResolution` events via WebSocket/Log polling. When resolved, call `redeemPositions` on the Gnosis CTF contract. Update `sys:bankroll:usdc` in Redis immediately upon success.
  - **Files**:
    - `src/accounting/redemption.rs`: Event listener and redemption logic.
    - `abi/ConditionalTokens.json`: Contract ABI.
  - **Step Dependencies**: Step 18

- [X] Step 21: Startup Recovery
  - **Task**: Implement `src/accounting/recovery.rs`. On application boot, query the blockchain for Open Orders and current Token Balances. Reconcile this data with Redis to ensure the local state matches the chain state.
  - **Files**:
    - `src/accounting/recovery.rs`: Recovery logic.
    - `src/main.rs`: Integrate recovery into startup sequence.
  - **Step Dependencies**: Step 5, Step 18

## Phase 8: Meta-Brain and HUD
- [ ] Step 22: Meta-Brain (Self-Learning Script)
  - **Task**: Implement `python/meta_brain.py`. Query `trade_logs` from TimescaleDB. Analyze "Regret" (optimal decision vs actual). Adjust `Snipe_Threshold` and `Kelly_Fraction` in `config/strategies.json`. Ensure the Rust engine hot-reloads these changes.
  - **Files**:
    - `python/meta_brain.py`: Analysis script.
    - `config/strategies.json`: Dynamic strategy config.
  - **Step Dependencies**: Step 6, Step 2

- [ ] Step 23: TUI Implementation (Ratatui)
  - **Task**: Implement the Terminal User Interface. Display "System Health" (Clock, Latency), "Active Markets" (with EV, Fee, Mode), and "Financials" (Bankroll, PnL). Ensure TUI rendering does not block the trading engine.
  - **Files**:
    - `src/ui/mod.rs`: TUI app loop.
    - `src/ui/widgets.rs`: Custom widget rendering.
  - **Step Dependencies**: Step 13

## Phase 9: Testing and Validation
- [ ] Step 24: Unit Testing (Rust & Python)
  - **Task**: Write unit tests for critical paths: Staleness calculations, Volatility halt triggers, Kelly sizing math (Python side), and EV calculations (Rust side).
  - **Files**:
    - `src/engine/risk.rs`: Tests module.
    - `src/engine/analysis.rs`: Tests module.
  - **Step Dependencies**: Step 8, Step 14

- [ ] Step 25: Simulation & Integration Tests
  - **Task**: Create `tests/simulation_test.rs`. Mock the Oracle inputs (feed a predefined CSV of prices/signals). Verify that `TradeIntent` is generated correctly and routed to the correct Rail. Verify Staleness Guard rejects old signals.
  - **Files**:
    - `tests/simulation_test.rs`: Integration test.
  - **Step Dependencies**: Step 19

- [ ] Step 26: Fork Testing (Polygon)
  - **Task**: Create `tests/fork_redemption.rs`. Use `foundry` or `anvil` to fork Polygon Mainnet. Simulate a market resolution and verify that `redeemPositions` correctly claims funds and gas estimation is accurate.
  - **Files**:
    - `tests/fork_redemption.rs`: Fork test logic.
  - **Step Dependencies**: Step 20
