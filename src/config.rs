/// Project name
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

/// Project version
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Project author
pub const PROJECT_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

/// Instructions rendered in main screen
pub const KEYBINDINGS_INFO_MAIN_SCREEN: &str =
    " (q) quit | (↓) Go next | (↑) Go previous | (Enter) Go selected ";

/// Instructions rendered in secondary screen
pub const KEYBINDINGS_INFO_SECONDARY_SCREEN: &str =
    " (q) quit | (Esc) Go back | (↓) Go next | (↑) Go previous | (Enter) Go selected ";

/// Symbol rendered at the selected object
pub const ITEM_SELECTION_SYMBOL_STRING: &str = ">> ";

/// Title for entry list rendered in main screen
pub const MAIN_SCREEN_ENTRIES_TITLE: &str = "Entry list";

/// Title for preview rendered in main screen
pub const MAIN_SCREEN_PREVIEW_TITLE: &str = "Preview";

/// Title for long description rendered in secondary screen
pub const SECONDARY_SCREEN_LONG_INFO_TITLE: &str = "Description";

/// Title for entry list rendered in secondary screen
pub const SECONDARY_SCREEN_ENTRIES_TITLE: &str = "Entry list";
