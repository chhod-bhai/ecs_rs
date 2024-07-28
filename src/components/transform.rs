use sdl2::rect::Point;

#[derive(PartialEq, Eq, Hash)]
pub struct CTransform {
    pub position: Point,
    pub velocity: Point,
}
