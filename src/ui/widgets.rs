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

use super::{
    ActiveWindowRow, FinancialPanelData, HealthPanelData, MarketMode, MarketRow,
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
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(layout[1]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(body[0]);
    render_markets(frame, left[0], &snapshot.markets);
    render_activity_log(frame, left[1], &snapshot.activity_log);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(22),
            Constraint::Percentage(18),
            Constraint::Percentage(26),
            Constraint::Percentage(34),
        ])
        .split(body[1]);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(right[0]);

    render_health(frame, top[0], &snapshot.health);
    render_financials(frame, top[1], &snapshot.financials);
    render_polymarket(frame, right[1], &snapshot.polymarket);
    render_active_windows(frame, right[2], &snapshot.active_windows);
    render_pipeline(frame, right[3], snapshot);
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
    let binance_label = if status.binance_online {
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
        Span::styled("Binance", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(binance_label, oracle_style(status.binance_online)),
        Span::raw("  "),
        Span::styled("Allora", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(allora_label, oracle_style(status.allora_online)),
        Span::raw("  "),
        Span::styled("Polymarket", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(polymarket_label, oracle_style(status.polymarket_online)),
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

fn render_markets(frame: &mut Frame, area: Rect, markets: &[MarketRow]) {
    let block = Block::default()
        .title(Span::styled(
            " Markets ",
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
        Cell::from("Sig 5m"),
        Cell::from("Sig 8h"),
        Cell::from("EV 5m"),
        Cell::from("EV 8h"),
        Cell::from("Fee"),
        Cell::from("Mode"),
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
            Cell::from(format_optional_f64(row.inference_5m, 4)),
            Cell::from(format_optional_f64(row.inference_8h, 4)),
            Cell::from(format_optional_f64(row.edge_bps_5m, 1)),
            Cell::from(format_optional_f64(row.edge_bps_8h, 1)),
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
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .block(block)
    .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_health(frame: &mut Frame, area: Rect, health: &HealthPanelData) {
    let status_line = if health.halted {
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("HALTED ({})", halt_reason_label(health.halt_reason)),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled("OK", Style::default().fg(Color::Green)),
        ])
    };

    let lines = vec![
        status_line,
        Line::from(format!("Latency: {} ms", health.latency_ms)),
        Line::from(format!("Clock drift: {} ms", health.clock_drift_ms)),
        Line::from(format!("Losses: {}", health.consecutive_losses)),
        Line::from(format!(
            "Binance window anchor: {}",
            if health.binance_window_anchor { "ON" } else { "OFF" }
        )),
    ];

    let block = Block::default()
        .title(Span::styled(
            " System Health ",
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

fn render_financials(frame: &mut Frame, area: Rect, financials: &FinancialPanelData) {
    let lines = vec![
        Line::from(format!(
            "Bankroll: {}",
            format_optional_currency(financials.bankroll_usdc)
        )),
        Line::from(format!(
            "24h PnL: {}",
            format_optional_currency(financials.pnl_24h)
        )),
    ];

    let block = Block::default()
        .title(Span::styled(
            " Financials ",
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

fn render_polymarket(frame: &mut Frame, area: Rect, polymarket: &PolymarketPanelData) {
    let status_style = oracle_style(polymarket.online);
    let status_label = if polymarket.online {
        "ONLINE"
    } else {
        "OFFLINE"
    };
    let count_line = match polymarket.asset_count {
        Some(count) => format!("Assets discovered: {}", count),
        None => "Assets discovered: --".to_string(),
    };
    let refresh_line = match polymarket.last_refresh {
        Some(age) => format!("Last refresh: {}s ago", age.as_secs()),
        None => "Last refresh: --".to_string(),
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(status_label, status_style),
        ]),
        Line::from(count_line),
        Line::from(refresh_line),
        Line::from("Source: Polymarket Gamma -> Redis asset ids"),
    ];

    let block = Block::default()
        .title(Span::styled(
            " Polymarket Discovery ",
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
            " Activity Log ",
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
            " Active Windows ",
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
    let label_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let mut lines: Vec<Line> = Vec::new();
    let assets = snapshot
        .polymarket
        .asset_count
        .map(|count| count.to_string())
        .unwrap_or_else(|| "--".to_string());
    let refresh = snapshot
        .polymarket
        .last_refresh
        .map(|age| format!("{}s", age.as_secs()))
        .unwrap_or_else(|| "--".to_string());

    lines.push(Line::from(Span::styled("Discovery", label_style)));
    lines.push(Line::from(format!("  assets={} refresh={}", assets, refresh)));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("Windows", label_style)));
    if snapshot.active_windows.is_empty() {
        lines.push(Line::from("  --"));
    } else {
        for row in snapshot.active_windows.iter() {
            let status_style = match row.status.as_str() {
                "ACTIVE" => Style::default().fg(Color::Green),
                "UPCOMING" => Style::default().fg(Color::Yellow),
                "PAST" => Style::default().fg(Color::DarkGray),
                _ => Style::default().fg(Color::DarkGray),
            };
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{:>3}", row.asset), Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(format!("{:<8}", row.status), status_style),
                Span::raw(" "),
                Span::raw(row.window_et.clone()),
            ]));
        }
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("Intent", label_style)));
    if snapshot.intent_log.is_empty() {
        lines.push(Line::from("  --"));
    } else {
        for entry in snapshot.intent_log.iter().take(2) {
            lines.push(Line::from(format!("  {entry}")));
        }
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("Order", label_style)));
    if snapshot.order_log.is_empty() {
        lines.push(Line::from("  --"));
    } else {
        for entry in snapshot.order_log.iter().take(2) {
            lines.push(Line::from(format!("  {entry}")));
        }
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
            " Execution Pipeline ",
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

fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn format_optional_f64(value: Option<f64>, precision: usize) -> String {
    match value {
        Some(value) => format!("{:.*}", precision, value),
        None => "--".to_string(),
    }
}

fn format_optional_currency(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("${value:.2}"),
        None => "N/A".to_string(),
    }
}

fn mode_style(mode: &MarketMode) -> Style {
    match mode {
        MarketMode::Snipe => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        MarketMode::Ladder => Style::default().fg(Color::Green),
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
