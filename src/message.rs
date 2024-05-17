#[derive(PartialEq, Debug)]
pub enum Message {
    Init,
    NextEntry,
    PreviousEntry,
    Enter,
    Quit,
}
