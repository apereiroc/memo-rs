mod entry;
mod message;
mod model;
mod tui;

use crossterm::event::{self, Event, KeyCode};
use message::Message;
use model::{Model, RunningState};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::SavedAndDone {
        // TODO: Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // TODO: Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // TODO: Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &mut Model, f: &mut Frame) {
    f.render_widget(Paragraph::new("Hello world!"), f.size());
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::SavedAndDone;
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
