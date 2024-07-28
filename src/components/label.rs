#[derive(PartialEq, Eq, Hash)]
pub struct Label {
    pub label: String
}

impl Label {
    pub fn new(label: String) -> Self {
        Self{label}
    }
}
