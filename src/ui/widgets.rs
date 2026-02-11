/**
 * @purpose
 * Ratatui widgets for the Bankai Terminal TUI dashboard.
 *
 * @dependencies
 * - ratatui: layout and widget primitives
 *
 * @notes
 * - Styling follows the "cyberpunk terminal" spec with double borders.
 */
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;
use serde_json::Value;

use super::{
    ActiveWindowRow, FinancialPanelData, HealthPanelData, MarketMode, MarketRow, PaperStatsData,
    PolymarketPanelData, StatusBarData, UiSnapshot,
};
use crate::engine::risk::HaltReason;

pub fn render_dashboard(frame: &mut Frame, snapshot: &UiSnapshot) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(frame.size());

    render_status_bar(frame, layout[0], &snapshot.status);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(layout[1]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(body[0]);
    render_window_bar(frame, left[0], &snapshot.active_windows);
    render_markets(frame, left[1], &snapshot.markets);
    render_activity_log(frame, left[2], &snapshot.activity_log);

    let right = if snapshot.no_money_mode {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(22),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(22),
                Constraint::Percentage(24),
            ])
            .split(body[1])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(24),
                Constraint::Percentage(20),
                Constraint::Percentage(26),
                Constraint::Percentage(30),
            ])
            .split(body[1])
    };

    render_system_panel(
        frame,
        right[0],
        &snapshot.status,
        &snapshot.health,
        &snapshot.financials,
        &snapshot.polymarket,
    );
    if snapshot.no_money_mode {
        render_paper_stats(frame, right[1], snapshot.paper_stats.as_ref());
        render_active_windows(frame, right[2], &snapshot.active_windows);
        render_pipeline(frame, right[3], snapshot);
        render_order_tape(frame, right[4], snapshot);
    } else {
        render_active_windows(frame, right[1], &snapshot.active_windows);
        render_pipeline(frame, right[2], snapshot);
        render_order_tape(frame, right[3], snapshot);
    }
}

