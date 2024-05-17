use crate::config::{PROJECT_AUTHOR, PROJECT_NAME, PROJECT_VERSION};
use crate::model::Model;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};

//  ---------------------------
// |           TITLE           |
//  ---------------------------
// |         |                 |
// | ENTRIES |     PREVIEW     |
// |         |                 |
//  ---------------------------
// |         INSTRUCTIONS      |
//  ---------------------------
pub fn make_main_screen(model: &Model, f: &mut Frame) {
    let [title_area, data_area, instruction_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(f.size());

    let [entries_area, preview_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .areas(data_area);

    make_title(model, f, title_area);
    make_entries(model, f, entries_area);
    make_preview(model, f, preview_area);
    make_instructions(model, f, instruction_area);
}

//  ---------------------------
// |           THIS            |
//  ---------------------------
// |         |                 |
// |         |                 |
// |         |                 |
//  ---------------------------
// |                           |
//  ---------------------------
fn make_title(_: &Model, f: &mut Frame, area: Rect) {
    // Make title block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Fill text
    let title = Paragraph::new(Text::from(
        format!(
            " {} v{} -- {} ",
            PROJECT_NAME, PROJECT_VERSION, PROJECT_AUTHOR
        )
        .bold()
        .yellow(),
    ))
    .block(title_block)
    .centered();

    // Render
    f.render_widget(title, area);
}

//  ---------------------------
// |                           |
//  ---------------------------
// |         |                 |
// |  THIS   |                 |
// |         |                 |
//  ---------------------------
// |                           |
//  ---------------------------
fn make_entries(model: &Model, f: &mut Frame, area: Rect) {
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
    let items: Vec<String> = model
        .entries
        .iter()
        .map(|entry_group| "· ".to_string() + &entry_group.description)
        .collect();

    let items = List::new(items)
        .block(inner_block)
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

    // Get current selected item
    let mut current_state = ListState::default();
    current_state.select(Some(model.idx_entrygroup));

    // Render
    f.render_widget(outer_block, outer_area);
    f.render_stateful_widget(items, inner_area, &mut current_state);
}

//  ---------------------------
// |                           |
//  ---------------------------
// |         |                 |
// |         |      THIS       |
// |         |                 |
//  ---------------------------
// |                           |
//  ---------------------------
fn make_preview(model: &Model, f: &mut Frame, area: Rect) {
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

    // Get list of items

    let items: Vec<String> = model.entries[model.idx_entrygroup]
        .entries
        .iter()
        .map(|entry| entry.short_info.clone())
        .collect();

    let items = List::new(items)
        .block(inner_block)
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always)
        // .scroll_padding(1)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    f.render_widget(items, inner_area);
    f.render_widget(outer_block, outer_area);
}

//  ---------------------------
// |                           |
//  ---------------------------
// |         |                 |
// |         |                 |
// |         |                 |
//  ---------------------------
// |           THIS            |
//  ---------------------------
fn make_instructions(_: &Model, f: &mut Frame, area: Rect) {
    // Make instruction block
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Fill text
    let instructions = Paragraph::new(
        Text::from(" <Up>: go previous   <Down>, <Tab>: go next   <q>: quit ")
            .bold()
            .yellow(),
    )
    .block(instruction_block)
    .centered();

    // Render
    f.render_widget(instructions, area);
}
