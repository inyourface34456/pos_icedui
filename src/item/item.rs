pub struct Item {
    id: usize,
    name: String
}

impl Item {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}