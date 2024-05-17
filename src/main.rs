mod config;
mod entry;
mod message;
mod model;
mod model_io;
mod tui;

use config::{PROJECT_AUTHOR, PROJECT_NAME, PROJECT_VERSION};
use crossterm::event::{self, Event, KeyCode};
use entry::{Entry, EntryGroup};
use message::Message;
use model::{Model, RunningState};
use ratatui::symbols::border;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// TODO: set cache thing (default value?)
    #[arg(short, long)]
    filename: String,
}

fn main() -> color_eyre::Result<()> {
    // Get command line arguments
    let args = Args::parse();

    // Initialise terminal
    tui::install_panic_hook()?;
    let mut terminal = tui::init_terminal()?;

    // Initialise model
    let mut model = Model::new(args.filename);

    // Main loop
    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    // Close and exit
    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &Model, f: &mut Frame) {
    if model.running_state != RunningState::Empty {
        make_main_screen(model, f);
    }
}

//  ---------------------------
// |           TITLE           |
//  ---------------------------
// |         |                 |
// | ENTRIES |     PREVIEW     |
// |         |                 |
//  ---------------------------
// |         INSTRUCTIONS      |
//  ---------------------------
fn make_main_screen(model: &Model, f: &mut Frame) {
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
    current_state.select(Some(model.idx_entry));

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

    let items: Vec<String> = model.entries[model.idx_entry]
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

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    // println!("Message: {:?}", msg);
    match msg {
        Message::Enter => {
            // Load the possible cache file
            // and updates the model
            model.load_from_cache();
        }
        Message::NextEntry => {
            // Go to next entry in entry group
            model.next_entry();
        }
        Message::PreviousEntry => {
            // Go to previous entry in entry group
            model.previous_entry();
        }
        Message::Quit => {
            // Save current status to cache
            // and exit
            model.save_to_cache();
            model.running_state = RunningState::Done;
        }
    };
    None
}

/// Convert Event to Message
fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if model.running_state == RunningState::Empty {
        return Ok(Some(Message::Enter));
    }

    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Down | KeyCode::Tab => Some(Message::NextEntry),
        KeyCode::Up => Some(Message::PreviousEntry),
        _ => None,
    }
}
