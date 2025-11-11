use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Status bar
        ])
        .split(f.area());

    render_title(f, chunks[0]);
    render_challenge_list(f, chunks[1], app);
    render_status_bar(f, chunks[2], app);
}

fn render_title(f: &mut Frame, area: Rect) {
    let title_text = vec![Line::from(Span::styled(
        "Crackmes.one Challenge Browser",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_challenge_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .challenges
        .iter()
        .map(|challenge| {
            let content = format!(
                "{:<30} | {:>4.1} | {:>4.1} | {:<15} | {:<8} | {:<15}",
                truncate(&challenge.name, 30),
                challenge.difficulty,
                challenge.quality,
                format!("{:?}", challenge.language),
                format!("{:?}", challenge.arch),
                format!("{:?}", challenge.platform),
            );

            ListItem::new(Line::from(Span::raw(content)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Challenges (↑/↓: Navigate, Enter: Download, q: Quit)"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let status_text = if let Some(challenge) = app.get_selected_challenge() {
        format!(
            "Selected: {} by {} | {}",
            challenge.name, challenge.author, app.status_message
        )
    } else {
        app.status_message.clone()
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:<width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
