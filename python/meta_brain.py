#!/usr/bin/env python3
# /**
#  * @purpose
#  * Meta-brain optimization loop for adjusting strategy parameters from trade logs.
#  *
#  * @dependencies
#  * - argparse: CLI configuration
#  * - json: strategy file parsing
#  * - psycopg: TimescaleDB access
#  *
#  * @notes
#  * - Writes updates to config/strategies.json atomically with version bumps.
#  */
from __future__ import annotations

import argparse
import json
import logging
import os
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable

try:
    import psycopg
except ImportError as exc:  # pragma: no cover - defensive message for operators.
    raise SystemExit(
        "psycopg is required for meta_brain; run `pip install -r python/requirements.txt`."
    ) from exc


DEFAULT_STRATEGY_PATH = Path("config/strategies.json")
DEFAULT_CONFIG_PATH = Path("config/config.json")
DEFAULT_LOOKBACK_HOURS = 6
DEFAULT_INTERVAL_HOURS = 6
DEFAULT_MIN_SAMPLES = 10

MIN_KELLY_FRACTION = 0.05
MAX_KELLY_FRACTION = 0.5
MIN_SNIPE_BPS = 50.0
MAX_SNIPE_BPS = 2000.0

STRATEGY_HEADER = """/**
 * @purpose
 * Dynamic strategy parameters managed by the meta-brain optimizer.
 *
 * @dependencies
 * - None
 *
 * @notes
 * - Values override config/config.json and hot-reload at runtime.
 */
"""


@dataclass
class StrategySettings:
    """Strategy parameters that can be tuned by the meta-brain."""

    kelly_fraction: float
    snipe_min_edge_bps: float
    spread_offset_bps: float


@dataclass
class StrategyDocument:
    """Strategy document wrapper with optional metadata."""

    version: int
    updated_at: str | None
    strategy: StrategySettings


@dataclass
class PerformanceStats:
    """Aggregate metrics for strategy performance analysis."""

    count: int = 0
    expected_total: float = 0.0
    net_total: float = 0.0
    regret_total: float = 0.0

    def add(self, expected_ev: float, net_pnl: float) -> None:
        """Add a trade entry to the aggregate stats."""

        self.count += 1
        self.expected_total += expected_ev
        self.net_total += net_pnl
        self.regret_total += expected_ev - net_pnl

    def avg_expected(self) -> float:
        """Return average expected EV."""

        return self.expected_total / self.count if self.count else 0.0

    def avg_net(self) -> float:
        """Return average net PnL."""

        return self.net_total / self.count if self.count else 0.0

    def avg_regret(self) -> float:
        """Return average regret value."""

        return self.regret_total / self.count if self.count else 0.0

    def regret_ratio(self) -> float:
        """Return regret ratio relative to expected EV magnitude."""

        expected = self.avg_expected()
        denom = max(abs(expected), 1e-6)
        return self.avg_regret() / denom


def strip_jsdoc_header(contents: str) -> str:
    """Strip a leading JSDoc header block from a file."""

    start_index = None
    for idx, char in enumerate(contents):
        if not char.isspace():
            start_index = idx
            break

    if start_index is None:
        return contents

    if not contents[start_index:].startswith("/**"):
        return contents

    header_start = start_index + 3
    header_end = contents[header_start:].find("*/")
    if header_end == -1:
        raise ValueError("config header missing closing marker")
    content_start = header_start + header_end + 2
    return contents[content_start:]


def parse_strategy_payload(payload: dict) -> StrategyDocument:
    """Parse strategy JSON into a StrategyDocument."""

    if "strategy" in payload:
        strategy_payload = payload.get("strategy") or {}
        version = int(payload.get("version", 0))
        updated_at = payload.get("updated_at")
    else:
        strategy_payload = payload
        version = 0
        updated_at = None

    kelly_fraction = float(strategy_payload["kelly_fraction"])
    snipe_min_edge_bps = float(strategy_payload["snipe_min_edge_bps"])
    spread_offset_bps = float(strategy_payload["spread_offset_bps"])
    strategy = StrategySettings(
        kelly_fraction=kelly_fraction,
        snipe_min_edge_bps=snipe_min_edge_bps,
        spread_offset_bps=spread_offset_bps,
    )
    validate_strategy(strategy)
    return StrategyDocument(version=version, updated_at=updated_at, strategy=strategy)


