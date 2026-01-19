# Project Rules: Bankai Terminal

These rules define the authoritative standards for building, operating, and evolving the Bankai Terminal high-frequency quantitative trading system. They are mandatory for all contributors and all deployments.

## 1. Core Principles (Non-Negotiable)
- No signal, no trade: orders must require positive alignment between Allora trend and Binance price signal.
- Fee evasion first: prefer maker laddering; taker snipes only when net EV is clearly above fees and gas.
- Speed matters: Rust is the source of truth for hot-path execution; Python is for non-latency-critical logic.
- Capital compounding is the goal: resolution and accounting must recycle capital immediately.
- Safety over greed: kill switches and volatility protection always override trading logic.

## 2. Architecture and Module Boundaries
- Enforce module separation:
  - Oracle (Binance + Allora) ingestion and signal validity.
  - Polymarket RTDS order book and market state (Redis hot cache).
  - Velocity execution (ladder/snipe strategies).
  - Dual-rail execution (relayer and direct contract failover).
  - Resolution + accounting (redemptions and bankroll).
  - Meta-brain (6h optimization loop).
- Hot path (Rust) must not block on Python or network I/O that is not explicitly required.
- Python logic must be exposed via PyO3 with clear, versioned interfaces and strict input validation.
- Every module must define:
  - Inputs/outputs (message schemas).
  - Error classes and retry semantics.
  - Latency budgets and timeout thresholds.
- All cross-module messages must be time-stamped with monotonic and wall-clock time.

## 3. Data Ingestion Rules
### Binance Oracle
- Subscribe to `aggTrade` for all supported assets (BTC, ETH, SOL, XRP, plus Allora-mapped assets).
- Compute DFO relative to the exact Polymarket candle open time; candle alignment must be deterministic.
- Maintain rolling 1-minute volatility standard deviation per asset.
- If `Vol > Threshold` and `Trend == Neutral`, halt trading for that market.

### Allora Signal
- Ingest `Combined Inference` via RPC for all mapped topics.
- Enforce staleness guard:
  - `Staleness_Ratio = (Current_Time - Signal_Timestamp) / Time_Remaining_In_Candle`
  - If `Staleness_Ratio > 0.05`, drop the market immediately and clear open intent.

### Polymarket RTDS
- Consume `price_change` channel and maintain a full local order book in Redis.
- Redis is authoritative for the local order book; API reads are only for validation and recovery.

## 4. Execution Rules (Velocity Engine)
- Market discovery via Gamma API every 30s with cache and jitter to avoid synchronized polling.
- Market eligibility rules:
  - Liquidity depth > $2,000.
  - Exclude "Augmented Negative Risk" markets (any with "Other" option).
  - Record and update `feeRateBps` per market ID.

### Mode 1: Ladder (Maker, Default)
- Preconditions: high signal score, low/medium volatility, valid staleness ratio.
- Place multiple limit bids below market price; avoid crossing spread.
- Orders must be cancellable within 250ms if conditions degrade.

### Mode 2: Snipe (Taker, Exceptional)
- Preconditions: high signal score and extreme Binance volatility.
- Must satisfy: `Estimated_Edge > (Taker_Fee + Gas_Cost + 2%)`.
- Execute FOK (fill or kill) on the ask; never partial-fill unless explicitly approved.

### Position Sizing (Fractional Kelly)
- `Bet_Size = Available_Capital * (0.25 * Kelly_Fraction)`
- `Available_Capital` must update immediately upon resolution events.
- Kelly inputs must be bounded and validated to avoid runaway sizing.

## 5. Dual-Rail Execution Rules
- Rail A (Relayer):
  - Primary route via `POST /order` API with batching enabled.
- Rail B (Direct Contract):
  - Trigger if relayer latency > 500ms or error response.
  - Use `eth_sendRawTransaction` to `CTFExchange`.
  - Nonce manager must use Redis mutex to prevent gaps and races.
- On rail failover, log the incident and backoff before retrying.

## 6. Resolution and Accounting (Critical Path)
- Subscribe to Polygon chain events: `ConditionResolution`.
- Upon resolution, immediately call `redeemPositions` on the Gnosis CTF contract.
- Startup routine must reconcile:
  - On-chain open orders and active balances.
  - Redis state rehydration (order book, nonce, bankroll).
  - Never assume empty state on boot.

