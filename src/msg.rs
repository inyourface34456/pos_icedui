#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrment,
    ToSet(String),
    SetSig,
}
