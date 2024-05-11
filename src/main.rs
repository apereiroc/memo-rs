mod entry;
mod message;
mod model;
mod model_io;
mod tui;

use crossterm::event::{self, Event, KeyCode};
use message::Message;
use model::{Model, RunningState};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    // Initialise terminal
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    // Initialise model
    let mut model = Model::new("cache_test.json".to_owned());
    model.load_from_cache();

    // Main loop
    while model.running_state != RunningState::SavedAndDone {
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
    f.render_widget(Paragraph::new("Hello world!"), f.size());
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            // You can handle cleanup and exit here
            model.save_to_cache();
        }
    };
    None
}

/// Convert Event to Message
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
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
