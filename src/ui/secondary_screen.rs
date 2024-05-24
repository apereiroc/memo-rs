use super::keybindings_block::render_keybindings_secondary_screen;
use super::title_block::render_title;
use crate::model::Model;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};

//  ----------------------------
// |           TITLE            |
//  ----------------------------
// |               |            |
// |   ENTRIES     | LONG INFO  |
// |               |            |
//  ----------------------------
// |         INSTRUCTIONS       |
//  ----------------------------
pub fn render_secondary_screen(model: &Model, f: &mut Frame) {
    let [title_area, data_area, instruction_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(f.size());

    let [entries_area, long_info_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .areas(data_area);

    render_title(model, f, title_area);
    render_entries(model, f, entries_area);
    render_long_info(model, f, long_info_area);
    render_keybindings_secondary_screen(model, f, instruction_area);
}

//  ----------------------------
// |                            |
//  ----------------------------
// |               |            |
// |   ENTRIES     |            |
// |               |            |
//  ----------------------------
// |                            |
//  ----------------------------
fn render_entries(model: &Model, f: &mut Frame, area: Rect) {
    let outer_block = Block::new()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left)
        .padding(Padding {
            left: 2,
            right: 2,
            top: 1,
            bottom: 1,
        })
        // .fg(TEXT_COLOR)
        // .bg(TODO_HEADER_BG)
        .title("Entry list");
    let inner_block = Block::new()
        // .fg(TEXT_COLOR)
        // .bg(NORMAL_ROW_COLOR)
        .borders(Borders::NONE);

    // Get the inner area from outer_block. We'll use this area later to render the table.
    let outer_area = area;
    let inner_area = outer_block.inner(outer_area);

    // Get list of items
    let (cmds, infos): (Vec<String>, Vec<String>) = model.entries[model.idx_entrygroup]
        .entries
        .iter()
        .map(|entry| {
            let cmd = entry.command.clone();
            let info = entry.short_info.clone();
            (cmd, info)
        })
        .unzip();

    let cmds = List::new(cmds)
        .block(inner_block.clone())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightCyan),
        )
        .highlight_symbol(">> ")
        // .scroll_padding(1)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let infos = List::new(infos)
        .block(inner_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightCyan),
        )
        .direction(ListDirection::TopToBottom);

    // Get current selected item
    let mut current_state = ListState::default();
    current_state.select(Some(model.idx_entry));

    // Render
    f.render_stateful_widget(cmds, inner_area, &mut current_state);
    f.render_widget(outer_block, outer_area);
}

//  ----------------------------
// |                            |
//  ----------------------------
// |               |            |
// |               | LONG INFO  |
// |               |            |
//  ----------------------------
// |                            |
//  ----------------------------
fn render_long_info(model: &Model, f: &mut Frame, area: Rect) {
    let outer_block = Block::new()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left)
        .padding(Padding {
            left: 2,
            right: 1,
            top: 1,
            bottom: 1,
        })
        // .fg(TEXT_COLOR)
        // .bg(TODO_HEADER_BG)
        .title("Preview");
    let inner_block = Block::new()
        // .fg(TEXT_COLOR)
        // .bg(NORMAL_ROW_COLOR)
        .borders(Borders::NONE);
    let outer_area = area;
    let inner_area = outer_block.inner(outer_area);

    // Get current long info
    let long_info = model.entries[model.idx_entrygroup].entries[model.idx_entry]
        .long_info
        .clone();

    let paragraph = Paragraph::new(long_info)
        .block(inner_block)
        .style(Style::new().white().on_black())
        .centered()
        .wrap(Wrap { trim: true });

    // Render
    f.render_widget(paragraph, inner_area);
    f.render_widget(outer_block, outer_area);
}
