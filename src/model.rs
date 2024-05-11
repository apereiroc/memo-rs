use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Empty,
    LoadedAndRunning,
    SavedAndDone,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Entry {
    pub command: String,
    pub short_info: String,
    pub long_info: String,
}

#[derive(Debug, Default)]
struct Model {
    cache_path: String,
    entries: Vec<Entry>,
    running_state: RunningState,
}

impl Model {
    pub fn new(cache_path: String, entries: Vec<Entry>) -> Model {
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
        // Read JSON data from the file
        let mut file = File::open(&self.cache_path).expect("Failed to open cache file");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Failed to read cache file");

        // Deserialize JSON data into a vector of Entry structs
        let loaded_entries: Vec<Entry> = json_data
            .lines()
            .map(|line| serde_json::from_str(line).expect("Deserialization failed"))
            .collect();

        // Print loaded entries for demonstration
        for entry in loaded_entries {
            self.entries.push(entry);
        }

        self.running_state = RunningState::LoadedAndRunning;
    }

    fn save_to_cache(&mut self) {
        // Serialize each Entry to JSON and collect into a vector of strings
        let json_entries: Vec<String> = self
            .entries
            .iter()
            .map(|entry| serde_json::to_string(entry).expect("Serialization failed"))
            .collect();

        let mut file = File::create(&self.cache_path).expect("Failed to create the cache file");

        for json_entry in &json_entries {
            file.write_all(json_entry.as_bytes())
                .expect("Failed to write to file");
            file.write_all(b"\n").expect("Failed to write to file");
        }

        self.running_state = RunningState::SavedAndDone;
    }
}

#[test]
fn create_default_entry() {
    let entry = Entry::default();
    assert_eq!(entry.command.len(), 0);
    assert_eq!(entry.short_info.len(), 0);
    assert_eq!(entry.long_info.len(), 0);
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
    let entries = vec![
        Entry {
            command: String::from("command1"),
            short_info: String::from("Short description 1"),
            long_info: String::from("Long description 1"),
        },
        Entry {
            command: String::from("command2"),
            short_info: String::from("Short description 2"),
            long_info: String::from("Long description 2"),
        },
    ];
    let cache_path = String::from("test.cache");

    let model = Model::new(cache_path, entries);
    assert_eq!(model.cache_path.len(), 10);
    assert_eq!(model.entries.len(), 2);
    assert_eq!(model.running_state, RunningState::Empty);
    assert_eq!(model.entries[0].command, "command1");
}

#[test]
fn save_and_load_entries_to_cache() -> std::io::Result<()> {
    let entries = vec![
        Entry {
            command: String::from("command1"),
            short_info: String::from("Short description 1"),
            long_info: String::from("Long description 1"),
        },
        Entry {
            command: String::from("command2"),
            short_info: String::from("Short description 2"),
            long_info: String::from("Long description 2"),
        },
    ];

    let mut model = Model::new(String::from("test.cache"), entries);

    model.save_to_cache();

    model.clear();

    model.load_from_cache();

    assert_eq!(model.cache_path.len(), 10);
    assert_eq!(model.entries.len(), 2);
    assert_eq!(model.running_state, RunningState::LoadedAndRunning);
    assert_eq!(model.entries[0].command, "command1");

    // Cleaning
    std::fs::remove_file("test.cache")?;
    Ok(())
}