## 7. Meta-Brain (Self-Learning)
- Run every 6 hours without restarting the bot.
- Inputs: trade logs, fee costs, win/loss, Allora confidence.
- Outputs: adjust `Snipe_Threshold` and `Kelly_Fraction` in `config.json`.
- Config updates must be atomic, versioned, and validated before apply.

## 8. Security and Safety
- Key management must use AES-256 encrypted keystore loaded into memory at runtime only.
- Secrets must never be logged or written to disk in plaintext.
- Kill switch triggers:
  - `Latency > 100ms`, or `Clock_Drift > 10ms`, or `Consecutive_Losses > 5`.
  - Action: cancel all orders and halt trading immediately.
- Enforce rate limits, request signing validation, and strict input sanitization.

## 9. Time, Clock, and Latency Discipline
- All systems must maintain precise clock sync (NTP/chrony); drift > 10ms is a hard stop.
- Use monotonic clocks for latency measurement; wall-clock only for event correlation.
- All network calls must have explicit timeouts and retry policies.

## 10. Storage and Data Retention
- Redis is for hot state; data must be durable in TimescaleDB.
- Persist trade logs, fills, fees, latencies, signal inputs, and decision outcomes.
- Retain "regret" data for meta-brain analysis.
- Ensure deterministic replay is possible from stored data.

## 11. Code Quality and Standards
- Rust:
  - `rustfmt` and `clippy` are mandatory; no warnings on CI.
  - Avoid allocations and blocking in hot paths; pre-allocate buffers where possible.
  - Use `tokio` primitives; avoid `std::sync` locks in async contexts.
- Python:
  - Target Python 3.11+; use `ruff`/`black` or equivalent standard formatters.
  - All PyO3 bindings must include type hints and explicit error handling.
- All code must include:
  - Clear error messages with context and actionable metadata.
  - Metrics emission for critical branches (trade decision, order placement, failover).
- Configuration changes must be validated and checked in review.

## 12. Testing Requirements
- Unit tests for signal validity, volatility guard, ladder and snipe eligibility, and sizing.
- Integration tests for:
  - Binance + Allora + Polymarket ingestion flows.
  - Redis order book sync.
  - Dual-rail failover behavior.
- Simulation or replay tests for latency and kill-switch behavior.
- Must include tests for:
  - Staleness rule enforcement.
  - Volatility halt rule.
  - Immediate accounting after resolution.

## 13. Deployment and Operations
- Target infra: AWS c6gn.xlarge (ARM64) or Latitude.sh bare metal (Ashburn, VA).
- Deployments must be reproducible, scripted, and versioned.
- All deployments must include:
  - Config checksum validation.
  - Health checks and readiness gates.
  - Rollback procedure.
- Do not deploy if:
  - Clock drift or latency thresholds are violated.
  - Redis is unavailable or stale.

## 14. Collaboration and Version Control
- Use clear, scoped PRs with linked issues and explicit acceptance criteria.
- Require code review for:
  - Trading logic changes.
  - Risk/kill-switch logic.
  - On-chain interaction or nonce handling.
- Enforce branch protection and CI passing before merge.
- Document any changes to:
  - Signal ingestion rules.
  - Fee logic.
  - Execution thresholds.

## 15. Documentation Standards
- Keep a living architecture doc describing modules, data flow, and interfaces.
- Every public function must have a docstring and example if non-trivial.
- Update runbooks for:
  - Recovery and startup.
  - Incident response.
  - Kill switch triggers and overrides.

## 16. UI/HUD Requirements
- HUD must display:
  - System health and clock sync status.
  - Active markets with mode, fee, EV net, and Allora direction.
  - Bankroll and compounding metrics.
- UI refresh must not block or degrade trading engine latency.
- All HUD data must be sourced from authoritative state (Redis + TimescaleDB).

## 17. Compliance and Risk Guardrails
- Avoid trading markets without reliable oracle alignment or with stale data.
- Respect Polymarket and Binance API terms of service and rate limits.
- Every trade decision must be traceable to inputs (signal, price, fee, volatility).

## 18. Change Control
- Any change that affects execution or risk must include:
  - Rationale.
  - Expected impact on EV and fees.
  - Test evidence or simulation result.
- Emergency changes must be recorded retroactively in the change log.
