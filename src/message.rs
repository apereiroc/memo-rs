/// Message that is "emitted" given some event
#[derive(PartialEq, Debug)]
pub enum Message {
    Init,
    NextEntry,
    PreviousEntry,
    Enter,
    Back,
    Quit,
}
