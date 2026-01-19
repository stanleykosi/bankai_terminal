#!/usr/bin/env python3
# /**
#  * @description
#  * Core strategy math helpers for Kelly sizing, signal validation, and staleness.
#  *
#  * @dependencies
#  * - json: inference payload parsing
#  *
#  * @notes
#  * - Functions are pure and deterministic for easy unit testing.
#  */
from __future__ import annotations

import json
from dataclasses import dataclass


MIN_ALIGNMENT_PCT = 0.001


@dataclass(frozen=True)
class InferencePayload:
    combined_value: float


def _parse_inference_payload(raw_json: str) -> InferencePayload:
    try:
        data = json.loads(raw_json)
    except json.JSONDecodeError as exc:
        raise ValueError("invalid inference JSON") from exc

    if not isinstance(data, dict):
        raise ValueError("inference JSON must be an object")

    raw_value = data.get("combined_value")
    if raw_value is None:
        network = data.get("network_inferences")
        if isinstance(network, dict):
            raw_value = network.get("combined_value")
    if raw_value is None:
        raise ValueError("combined_value missing from inference JSON")

    try:
        combined_value = float(raw_value)
    except (TypeError, ValueError) as exc:
        raise ValueError("combined_value must be numeric") from exc

    return InferencePayload(combined_value=combined_value)


def calculate_kelly(win_prob: float, odds: float) -> float:
    """
    Fractional Kelly sizing for binary outcomes.

    Args:
        win_prob: Probability of winning (0.0 - 1.0).
        odds: Decimal odds / payout ratio (e.g., 1.0 for even odds).

    Returns:
        Kelly fraction (>= 0.0).
    """
    if win_prob <= 0.0 or win_prob >= 1.0:
        return 0.0
    if odds <= 1.0:
        return 0.0

    payout = odds - 1.0
    loss_prob = 1.0 - win_prob
    kelly = (win_prob * payout - loss_prob) / payout
    return max(0.0, min(kelly, 1.0))


def validate_signal(inference_json: str, current_price: float) -> bool:
    """
    Validate that Allora inference aligns with price direction.

    Args:
        inference_json: Raw JSON string with "combined_value".
        current_price: Current Binance price.

    Returns:
        True if inference direction aligns with price direction.
    """
    if current_price <= 0.0:
        raise ValueError("current_price must be positive")

    payload = _parse_inference_payload(inference_json)
    combined_value = payload.combined_value

    if combined_value <= 0.0:
        return False

    delta = (combined_value - current_price) / current_price
    return abs(delta) >= MIN_ALIGNMENT_PCT


def calculate_staleness(
    signal_timestamp_ms: int,
    candle_end_timestamp_ms: int,
    now_timestamp_ms: int,
) -> float:
    """
    Calculate staleness ratio for a signal within a candle.

    Ratio = (now - signal_timestamp) / (candle_end - now)
    """
    if candle_end_timestamp_ms <= now_timestamp_ms:
        raise ValueError("candle_end_timestamp_ms must be after now_timestamp_ms")

    elapsed = now_timestamp_ms - signal_timestamp_ms
    if elapsed < 0:
        elapsed = 0

    remaining = candle_end_timestamp_ms - now_timestamp_ms
    if remaining <= 0:
        return 1.0

    return elapsed / remaining
