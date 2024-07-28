
#[derive(PartialEq, Eq, Hash)]
pub struct Circle {
    pub radius: u16
}
#[derive(PartialEq, Eq, Hash)]
pub struct Rectange {
    pub width: u16,
    pub height: u16,
}

#[derive(PartialEq, Eq, Hash)]
pub enum CShape{
    Circle(Circle),
    Rectangle(Rectange)
}