def load_strategy_document(path: Path, fallback_path: Path | None) -> StrategyDocument:
    """Load the strategy document from disk."""

    if not path.exists():
        if fallback_path is None:
            raise FileNotFoundError(f"strategy config not found: {path}")
        return load_strategy_from_config(fallback_path)

    raw = path.read_text(encoding="utf-8")
    stripped = strip_jsdoc_header(raw)
    payload = json.loads(stripped)
    if not isinstance(payload, dict):
        raise ValueError("strategy config must be a JSON object")
    return parse_strategy_payload(payload)


def load_strategy_from_config(path: Path) -> StrategyDocument:
    """Load strategy settings from the main config when overrides are missing."""

    raw = path.read_text(encoding="utf-8")
    stripped = strip_jsdoc_header(raw)
    payload = json.loads(stripped)
    if not isinstance(payload, dict):
        raise ValueError("config must be a JSON object")
    strategy_payload = payload.get("strategy")
    if not isinstance(strategy_payload, dict):
        raise ValueError("config.strategy missing or invalid")
    return parse_strategy_payload(strategy_payload)


def write_strategy_document(path: Path, document: StrategyDocument) -> None:
    """Write the strategy document to disk atomically."""

    payload = {
        "version": document.version,
        "updated_at": document.updated_at,
        "strategy": {
            "kelly_fraction": document.strategy.kelly_fraction,
            "snipe_min_edge_bps": document.strategy.snipe_min_edge_bps,
            "spread_offset_bps": document.strategy.spread_offset_bps,
        },
    }
    serialized = STRATEGY_HEADER + json.dumps(payload, indent=2) + "\n"
    tmp_path = path.with_name(f"{path.name}.tmp")
    tmp_path.write_text(serialized, encoding="utf-8")
    os.replace(tmp_path, path)


def validate_strategy(strategy: StrategySettings) -> None:
    """Ensure strategy parameters are within safe bounds."""

    if not MIN_KELLY_FRACTION <= strategy.kelly_fraction <= MAX_KELLY_FRACTION:
        raise ValueError("kelly_fraction out of bounds")
    if strategy.snipe_min_edge_bps < 0.0:
        raise ValueError("snipe_min_edge_bps must be non-negative")
    if strategy.spread_offset_bps < 0.0:
        raise ValueError("spread_offset_bps must be non-negative")


def clamp(value: float, min_value: float, max_value: float) -> float:
    """Clamp a float between min and max."""

    return max(min_value, min(max_value, value))


def fetch_trade_logs(
    conn: psycopg.Connection, lookback_hours: int
) -> Iterable[tuple[str, float, float | None, float]]:
    """Fetch recent trade logs for the given lookback window."""

    query = """
        SELECT mode, expected_ev, actual_pnl, fees_paid
        FROM trade_logs
        WHERE time >= NOW() - (%s * interval '1 hour')
    """
    with conn.cursor() as cursor:
        cursor.execute(query, (lookback_hours,))
        return cursor.fetchall()


def compute_stats(
    rows: Iterable[tuple[str, float, float | None, float]]
) -> tuple[dict[str, PerformanceStats], PerformanceStats, int]:
    """Compute performance metrics grouped by trade mode."""

    by_mode: dict[str, PerformanceStats] = {}
    overall = PerformanceStats()
    skipped = 0

    for mode, expected_ev, actual_pnl, fees_paid in rows:
        if actual_pnl is None:
            skipped += 1
            continue
        net_pnl = float(actual_pnl) - float(fees_paid)
        stats = by_mode.setdefault(str(mode).upper(), PerformanceStats())
        stats.add(float(expected_ev), net_pnl)
        overall.add(float(expected_ev), net_pnl)

    return by_mode, overall, skipped


