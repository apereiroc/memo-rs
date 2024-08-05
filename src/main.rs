mod config;
mod message;
mod model;
mod ui;

use model::entry;
use ui::main_screen::render_main_screen;
use ui::secondary_screen::render_secondary_screen;
use ui::tui;

use arboard::Clipboard;
use crossterm::event::{self, Event, KeyCode};
use message::Message;
use model::{CurrentScreen, Model, RunningState};
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

    // Initialise clipboard
    let mut clipboard = Clipboard::new().unwrap();

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

    clipboard.set_text(
        model.entries[model.idx_entrygroup].entries[model.idx_entry]
            .command
            .clone(),
    )?;

    // Close and exit
    tui::restore_terminal()?;
    Ok(())
}

/// Render screen given the state of the model
fn view(model: &Model, f: &mut Frame) {
    match model.current_screen {
        CurrentScreen::Main => match model.running_state {
            RunningState::Empty => (),
            _ => render_main_screen(model, f),
        },
        CurrentScreen::Secondary => render_secondary_screen(model, f),
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        // Load the possible cache file
        // and updates the model
        Message::Init => {
            model.load_from_cache();
        }
        // If main screen: go to next entry group
        // If secondary screen: go to next entry
        Message::NextEntry => {
            model.next_entry();
        }
        // If main screen: go to previous entry group
        // If secondary screen: go to previous entry
        Message::PreviousEntry => {
            model.previous_entry();
        }
        // Go to the secondary screen
        Message::Enter => match model.current_screen {
            CurrentScreen::Main => {
                model.current_screen = CurrentScreen::Secondary;
            }
            CurrentScreen::Secondary => {
                model.running_state = RunningState::Done;
            }
        },
        // Go to the main screen
        Message::Back => {
            model.current_screen = CurrentScreen::Main;
        }
        // Save current status to cache
        // and exit
        Message::Quit => {
            model.save_to_cache();
        }
    };
    None
}

/// Convert general Event to Message
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

/// Convert KeyEvent to Message
fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => Some(Message::NextEntry),
        KeyCode::Up | KeyCode::Char('k') => Some(Message::PreviousEntry),
        KeyCode::Enter => Some(Message::Enter),
        KeyCode::Esc => Some(Message::Back),
        _ => None,
    }
}
