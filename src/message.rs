#[derive(PartialEq, Debug)]
pub enum Message {
    Init,
    NextEntryGroup,
    PreviousEntryGroup,
    NextEntry,
    PreviousEntry,
    Enter,
    Quit,
}
