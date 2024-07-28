
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r,g,b}
    }
}