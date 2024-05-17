mod config;
mod message;
mod model;
mod ui;
use crate::model::entry;
use crate::ui::main_screen::make_main_screen;
use crate::ui::tui;

use crossterm::event::{self, Event, KeyCode};
use message::Message;
use model::{Model, RunningState};
use ratatui::prelude::*;
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

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    // println!("Message: {:?}", msg);
    match msg {
        Message::Init => {
            // Load the possible cache file
            // and updates the model
            model.load_from_cache();
        }
        Message::NextEntry => {
            // Go to next entry in entry group
            model.next_entrygroup();
        }
        Message::PreviousEntry => {
            // Go to previous entry in entry group
            model.previous_entrygroup();
        }
        Message::Enter => {
            todo!();
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
        return Ok(Some(Message::Init));
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
        KeyCode::Enter => Some(Message::Enter),
        _ => None,
    }
}
