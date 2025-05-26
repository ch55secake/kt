use crate::app::App;
use ratatui::backend::Backend;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

#[allow(clippy::extra_unused_type_parameters)]
pub fn draw<B: Backend>(f: &mut Frame, app: &mut App) {
    let size = f.area();

    let chunks = if app.command_mode {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(size)
    } else {
        vec![size].into()
    };

    let col_name = 20;
    let col_status = 10;
    let col_age = 6;

    let title_span = Span::styled(
        app.current_page.title(),
        Style::default()
            .fg(app.current_page.color())
            .add_modifier(Modifier::BOLD),
    );

    let header_block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .title(title_span);

    let header_area = Rect {
        x: chunks[0].x,
        y: chunks[0].y,
        width: chunks[0].width,
        height: 1,
    };
    f.render_widget(header_block, header_area);

    let help_text = "? = Help";
    let help_width = help_text.len() as u16;
    let help_area = Rect {
        x: chunks[0].x + chunks[0].width.saturating_sub(help_width + 2),
        y: chunks[0].y,
        width: help_width + 1,
        height: 1,
    };
    let help_span = Span::styled(
        help_text,
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );
    let help_para = Paragraph::new(help_span);
    f.render_widget(help_para, help_area);

    let list_area = Rect {
        x: chunks[0].x,
        y: chunks[0].y + 1,
        width: chunks[0].width,
        height: chunks[0].height.saturating_sub(1),
    };

    let mut items: Vec<ListItem> = Vec::new();

    let header_line = create_header(col_name, col_status, col_age);
    items.push(ListItem::new(header_line).style(Style::default().fg(Color::Gray)));

    build_columns_from_items(app, col_name, col_status, col_age, &mut items);

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index + 1));

    let list = List::new(items)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, list_area, &mut list_state);

    if app.command_mode {
        f.render_widget(create_command_palette_widget(app), chunks[1]);
    }

    if app.show_help {
        let area = Rect {
            x: size.width / 6,
            y: size.height / 6,
            width: size.width * 2 / 3,
            height: size.height * 2 / 3,
        };
        f.render_widget(Clear, area);
        f.render_widget(create_help_popup(), area);
    }
}

/// Create the actual help popup
fn create_help_popup() -> Paragraph<'static> {
    Paragraph::new(create_help_info())
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true })
}

/// Create header line that in has Name, Status and Age
fn create_header(col_name: u16, col_status: u16, col_age: u16) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            fixed_width("Name", col_name),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            fixed_width("Status", col_status),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            fixed_width("Age", col_age),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])
}

/// Set fixed width of each column
fn fixed_width(s: &str, width: u16) -> String {
    let len = s.len() as u16;
    if len == width {
        s.to_string()
    } else if len > width {
        s.chars().take(width as usize).collect()
    } else {
        let mut result = s.to_string();
        for _ in 0..(width - len) {
            result.push(' ');
        }
        result
    }
}

/// Build the columns from the items that the app has, e.g: the pods defined
fn build_columns_from_items(
    app: &mut App,
    col_name: u16,
    col_status: u16,
    col_age: u16,
    items: &mut Vec<ListItem>,
) {
    for item in app.items() {
        let status = "Running";
        let age = "5m";
        let line = Line::from(vec![
            Span::raw(fixed_width(item, col_name)),
            Span::raw(" "),
            Span::raw(fixed_width(status, col_status)),
            Span::raw(" "),
            Span::raw(fixed_width(age, col_age)),
        ]);
        items.push(ListItem::new(line));
    }
}

/// Create the command palette widget and pass through the mutable app
fn create_command_palette_widget(app: &mut App) -> Paragraph {
    Paragraph::new(Line::from(vec![Span::styled(
        format!(":{}", app.command_input),
        Style::default().fg(Color::LightBlue),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Command"))
}

/// Create text within help panes divided into lines
fn create_help_info() -> Vec<Line<'static>> {
    vec![
        Line::from("Help:"),
        Line::from("  q - Quit"),
        Line::from("  Left/Right - Switch page"),
        Line::from("  Up/Down - Move selection"),
        Line::from("  / - Enter command mode"),
        Line::from("  ? - Toggle help"),
        Line::from(""),
        Line::from("Commands (type after / and press Enter):"),
        Line::from("  pods"),
        Line::from("  deployments"),
        Line::from("  services"),
        Line::from(""),
        Line::from("Press ? again to close this help."),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_width() {}

    #[test]
    fn test_create_command_palette_widget() {}

    #[test]
    fn test_build_columns_from_items() {}
}
