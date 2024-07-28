use std::slice::Iter;

use crate::components::{color::CColor, label::Label, shape::CShape, transform::CTransform};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum EntityTag {
    Shape,
    Window
}

impl EntityTag {
    pub fn iter() -> Iter<'static, EntityTag> {
        static ENTITY_TAGS: [EntityTag;2] = [EntityTag::Shape, EntityTag::Window];
        ENTITY_TAGS.iter()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
    pub shape: Option<CShape>,
    pub tag: EntityTag,
    pub color: Option<CColor>,
    pub transform: Option<CTransform>,
    pub label: Option<Label>,
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.id.cmp(&other.id);
    }
}


