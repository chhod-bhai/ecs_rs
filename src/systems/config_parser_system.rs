use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};

use sdl2::rect::Point;

use crate::{components::{color::CColor, label::Label, shape::{CShape, Circle, Rectange}, transform::CTransform}, entity::{entity::EntityTag, entity_manager::EntityManager}};

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

pub fn config_parser(file: &str, entity_manager: &mut EntityManager) -> Option<WindowConfig>  {
    println!("parse configs {file}");
    let file_res = File::open(file);
    match file_res {
        Ok(file) => {
            let rdr = BufReader::new(file);
            let mut window_cfg: Option<WindowConfig> = None;
            for line_res in rdr.lines() {
                if let Ok(line) = line_res {
                    let mut line = line.split(' ');
                    let config_char = line.next();
                    match config_char {
                        Some("Window") => {
                           let width_opt = line.next();
                           let height_opt = line.next();
                           window_cfg = width_opt.zip(height_opt).map(|(w, h)| {
                            let width: u32 = FromStr::from_str(w).unwrap();
                            let height: u32 = FromStr::from_str(h).unwrap();
                            WindowConfig{
                                width, height
                            }
                           });
                           
                        },
                        Some("Circle") => {
                            let added_entity_id = entity_manager.add(EntityTag::Shape);
                            if let Some(added_entity) = entity_manager.get_by_id(added_entity_id) {
                                let mut entity = added_entity.borrow_mut();
                                let label = line.next();
                                let x = line.next();
                                let y = line.next();
                                let vx = line.next();
                                let vy = line.next();
                                let r = line.next();
                                let g = line.next();
                                let b = line.next();
                                let radius = line.next();
                                let transform_opt = match (x, y, vx, vy) {
                                   (Some(x), Some(y), Some(vx), Some(vy)) => {
                                    let position = Point::new(FromStr::from_str(x).unwrap(), FromStr::from_str(y).unwrap());
                                    let velocity = Point::new(FromStr::from_str(vx).unwrap(), FromStr::from_str(vy).unwrap());
                                    Some(CTransform {
                                        position, velocity
                                    })
                                   },
                                   _ => None
                                };
                                entity.transform = transform_opt;
                                entity.label = label.map(|l| {
                                    Label {
                                        label: l.to_string()
                                    }
                                });
                                entity.shape = radius.map(|r|  {
                                    CShape::Circle(Circle {
                                        radius: FromStr::from_str(r).unwrap()
                                    })
                                });
                                entity.color = match(r, g, b) {
                                    (Some(r), Some(g), Some(b)) => {
                                        let color = CColor {
                                            r: FromStr::from_str(r).unwrap(),
                                            g: FromStr::from_str(g).unwrap(),
                                            b: FromStr::from_str(b).unwrap(),
                                        };
                                        Some(color)
                                       },
                                       _ => None
                                };
                            }
                        },
                        Some("Rectangle") => {
                            let added_entity_id = entity_manager.add(EntityTag::Shape);
                            if let Some(added_entity) = entity_manager.get_by_id(added_entity_id) {
                                let mut entity = added_entity.borrow_mut();
                                let label = line.next();
                                let x = line.next();
                                let y = line.next();
                                let vx = line.next();
                                let vy = line.next();
                                let r = line.next();
                                let g = line.next();
                                let b = line.next();
                                let width = line.next();
                                let height = line.next();
                                let transform_opt = match (x, y, vx, vy) {
                                   (Some(x), Some(y), Some(vx), Some(vy)) => {
                                    let position = Point::new(FromStr::from_str(x).unwrap(), FromStr::from_str(y).unwrap());
                                    let velocity = Point::new(FromStr::from_str(vx).unwrap(), FromStr::from_str(vy).unwrap());
                                    Some(CTransform {
                                        position, velocity
                                    })
                                   },
                                   _ => None
                                };
                                entity.transform = transform_opt;
                                entity.label = label.map(|l| {
                                    Label {
                                        label: l.to_string()
                                    }
                                });
                                entity.shape = width.zip(height).map(|(w, h)|  {
                                    CShape::Rectangle(Rectange { width:FromStr::from_str(w).unwrap() , height: FromStr::from_str(h).unwrap()})
                                });
                                entity.color = match(r, g, b) {
                                    (Some(r), Some(g), Some(b)) => {
                                        let color = CColor {
                                            r: FromStr::from_str(r).unwrap(),
                                            g: FromStr::from_str(g).unwrap(),
                                            b: FromStr::from_str(b).unwrap(),
                                        };
                                        Some(color)
                                       },
                                       _ => None
                                };
                            }
                        },
                        Some(..) => {
                            eprintln!("Invalid config");
                        },
                        None => {
                            eprintln!("No config found");
                        }
                        
                    }
                }
            }
            return window_cfg;
        },
        Err(e) =>{
            eprintln!("Error reading file {:?}",e);
            return None;
        }
    }
}