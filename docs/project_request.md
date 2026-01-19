# Project Manifest: Bankai Terminal 

**Type:** High-Frequency Quantitative Trading System
**Target Ecosystem:** Polymarket (Execution) + Allora Network (Signal) + Binance (Oracle)
**Core Objective:** Aggressive capital compounding ($50 → $5,000+) via high-precision, fee-aware arbitrage.

## 1. Project Context & Philosophy
This is a **probability arbitrage engine** designed to exploit the latency between **prediction markets** (Polymarket) and **spot markets** (Binance).

### The "Algorithmic Battleground" Reality
We acknowledge that this trade is competitive. To survive against institutional bots:
1.  **No Signal, No Trade:** We strictly enforce alignment between **Allora (Trend)** and **Binance (Price)**.
2.  **Fee Evasion:** We prioritize **Maker Orders (Laddering)** to avoid the ~3% Taker Fee on 15m markets, only "Sniping" when the EV is overwhelming.
3.  **Speed Dominance:** We use **Rust** and **Direct Contract Interaction** to beat JavaScript/Python bots to the order book.

---

## 2. Technical Architecture & Stack
*   **Language:** **Rust** (Tokio/Tungstenite) for the Core Engine (Zero-Latency).
*   **Logic Layer:** **Python 3.11+** (via PyO3) for LLM integration and complex math.
*   **Hot Storage:** **Redis** (In-Memory) for live Order Book, Nonce management, and State.
*   **Cold Storage:** **TimescaleDB** for trade logs and "Regret" data.
*   **Infrastructure:** AWS c6gn.xlarge (ARM64) or Latitude.sh Bare Metal (Ashburn, VA).

---

## 3. The "Oracle" Module (Data Ingestion)
### A. Binance Feed ("The Truth")
*   **Stream:** Direct WebSocket (`aggTrade`) for BTC, ETH, SOL, XRP (and any asset supported by Allora).
*   **Logic:** Calculate **DFO (Distance From Open)** relative to the *exact* start time of the Polymarket candle.
*   **Volatility Monitor:** Calculate rolling 1-minute standard deviation. If `Vol > Threshold` AND `Trend == Neutral`, **HALT TRADING** (Whipsaw Protection).

### B. Allora Network ("The Signal")
*   **Data:** Ingest `Combined Inference` via RPC for all mapped topics.
*   **Temporal Validity System (The Stale Data Guard):**
    *   Formula: `Staleness_Ratio = (Current_Time - Signal_Timestamp) / Time_Remaining_In_Candle`
    *   **Hard Rule:** If `Staleness_Ratio > 5%`, the signal is invalid. **DROP MARKET.**

### C. Polymarket RTDS ("The Execution Venue")
*   **Stream:** `price_change` channel.
*   **Logic:** Maintain a local Order Book in Redis to bypass API latency and detect liquidity imbalances.

---

## 4. The "Velocity" Execution Algorithm
### A. Market Discovery (Gamma API)
*   **Poll:** `gamma-api.polymarket.com` every 30s.
*   **Filter:**
    1.  **Liquidity:** Market must have >$2,000 depth (to ensure exit).
    2.  **Structure:** Exclude "Augmented Negative Risk" (Markets with "Other" options).
    3.  **Fee Check:** Store dynamic `feeRateBps` per market ID.

### B. Execution Modes (Fee-Aware)
*   **Mode 1: The Ladder (Default - Maker)**
    *   *Goal:* Earn the spread, avoid the ~3% fee.
    *   *Context:* High Signal Score, Low/Medium Volatility.
    *   *Action:* Place multiple Limit Bids *below* market price.
*   **Mode 2: The Snipe (Aggressive - Taker)**
    *   *Goal:* Capture massive dislocation.
    *   *Context:* High Signal Score, **Extreme Binance Volatility**.
    *   *Calculation:* `Estimated_Edge > (Taker_Fee + Gas_Cost + 2%)`.
    *   *Action:* Execute **FOK (Fill or Kill)** immediately on the Ask.

### C. Compounding (Fractional Kelly)
*   **Formula:** `Bet_Size = Available_Capital * (0.25 * Kelly_Fraction)`
*   **Accounting:** `Available_Capital` must update instantly upon trade resolution (See Section 6).

---

## 5. "Dual-Rail" Infrastructure
### Rail A: The Relayer (Primary)
*   **Method:** Polymarket `POST /order` API (Batching enabled).
*   **Role:** Standard execution to save gas.

### Rail B: The Direct Contract (Failover)
*   **Method:** Direct `eth_sendRawTransaction` to `CTFExchange`.
*   **Trigger:** Relayer Latency > 500ms or Error.
*   **Nonce Manager:** Local Redis Mutex to prevent nonce gaps.

---

## 6. Resolution & Accounting (CRUCIAL)
*The engine must recycle capital immediately to maximize the "Compounding Velocity."*
1.  **Resolution Listener:**
    *   Listen for Polygon Chain Events (`ConditionResolution`).
    *   **Action:** Immediately call `redeemPositions` on the Gnosis CTF Contract.
2.  **Crash Recovery:**
    *   **Startup Routine:** Query the Blockchain for Open Orders and Active Balances. Re-populate Redis State. *Never assume an empty state on startup.*

---

## 7. The "Meta-Brain" (Self-Learning)
*   **Cycle:** Every 6 hours.
*   **Input:** Trade Logs (Win/Loss vs. Fee Paid vs. Allora Confidence).
*   **Analysis:**
    *   "Did we pay too many fees in 'Snipe' mode?"
    *   "Did 'Ladder' orders get left behind?"
*   **Action:** LLM adjusts `Snipe_Threshold` and `Kelly_Fraction` in `config.json` without restarting the bot.

---

## 8. Security & Health
1.  **Key Management:** Use an Encrypted Keystore (AES-256) loaded into memory at runtime.
2.  **Kill Switch:** If `Latency > 100ms`, `Clock_Drift > 10ms`, or `Consecutive_Losses > 5`: **Cancel All & Halt**.

---

## 9. Design & UI ("The HUD")
*   **Top:** System Health & Clock Sync status.
*   **Middle:** Active Markets.
    *   `SOL 15m | Mode: LADDER | Fee: 3.1% | EV (Net): +4.2% | Allora: UP`
*   **Bottom:** Financials.
    *   `Bankroll: $152.40`
    *   `Avg Compound Rate: +4.2%`
    *   `Projected Monthly: +120%`