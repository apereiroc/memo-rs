use crate::config::KEYBINDINGS_INFO;
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
pub(crate) fn render_keybindings(_: &Model, f: &mut Frame, area: Rect) {
    // Make instruction block
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Fill text
    let instructions = Paragraph::new(Text::from(KEYBINDINGS_INFO).bold().yellow())
        .block(instruction_block)
        .centered();

    // Render
    f.render_widget(instructions, area);
}
