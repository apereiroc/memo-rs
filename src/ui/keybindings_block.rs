use crate::config::{KEYBINDINGS_INFO_MAIN_SCREEN, KEYBINDINGS_INFO_SECONDARY_SCREEN};
use crate::model::Model;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};

//  ---------------------------
// |                           |
//  ---------------------------
// |                           |
// |                           |
// |                           |
//  ---------------------------
// |           THIS            |
//  ---------------------------
fn render_keybindings(_: &Model, f: &mut Frame, area: Rect, keybindings_info: &str) {
    // Make instruction block
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Fill text
    let instructions = Paragraph::new(Text::from(keybindings_info).bold().yellow())
        .block(instruction_block)
        .centered();

    // Render
    f.render_widget(instructions, area);
}

pub(crate) fn render_keybindings_main_screen(model: &Model, f: &mut Frame, area: Rect) {
    render_keybindings(model, f, area, KEYBINDINGS_INFO_MAIN_SCREEN);
}

pub(crate) fn render_keybindings_secondary_screen(model: &Model, f: &mut Frame, area: Rect) {
    render_keybindings(model, f, area, KEYBINDINGS_INFO_SECONDARY_SCREEN);
}
