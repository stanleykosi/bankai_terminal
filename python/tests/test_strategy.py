# /**
#  * @description
#  * Unit tests for strategy helpers (Kelly sizing, signal validation, staleness).
#  *
#  * @dependencies
#  * - pytest: test runner
#  *
#  * @notes
#  * - Use simple numeric cases to validate edge handling.
#  */
from __future__ import annotations

import json

import pathlib
import sys

import pytest

STRATEGY_DIR = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(STRATEGY_DIR))

from strategy import calculate_kelly, calculate_staleness, validate_signal


def test_calculate_kelly_even_odds() -> None:
    kelly = calculate_kelly(0.6, 2.0)
    assert kelly == pytest.approx(0.2)


def test_calculate_kelly_invalid_inputs() -> None:
    assert calculate_kelly(0.0, 2.0) == 0.0
    assert calculate_kelly(1.0, 2.0) == 0.0
    assert calculate_kelly(0.5, 1.0) == 0.0


def test_validate_signal_threshold() -> None:
    payload = {"combined_value": 101.0}
    assert validate_signal(json.dumps(payload), 100.0) is True

    payload = {"combined_value": 100.05}
    assert validate_signal(json.dumps(payload), 100.0) is False


def test_validate_signal_nested_payload() -> None:
    payload = {"network_inferences": {"combined_value": "150.0"}}
    assert validate_signal(json.dumps(payload), 100.0) is True


def test_validate_signal_invalid_payload() -> None:
    with pytest.raises(ValueError):
        validate_signal("not-json", 100.0)

    with pytest.raises(ValueError):
        validate_signal(json.dumps({"foo": "bar"}), 100.0)


def test_calculate_staleness_ratio() -> None:
    ratio = calculate_staleness(900, 2000, 1000)
    assert ratio == pytest.approx(0.1)


def test_calculate_staleness_future_signal() -> None:
    ratio = calculate_staleness(1200, 2000, 1000)
    assert ratio == pytest.approx(0.0)


def test_calculate_staleness_invalid_window() -> None:
    with pytest.raises(ValueError):
        calculate_staleness(1000, 900, 1000)