fn render_paper_stats(frame: &mut Frame, area: Rect, stats: Option<&PaperStatsData>) {
    let block = Block::default()
        .title(" Paper Stats ")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));
    let mut lines = Vec::new();
    if let Some(stats) = stats {
        lines.push(Line::from(format!(
            "Acc: {:.2}% | CI95: {:.1}-{:.1}%",
            stats.accuracy_pct, stats.win_rate_ci_low, stats.win_rate_ci_high
        )));
        lines.push(Line::from(format!(
            "Bank: {:.2} ({:+.2}%) | Wins: {:.0}",
            stats.bankroll_usdc, stats.roi_pct, stats.wins
        )));
        lines.push(Line::from(format!(
            "Loss: {:.0} | Missed: {:.0} | Total: {:.0}",
            stats.losses, stats.missed, stats.total
        )));
        let reason = stats.missed_reason.as_deref().unwrap_or("--");
        lines.push(Line::from(format!("Last Miss: {reason}")));
    } else {
        lines.push(Line::from("Acc: -- | CI95: --"));
        lines.push(Line::from("Bank: -- | Wins: --"));
        lines.push(Line::from("Loss: -- | Missed: -- | Total: --"));
        lines.push(Line::from("Last Miss: --"));
    }
    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, status: &StatusBarData) {
    let risk_style = if status.halted {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let risk_label = if status.halted {
        format!("HALTED ({})", halt_reason_label(status.halt_reason))
    } else {
        "OK".to_string()
    };
    let chainlink_label = if status.chainlink_online {
        "ONLINE"
    } else {
        "OFFLINE"
    };
    let allora_label = if status.allora_online {
        "ONLINE"
    } else {
        "OFFLINE"
    };
    let polymarket_label = if status.polymarket_online {
        "ONLINE"
    } else {
        "OFFLINE"
    };
    let mode_label = if status.no_money_mode {
        "PAPER"
    } else {
        "LIVE"
    };
    let mode_style = if status.no_money_mode {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };

    let spans = vec![
        Span::styled(
            " Bankai ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("Uptime", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(
            format_duration(status.uptime),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled("Risk", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(risk_label, risk_style),
        Span::raw("  "),
        Span::styled("Chainlink", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(chainlink_label, oracle_style(status.chainlink_online)),
        Span::raw("  "),
        Span::styled("Allora", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(allora_label, oracle_style(status.allora_online)),
        Span::raw("  "),
        Span::styled("Polymarket", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(polymarket_label, oracle_style(status.polymarket_online)),
        Span::raw("  "),
        Span::styled("Mode", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(mode_label, mode_style),
        Span::raw("  "),
        Span::styled("Model", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(
            status.model_version.clone(),
            Style::default().fg(Color::White),
        ),
    ];

    let paragraph = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::Black)),
        );
    frame.render_widget(paragraph, area);
}

fn render_system_panel(
    frame: &mut Frame,
    area: Rect,
    status: &StatusBarData,
    health: &HealthPanelData,
    financials: &FinancialPanelData,
    polymarket: &PolymarketPanelData,
) {
    let title = Span::styled(
        " System ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    let risk_label = if status.halted {
        format!("HALTED ({})", halt_reason_label(health.halt_reason))
    } else {
        "OK".to_string()
    };
    lines.push(Line::from(vec![
        Span::styled("Risk: ", Style::default().fg(Color::White)),
        Span::styled(
            risk_label,
            if status.halted {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            },
        ),
    ]));

    lines.push(Line::from(format!(
        "Latency: {} ms | Drift: {} ms | Losses: {}",
        health.latency_ms, health.clock_drift_ms, health.consecutive_losses
    )));
    lines.push(Line::from(format!(
        "Chainlink: {} | Allora: {} | Poly: {}",
        oracle_label(status.chainlink_online),
        oracle_label(status.allora_online),
        oracle_label(status.polymarket_online),
    )));

    let bankroll = financials
        .bankroll_usdc
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "--".to_string());
    let pnl = financials
        .pnl_24h
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "--".to_string());
    lines.push(Line::from(format!("Bankroll: {bankroll} | PnL 24h: {pnl}")));

    let assets = polymarket
        .asset_count
        .map(|v| v.to_string())
        .unwrap_or_else(|| "--".to_string());
    let refresh = polymarket
        .last_refresh
        .map(format_duration)
        .unwrap_or_else(|| "--".to_string());
    lines.push(Line::from(format!(
        "Poly Assets: {assets} | Refresh: {refresh}"
    )));

    if health.chainlink_window_anchor {
        lines.push(Line::from(Span::styled(
            "15m anchor: locked",
            Style::default().fg(Color::Green),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "15m anchor: pending",
            Style::default().fg(Color::Yellow),
        )));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_window_bar(frame: &mut Frame, area: Rect, windows: &[ActiveWindowRow]) {
    let block = Block::default()
        .title(Span::styled(
            " Window Countdown ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let now_ms = now_epoch_ms();
    let mut spans: Vec<Span> = Vec::new();
    for row in windows {
        let countdown = if row.start_time_ms == 0 || row.end_time_ms == 0 {
            "--".to_string()
        } else if now_ms < row.start_time_ms {
            format!("+{}", fmt_secs((row.start_time_ms - now_ms) / 1000))
        } else if now_ms <= row.end_time_ms {
            format!("-{}", fmt_secs((row.end_time_ms - now_ms) / 1000))
        } else {
            "done".to_string()
        };
        let status_style = match row.status.as_str() {
            "ACTIVE" => Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            "UPCOMING" => Style::default().fg(Color::Yellow),
            "PAST" => Style::default().fg(Color::DarkGray),
            _ => Style::default().fg(Color::DarkGray),
        };
        spans.push(Span::styled(
            format!(" {} {} ", row.asset, countdown),
            status_style,
        ));
        spans.push(Span::raw(" "));
    }
    if spans.is_empty() {
        spans.push(Span::styled("--", Style::default().fg(Color::DarkGray)));
    }

    let paragraph = Paragraph::new(Line::from(spans))
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_markets(frame: &mut Frame, area: Rect, markets: &[MarketRow]) {
    let block = Block::default()
        .title(Span::styled(
            " Market Scanner ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    if markets.is_empty() {
        let text = Text::from(Line::from("waiting for market data..."));
        let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Asset"),
        Cell::from("Price"),
        Cell::from("Price (Up)"),
        Cell::from("Start Price"),
        Cell::from("Sig 5m"),
        Cell::from("EV"),
        Cell::from("Side"),
        Cell::from("FeeBps"),
        Cell::from("Signal"),
        Cell::from("Age"),
    ])
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows = markets.iter().map(|row| {
        let mode_style = mode_style(&row.mode);
        Row::new(vec![
            Cell::from(row.asset.clone()),
            Cell::from(format_optional_f64(row.price, 4)),
            Cell::from(format_optional_f64(row.implied_up, 4)),
            Cell::from(format_optional_f64(row.start_price, 4)),
            Cell::from(format_optional_f64(row.inference_5m, 4)),
            Cell::from(format_optional_f64(row.edge_bps, 1)),
            Cell::from(row.side.clone().unwrap_or_else(|| "--".to_string())),
            Cell::from(format_optional_f64(row.fee_bps, 1)),
            Cell::from(Span::styled(row.mode.label(), mode_style)),
            Cell::from(format_age(row.last_update_ms)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(13),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(block)
    .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_activity_log(frame: &mut Frame, area: Rect, entries: &[String]) {
    let lines: Vec<Line> = if entries.is_empty() {
        vec![Line::from("no recent activity")]
    } else {
        entries
            .iter()
            .map(|entry| Line::from(entry.as_str()))
            .collect()
    };

    let block = Block::default()
        .title(Span::styled(
            " Event Feed ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_active_windows(frame: &mut Frame, area: Rect, windows: &[ActiveWindowRow]) {
    let block = Block::default()
        .title(Span::styled(
            " Window Stack ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    if windows.is_empty() {
        let paragraph = Paragraph::new(Line::from("no window data"))
            .block(block)
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Asset"),
        Cell::from("Status"),
        Cell::from("Window (ET)"),
        Cell::from("Market ID"),
    ])
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows = windows.iter().map(|row| {
        Row::new(vec![
            Cell::from(row.asset.clone()),
            Cell::from(row.status.clone()),
            Cell::from(row.window_et.clone()),
            Cell::from(row.market_id.clone()),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Length(22),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(block)
    .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_pipeline(frame: &mut Frame, area: Rect, snapshot: &UiSnapshot) {
    let mut lines: Vec<Line> = Vec::new();
    let label_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    if let Some(status) = snapshot.execution_status.as_ref() {
        lines.push(Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(status.clone(), Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(""));
    }
    let open_orders_label = snapshot
        .open_orders
        .map(|value| value.to_string())
        .unwrap_or_else(|| "--".to_string());
    lines.push(Line::from(vec![
        Span::styled("Orders: ", Style::default().fg(Color::White)),
        Span::raw(format!(
            "ok={} fail={} open={}",
            snapshot.orders_ok, snapshot.orders_fail, open_orders_label
        )),
    ]));
    lines.push(Line::from(""));
    if let Some(state) = format_last_order_state(snapshot.last_order_state.as_ref()) {
        lines.push(Line::from(vec![
            Span::styled("Last Order: ", Style::default().fg(Color::White)),
            Span::raw(state),
        ]));
        lines.push(Line::from(""));
    }

    let trade_line = snapshot
        .order_log
        .iter()
        .find(|entry| entry.contains("[TRADE]"))
        .cloned();
    let alert_line = snapshot
        .activity_log
        .iter()
        .find(|entry| entry.contains("[ALERT]"))
        .cloned();
    if trade_line.is_some() || alert_line.is_some() {
        lines.push(Line::from(""));
    }
    if let Some(trade) = trade_line {
        lines.push(Line::from(Span::styled("Trade", label_style)));
        lines.push(Line::from(format!("  {trade}")));
    }
    if let Some(alert) = alert_line {
        lines.push(Line::from(Span::styled("Alert", label_style)));
        lines.push(Line::from(format!("  {alert}")));
    }

    let block = Block::default()
        .title(Span::styled(
            " Execution ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_order_tape(frame: &mut Frame, area: Rect, snapshot: &UiSnapshot) {
    let block = Block::default()
        .title(Span::styled(
            " Order Tape ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        "Recent Intents",
        Style::default().fg(Color::White),
    )));
    if snapshot.intent_log.is_empty() {
        lines.push(Line::from("  --"));
    } else {
        for entry in snapshot.intent_log.iter().take(3) {
            lines.push(Line::from(format!("  {}", format_intent_entry(entry))));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Recent Orders",
        Style::default().fg(Color::White),
    )));
    if snapshot.order_log.is_empty() {
        lines.push(Line::from("  --"));
    } else {
        for entry in snapshot.order_log.iter().take(3) {
            lines.push(Line::from(format!("  {entry}")));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn format_intent_entry(entry: &str) -> String {
    let mut prefix = "";
    let mut asset = "";
    let mut side = "";
    if let Some(idx) = entry.find("[INTENT]") {
        let before = entry[..idx].trim();
        if let Some(first) = before.split_whitespace().next() {
            if first.starts_with('[') && first.ends_with(']') {
                prefix = first;
            }
        }
        let rest = &entry[(idx + "[INTENT]".len())..];
        let mut parts = rest.split_whitespace();
        asset = parts.next().unwrap_or("");
        side = parts.next().unwrap_or("");
    }
    let outcome = entry
        .split("outcome=")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or("");
    let implied = entry
        .split("implied=")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or("");

    if !asset.is_empty() && !implied.is_empty() {
        let label = if !outcome.is_empty() { outcome } else { side };
        if !prefix.is_empty() {
            format!("{prefix} {asset} {label} implied={implied}")
        } else {
            format!("{asset} {label} implied={implied}")
        }
    } else {
        entry.to_string()
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn now_epoch_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn fmt_secs(secs: u64) -> String {
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes:02}:{seconds:02}")
}

fn format_optional_f64(value: Option<f64>, precision: usize) -> String {
    match value {
        Some(value) => format!("{:.*}", precision, value),
        None => "--".to_string(),
    }
}

fn format_last_order_state(payload: Option<&String>) -> Option<String> {
    let payload = payload?;
    let value: Value = serde_json::from_str(payload).ok()?;
    let status = value.get("status")?.as_str()?.to_string();
    let id = value.get("id").and_then(|v| v.as_str()).unwrap_or("--");
    let matched = value
        .get("matched_size")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let original = value
        .get("original_size")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    Some(format!("{status} id={} {matched:.4}/{original:.4}", id))
}

fn mode_style(mode: &MarketMode) -> Style {
    match mode {
        MarketMode::Snipe => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        MarketMode::Ladder => Style::default().fg(Color::Green),
        MarketMode::Wait => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        MarketMode::Hold => Style::default().fg(Color::Yellow),
        MarketMode::NoSignal => Style::default().fg(Color::DarkGray),
    }
}

fn oracle_style(online: bool) -> Style {
    if online {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    }
}

fn oracle_label(online: bool) -> &'static str {
    if online {
        "ONLINE"
    } else {
        "OFFLINE"
    }
}

fn format_age(last_update_ms: Option<u64>) -> String {
    match last_update_ms {
        Some(value) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            if now == 0 || value == 0 {
                "--".to_string()
            } else {
                let secs = now.saturating_sub(value) / 1000;
                format!("{secs}s")
            }
        }
        None => "--".to_string(),
    }
}

fn halt_reason_label(reason: HaltReason) -> &'static str {
    match reason {
        HaltReason::Latency => "Latency",
        HaltReason::ClockDrift => "Clock",
        HaltReason::ConsecutiveLosses => "Losses",
        HaltReason::Manual => "Manual",
        HaltReason::None => "None",
    }
}
