use crate::entry::EntryGroup;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Empty,
    LoadedAndRunning,
    SavedAndDone,
}

#[derive(Debug, Default)]
pub struct Model {
    pub cache_path: String,
    pub(crate) entries: Vec<EntryGroup>,
    pub running_state: RunningState,
}

impl Model {
    pub fn new(cache_path: String) -> Model {
        Model {
            cache_path,
            entries: vec![],
            running_state: RunningState::Empty,
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
        assert_eq!(model.cache_path.len(), 0);
        assert!(model.entries.is_empty());
        assert_eq!(model.running_state, RunningState::Empty);
    }

    #[test]
    fn create_model() {
        let cache_path = String::from("test.cache");
        let entrygroup = make_test_entry_group();

        let mut model = Model::new(cache_path);
        model.entries = vec![entrygroup];
        assert_eq!(model.cache_path.len(), 10);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::Empty);
        assert_eq!(model.entries[0].entries[0].command, "command1");
    }
}
