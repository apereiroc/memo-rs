use crate::entry::EntryGroup;
use crate::model::Model;
use crate::RunningState;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

impl Model {
    pub fn load_from_cache(&mut self) {
        if self.cache_path.is_empty() {
            panic!("Trying to read previous status, but no path was specified");
        }

        // Attempt to open the file
        let file_result = File::open(&self.cache_path);

        // Check if the file was successfully opened
        match file_result {
            Ok(mut file) => {
                // File exists, read JSON data from the file
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
            }
            Err(_) => {
                // File does not exist, do nothing
            }
        }

        self.running_state = RunningState::Loaded;
    }

    pub fn save_to_cache(&mut self) {
        if self.cache_path.is_empty() {
            panic!("Trying to save current status, but no path was specified");
        }

        // Serialize EntryGroups to JSON
        let json_data = serde_json::to_string_pretty(&self.entries).expect("Serialization failed");

        // Ensure the parent directories exist
        let parent_dir = Path::new(&self.cache_path).parent().unwrap();
        if !parent_dir.exists() {
            create_dir_all(parent_dir).expect("Failed to create directories");
        }

        let mut file = File::create(&self.cache_path).expect("Failed to create the cache file");

        file.write_all(json_data.as_bytes())
            .expect("Failed to write to file");

        self.running_state = RunningState::Done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tests::make_test_entry_group;

    #[test]
    fn save_and_load_entries_to_cache() -> std::io::Result<()> {
        let entrygroup = make_test_entry_group();

        let mut model = Model::new(String::from("./path/to/test/test.cache"));
        model.entries = vec![entrygroup];

        model.save_to_cache();

        model.entries.clear();
        model.running_state = RunningState::Empty;

        model.load_from_cache();

        assert_eq!(model.cache_path.len(), 25);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::Loaded);
        assert_eq!(model.entries[0].entries[0].command, "command1");

        // Cleaning
        std::fs::remove_dir_all("./path")?;
        Ok(())
    }
}
