#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
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
    assert_eq!(model.entries.is_empty(), true);
    assert_eq!(model.running_state, RunningState::Running);
}
