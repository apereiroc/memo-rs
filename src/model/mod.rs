pub mod entry;
mod model_io;

use crate::entry::EntryGroup;

/// Model's current running state
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Empty,
    Loaded,
    Done,
}

/// Screen that is currently shown
#[derive(Debug, Default, PartialEq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Secondary,
}

/// Flagship struct in the package. Contains all needed information to display the correct
/// behaviour
#[derive(Debug, Default)]
pub struct Model {
    pub file: String,
    pub(crate) entries: Vec<EntryGroup>,
    pub running_state: RunningState,
    pub idx_entrygroup: usize,
    pub idx_entry: usize,
    pub current_screen: CurrentScreen,
}

impl Model {
    /// Create a new object
    pub fn new(file: String) -> Model {
        Model {
            file,
            entries: vec![],
            running_state: RunningState::Empty,
            idx_entrygroup: 0,
            idx_entry: 0,
            current_screen: CurrentScreen::Main,
        }
    }

    /// Update entry/entrygroup iterator to highlight the next entry
    pub fn next_entry(&mut self) {
        match self.current_screen {
            CurrentScreen::Main => {
                self.idx_entrygroup = (self.idx_entrygroup + 1) % self.entries.len();
            }
            CurrentScreen::Secondary => {
                self.idx_entry =
                    (self.idx_entry + 1) % self.entries[self.idx_entrygroup].entries.len();
            }
        }
    }

    /// Update entry/entrygroup iterator to highlight the previous entry
    pub fn previous_entry(&mut self) {
        match self.current_screen {
            CurrentScreen::Main => {
                self.idx_entrygroup = match self.idx_entrygroup {
                    0 => self.entries.len() - 1,
                    _ => self.idx_entrygroup - 1,
                };
            }
            CurrentScreen::Secondary => {
                self.idx_entry = match self.idx_entry {
                    0 => self.entries[self.idx_entrygroup].entries.len() - 1,
                    _ => self.idx_entry - 1,
                };
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::entry::Entry;

    pub fn make_test_entry_group() -> EntryGroup {
        let entry1 = Entry {
            command: String::from("command1"),
            short_info: String::from("Short description 1"),
            long_info: String::from("Long description 1"),
        };

        let entry2 = Entry {
            command: String::from("command2"),
            short_info: String::from("Short description 2"),
            long_info: String::from("Long description 2"),
        };

        let description = String::from("description");

        EntryGroup::new(description, vec![entry1, entry2])
    }

    #[test]
    fn create_default_model() {
        let model = Model::default();
        assert_eq!(model.file.len(), 0);
        assert!(model.entries.is_empty());
        assert_eq!(model.running_state, RunningState::Empty);
        assert_eq!(model.current_screen, CurrentScreen::Main);
        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 0);
    }

    #[test]
    fn create_model() {
        let file = String::from("test.cache");
        let entrygroup = make_test_entry_group();

        let mut model = Model::new(file);
        model.entries = vec![entrygroup];
        assert_eq!(model.file.len(), 10);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::Empty);
        assert_eq!(model.entries[0].entries[0].command, "command1");
        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 0);
        assert_eq!(model.current_screen, CurrentScreen::Main);
    }

    #[test]
    fn increase_index_entrygroup() {
        let mut model = Model::default();

        let mut egs: Vec<EntryGroup> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };
            let eg = EntryGroup::new("".to_owned(), vec![entry]);

            egs.push(eg);
        }

        model.entries = egs;

        assert_eq!(model.idx_entrygroup, 0);
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 1);
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 2);
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 9);
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 0);
    }

    #[test]
    fn decrease_index_entrygroup() {
        let mut model = Model::default();

        let mut egs: Vec<EntryGroup> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };
            let eg = EntryGroup::new("".to_owned(), vec![entry]);

            egs.push(eg);
        }

        model.entries = egs;

        assert_eq!(model.idx_entrygroup, 0);
        model.previous_entry();
        assert_eq!(model.idx_entrygroup, 9);
        model.previous_entry();
        assert_eq!(model.idx_entrygroup, 8);
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        assert_eq!(model.idx_entrygroup, 1);
        model.previous_entry();
        assert_eq!(model.idx_entrygroup, 0);
    }

    #[test]
    fn increase_index_entry() {
        let mut model = Model::default();

        let mut entries: Vec<Entry> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };

            entries.push(entry);
        }

        let eg = EntryGroup::new("".to_owned(), entries);

        model.entries = vec![eg];
        model.current_screen = CurrentScreen::Secondary;

        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 0);
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 1);
        model.next_entry();
        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 2);
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        model.next_entry();
        assert_eq!(model.idx_entry, 9);
        model.next_entry();
        assert_eq!(model.idx_entry, 0);
    }

    #[test]
    fn decrease_index_entry() {
        let mut model = Model::default();

        let mut entries: Vec<Entry> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };

            entries.push(entry);
        }

        let eg = EntryGroup::new("".to_owned(), entries);

        model.entries = vec![eg];
        model.current_screen = CurrentScreen::Secondary;

        assert_eq!(model.idx_entrygroup, 0);
        assert_eq!(model.idx_entry, 0);
        model.previous_entry();
        assert_eq!(model.idx_entry, 9);
        assert_eq!(model.idx_entrygroup, 0);
        model.previous_entry();
        assert_eq!(model.idx_entry, 8);
        assert_eq!(model.idx_entrygroup, 0);
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        model.previous_entry();
        assert_eq!(model.idx_entry, 1);
        model.previous_entry();
        assert_eq!(model.idx_entry, 0);
    }
}
