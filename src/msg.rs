#[derive(Debug, Clone)]
pub enum Message {
    ToSet(String),
    AddToList,
    Remove(usize)
}
