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
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    // Initialise model
    let mut model = Model::new(args.filename);

    // Main loop
    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

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
    make_main_screen(model, f);
}

fn make_main_screen(model: &Model, f: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Make title block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

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

    f.render_widget(title, main_layout[0]);

    // Make instruction block
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::from(" <q>: quit ").bold().yellow())
        .block(instruction_block)
        .centered();

    f.render_widget(title, main_layout[2]);

    // Make data block
    let data_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(main_layout[1]);
    let data_block = Block::default()
        .borders(Borders::ALL)
        .title("List of entries")
        .style(Style::default());

    let mut list_items = Vec::<ListItem>::new();
    for entry in &model.entries {
        list_items.push(ListItem::new(Line::from(Span::styled(
            &entry.description,
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items)
        .block(data_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let mut state = ListState::default();
    state.select(Some(model.idx_entry));

    f.render_stateful_widget(list, data_layout[0], &mut state);
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
        KeyCode::Down => Some(Message::NextEntry),
        KeyCode::Up => Some(Message::PreviousEntry),
        _ => None,
    }
}
