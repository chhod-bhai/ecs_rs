use std::{thread::sleep, time::Duration};

use components::shape::CShape;
use entity::entity_manager::EntityManager;
use sdl2::gfx::primitives::DrawRenderer;

mod components;
mod entity;
mod systems;

pub fn main() {
    let mut entity_manager = EntityManager::new();
    let window_opt =
        systems::config_parser_system::config_parser("config.txt", &mut entity_manager);
    if let Some(window_config) = window_opt {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let sdl_window = video_subsystem
            .window("rust-sdl2 demo", window_config.width, window_config.height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = sdl_window.into_canvas().build().unwrap();

        canvas.clear();
        canvas.present();
        loop {
            canvas.clear();
            for entity_cell in entity_manager.entities.values() {
                let entity = entity_cell.borrow();
                match (
                    entity.color.as_ref(),
                    entity.shape.as_ref(),
                    entity.transform.as_ref(),
                ) {
                    (Some(color), Some(shape), Some(transform)) => {
                        match shape {
                            CShape::Circle(circle) => {
                                let _res: Result<(), String> = canvas.circle(
                                    transform.position.x.try_into().unwrap(),
                                    transform.position.y.try_into().unwrap(),
                                    circle.radius.try_into().unwrap(),
                                    (color.r, color.g, color.b, 1),
                                );
                            }
                            CShape::Rectangle(rect) => {
                                let x1: i16 = transform.position.x.try_into().unwrap();
                                let y1: i16 = transform.position.y.try_into().unwrap();
                                let color = (color.r, color.g, color.b,1);
                                let width:i16 = rect.width.try_into().unwrap();
                                let height: i16 = rect.height.try_into().unwrap();

                                let _res = canvas.rectangle(x1, y1, x1 + width, y1+ height, color);
                            }
                        }
                    }
                    _ => {
                        eprintln!("values not found for rendering shapes")
                    }
                }
            }
            canvas.present();
            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
