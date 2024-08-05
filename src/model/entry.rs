use serde::{Deserialize, Serialize};

/// Entry model. Saves information about the command string that will be returned by the
/// application, a brief description that will be shown to indicate what that command does,
/// and a long description that the user can pass to explain more in detail what the command does,
/// or to give an example, or whatever
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Entry {
    pub command: String,
    pub short_info: String,
    pub long_info: String,
}

/// Entry group model. Saves a global description and a vector of entries
/// e.g. cmake - 1. create build files 2. build 3. install
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EntryGroup {
    pub description: String,
    pub entries: Vec<Entry>,
}

impl EntryGroup {
    pub fn new(description: String, entries: Vec<Entry>) -> EntryGroup {
        EntryGroup {
            description,
            entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_entry() {
        let entry = Entry::default();
        assert_eq!(entry.command.len(), 0);
        assert_eq!(entry.short_info.len(), 0);
        assert_eq!(entry.long_info.len(), 0);
    }

    #[test]
    fn create_entry() {
        let command = String::from("my_command");
        let short_info = String::from("short_info");
        let long_info = String::from("long_info");
        let entry = Entry {
            command,
            short_info,
            long_info,
        };

        assert_eq!(entry.command, "my_command");
        assert_eq!(entry.short_info, "short_info");
        assert_eq!(entry.long_info, "long_info");
    }

    #[test]
    fn create_default_entry_group() {
        let entry = EntryGroup::default();
        assert_eq!(entry.description.len(), 0);
        assert_eq!(entry.entries.len(), 0);
    }

    #[test]
    fn create_entry_group() {
        let description = String::from("my_description");
        let entry1 = Entry {
            command: String::from("my_command1"),
            short_info: String::from("info"),
            long_info: String::from(""),
        };
        let entry2 = Entry {
            command: String::from("my_command2"),
            short_info: String::from("infooo"),
            long_info: String::from(""),
        };

        let eg = EntryGroup::new(description, vec![entry1, entry2]);

        assert_eq!(eg.description, "my_description");
        assert_eq!(eg.entries[0].command, "my_command1");
        assert_eq!(eg.entries[1].command, "my_command2");
        assert_eq!(eg.entries[0].short_info, "info");
        assert_eq!(eg.entries[1].short_info, "infooo");
        assert_eq!(eg.entries[0].long_info, "");
        assert_eq!(eg.entries[1].long_info.len(), 0);
    }
}
