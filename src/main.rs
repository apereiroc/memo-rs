mod config;
mod entry;
mod message;
mod model;
mod model_io;
mod tui;

use config::{PROJECT_NAME, PROJECT_VERSION};
use crossterm::event::{self, Event, KeyCode};
use message::Message;
use model::{Model, RunningState};
use ratatui::symbols::border;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    // Initialise terminal
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    // Initialise model
    let mut model = Model::new("cache/test.json".to_owned());

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

fn view(_model: &mut Model, f: &mut Frame) {
    let widget = make_main_screen();
    f.render_widget(widget, f.size());
}

fn make_main_screen() -> Paragraph<'static> {
    let title = Title::from(format!(" {} v{}", PROJECT_NAME, PROJECT_VERSION).bold());
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let widget = Paragraph::new("").centered().block(block);
    widget
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Enter => {
            // Load the possible cache file
            // and updates the model
            model.load_from_cache();
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
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
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
        _ => None,
    }
}
