#[derive(PartialEq, Debug)]
pub enum Message {
    Init,
    NextEntryGroup,
    PreviousEntryGroup,
    Enter,
    Quit,
}
