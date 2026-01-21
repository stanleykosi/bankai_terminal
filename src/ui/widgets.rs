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

use super::{FinancialPanelData, HealthPanelData, MarketMode, MarketRow, StatusBarData, UiSnapshot};
use crate::engine::risk::HaltReason;

pub fn render_dashboard(frame: &mut Frame, snapshot: &UiSnapshot) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(frame.size());

    render_status_bar(frame, layout[0], &snapshot.status);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(layout[1]);

    render_markets(frame, body[0], &snapshot.markets);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(body[1]);

    render_health(frame, right[0], &snapshot.health);
    render_financials(frame, right[1], &snapshot.financials);
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
    let binance_label = if status.binance_online { "ONLINE" } else { "OFFLINE" };
    let allora_label = if status.allora_online { "ONLINE" } else { "OFFLINE" };

    let spans = vec![
        Span::styled("Bankai", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | uptime: "),
        Span::styled(format_duration(status.uptime), Style::default().fg(Color::White)),
        Span::raw(" | risk: "),
        Span::styled(risk_label, risk_style),
        Span::raw(" | binance: "),
        Span::styled(binance_label, oracle_style(status.binance_online)),
        Span::raw(" | allora: "),
        Span::styled(allora_label, oracle_style(status.allora_online)),
    ];

    let paragraph = Paragraph::new(Line::from(spans)).alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_markets(frame: &mut Frame, area: Rect, markets: &[MarketRow]) {
    let block = Block::default()
        .title("Active Markets")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    if markets.is_empty() {
        let text = Text::from(Line::from("waiting for market data..."));
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Asset"),
        Cell::from("Price"),
        Cell::from("Signal"),
        Cell::from("EV(bps)"),
        Cell::from("Fee(bps)"),
        Cell::from("Mode"),
    ])
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows = markets.iter().map(|row| {
        let mode_style = mode_style(&row.mode);
        Row::new(vec![
            Cell::from(row.asset.clone()),
            Cell::from(format_optional_f64(row.price, 4)),
            Cell::from(format_optional_f64(row.inference, 4)),
            Cell::from(format_optional_f64(row.edge_bps, 1)),
            Cell::from(format_optional_f64(row.fee_bps, 1)),
            Cell::from(Span::styled(row.mode.label(), mode_style)),
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
    ];

    let block = Block::default()
        .title("System Health")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block).alignment(Alignment::Left);
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
        .title("Financials")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block).alignment(Alignment::Left);
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
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Red)
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