def adjust_strategy(
    current: StrategySettings,
    by_mode: dict[str, PerformanceStats],
    overall: PerformanceStats,
    min_samples: int,
) -> tuple[StrategySettings, float, float]:
    """Adjust strategy settings based on performance stats."""

    snipe_delta = 0.0
    kelly_delta = 0.0

    snipe_stats = by_mode.get("SNIPE")
    if snipe_stats and snipe_stats.count >= min_samples:
        ratio = snipe_stats.regret_ratio()
        if ratio > 0.1:
            snipe_delta = min(50.0, 10.0 + ratio * 20.0)
        elif ratio < -0.1:
            snipe_delta = -min(25.0, 5.0 + abs(ratio) * 10.0)

    if overall.count >= min_samples:
        ratio = overall.regret_ratio()
        if overall.avg_net() < 0.0 or ratio > 0.1:
            kelly_delta = -0.02 if ratio <= 0.25 else -0.05
        elif overall.avg_net() > 0.0 and ratio < -0.1:
            kelly_delta = 0.02

    new_kelly = clamp(
        current.kelly_fraction + kelly_delta, MIN_KELLY_FRACTION, MAX_KELLY_FRACTION
    )
    new_snipe = clamp(
        current.snipe_min_edge_bps + snipe_delta, MIN_SNIPE_BPS, MAX_SNIPE_BPS
    )

    updated = StrategySettings(
        kelly_fraction=new_kelly,
        snipe_min_edge_bps=new_snipe,
        spread_offset_bps=current.spread_offset_bps,
    )
    return updated, kelly_delta, snipe_delta


def run_once(args: argparse.Namespace) -> None:
    """Run a single optimization cycle."""

    strategy_path = Path(args.strategies_path)
    config_path = Path(args.config_path)
    document = load_strategy_document(strategy_path, config_path)

    if not args.db_url:
        raise ValueError("TIMESCALE_URL missing; set env or pass --db-url")

    with psycopg.connect(args.db_url, connect_timeout=5) as conn:
        rows = fetch_trade_logs(conn, args.lookback_hours)

    by_mode, overall, skipped = compute_stats(rows)

    if overall.count < args.min_samples:
        logging.info(
            "insufficient samples for optimization (count=%s, skipped=%s)",
            overall.count,
            skipped,
        )
        return

    updated_strategy, kelly_delta, snipe_delta = adjust_strategy(
        document.strategy, by_mode, overall, args.min_samples
    )

    if (
        updated_strategy.kelly_fraction == document.strategy.kelly_fraction
        and updated_strategy.snipe_min_edge_bps == document.strategy.snipe_min_edge_bps
    ):
        logging.info("no strategy changes required (skipped=%s)", skipped)
        return

    version = document.version + 1
    updated_at = datetime.now(timezone.utc).isoformat()
    updated_doc = StrategyDocument(
        version=version, updated_at=updated_at, strategy=updated_strategy
    )
    write_strategy_document(strategy_path, updated_doc)

    logging.info(
        "strategy updated: kelly_delta=%.4f, snipe_delta=%.2f bps, version=%s",
        kelly_delta,
        snipe_delta,
        version,
    )


def parse_args() -> argparse.Namespace:
    """Parse command line arguments."""

    parser = argparse.ArgumentParser(description="Run meta-brain optimization loop.")
    parser.add_argument(
        "--db-url",
        default=os.getenv("TIMESCALE_URL"),
        help="TimescaleDB connection URL (default: TIMESCALE_URL env var).",
    )
    parser.add_argument(
        "--strategies-path",
        default=str(DEFAULT_STRATEGY_PATH),
        help="Path to strategies.json.",
    )
    parser.add_argument(
        "--config-path",
        default=str(DEFAULT_CONFIG_PATH),
        help="Fallback path to config.json if strategies.json is missing.",
    )
    parser.add_argument(
        "--lookback-hours",
        type=int,
        default=DEFAULT_LOOKBACK_HOURS,
        help="Lookback window for trade logs.",
    )
    parser.add_argument(
        "--interval-hours",
        type=int,
        default=DEFAULT_INTERVAL_HOURS,
        help="Interval between optimization cycles when looping.",
    )
    parser.add_argument(
        "--min-samples",
        type=int,
        default=DEFAULT_MIN_SAMPLES,
        help="Minimum sample size before adjusting parameters.",
    )
    parser.add_argument(
        "--once",
        action="store_true",
        help="Run a single optimization cycle and exit.",
    )
    return parser.parse_args()


def main() -> int:
    """Entrypoint for meta-brain optimization."""

    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s",
    )
    args = parse_args()

    while True:
        run_once(args)
        if args.once:
            break
        time.sleep(max(1, args.interval_hours) * 3600)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
