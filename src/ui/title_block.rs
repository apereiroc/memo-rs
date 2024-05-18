use crate::config::{PROJECT_AUTHOR, PROJECT_NAME, PROJECT_VERSION};
use crate::model::Model;
use ratatui::widgets::block::*;
use ratatui::{prelude::*, widgets::*};

//  ---------------------------
// |           THIS            |
//  ---------------------------
// |                           |
// |                           |
// |                           |
//  ---------------------------
// |                           |
//  ---------------------------
pub(crate) fn render_title(_: &Model, f: &mut Frame, area: Rect) {
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
