use std::fs::File;
use std::io::prelude::*;

use crate::entry::{Entry, EntryGroup};

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Empty,
    LoadedAndRunning,
    SavedAndDone,
}

#[derive(Debug, Default)]
struct Model {
    cache_path: String,
    entries: Vec<EntryGroup>,
    running_state: RunningState,
}

impl Model {
    pub fn new(cache_path: String, entries: Vec<EntryGroup>) -> Model {
        Model {
            cache_path,
            entries,
            running_state: RunningState::Empty,
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.running_state = RunningState::Empty;
    }

    fn load_from_cache(&mut self) {
        if self.cache_path.is_empty() {
            panic!("Trying to read previous status, but no path was specified");
        }

        // Read JSON data from the file
        let mut file = File::open(&self.cache_path).expect("Failed to open cache file");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Failed to read cache file");

        // Deserialize JSON data into a vector of Entry structs
        let entry_groups: Vec<EntryGroup> =
            serde_json::from_str(&json_data).expect("Deserialization failed");

        // Print loaded entries for demonstration
        for entry_group in entry_groups {
            self.entries.push(entry_group);
        }

        self.running_state = RunningState::LoadedAndRunning;
    }

    fn save_to_cache(&mut self) {
        if self.cache_path.is_empty() {
            panic!("Trying to save current status, but no path was specified");
        }

        // Serialize EntryGroups to JSON
        let json_data = serde_json::to_string(&self.entries).expect("Serialization failed");

        let mut file = File::create(&self.cache_path).expect("Failed to create the cache file");

        file.write_all(json_data.as_bytes())
            .expect("Failed to write to file");

        self.running_state = RunningState::SavedAndDone;
    }
}

fn make_test_entry_group() -> EntryGroup {
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let model = Model::new(cache_path, vec![entrygroup]);
        assert_eq!(model.cache_path.len(), 10);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::Empty);
        assert_eq!(model.entries[0].entries[0].command, "command1");
    }

    #[test]
    fn save_and_load_entries_to_cache() -> std::io::Result<()> {
        let entrygroup = make_test_entry_group();

        let mut model = Model::new(String::from("test.cache"), vec![entrygroup]);

        model.save_to_cache();

        model.clear();

        model.load_from_cache();

        assert_eq!(model.cache_path.len(), 10);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::LoadedAndRunning);
        assert_eq!(model.entries[0].entries[0].command, "command1");

        // Cleaning
        std::fs::remove_file("test.cache")?;
        Ok(())
    }
}
