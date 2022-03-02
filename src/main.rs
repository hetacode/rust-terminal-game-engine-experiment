use engine::surface_object::{ForegroundColor, SurfaceObject};
use std::sync::Arc;
use std::sync::Mutex;
mod engine;

struct Game {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut e = engine::Engine::new();
    let data = Arc::new(Mutex::new(Game { x: 10, y: 10 }));

    let d = data.clone();
    e.draw(move |surface| {
        let d = d.lock().unwrap();

        surface.draw(SurfaceObject::ClearScr);
        surface.draw(SurfaceObject::Point {
            x: d.x,
            y: d.y,
            value: "🐧",
        });

        for x in 1..11 {
            surface.draw(SurfaceObject::Point {
                x,
                y: x,
                value: "🐵",
            });

            surface.draw(SurfaceObject::Color(ForegroundColor::Blue));
            surface.draw(SurfaceObject::Point {
                x,
                y: 5,
                value: "=",
            });

            surface.draw(SurfaceObject::Color(ForegroundColor::Red));
            surface.draw(SurfaceObject::Point {
                x,
                y: 6,
                value: "=",
            });

            surface.draw(SurfaceObject::Reset);

            surface.draw(SurfaceObject::Line {
                x1: 16,
                y1: 1,
                x2: 16,
                y2: 10,
                value: "#",
            });
            surface.draw(SurfaceObject::Line {
                x1: 17,
                y1: 1,
                x2: 21,
                y2: 10,
                value: "#",
            });
            surface.draw(SurfaceObject::Line {
                x1: 15,
                y1: 1,
                x2: 10,
                y2: 10,
                value: "%",
            });
        }
    });

    let d = data.clone();
    e.keyboard_events(move |key_code| {
        let mut d = d.lock().unwrap();
        match key_code {
            100 => d.x = d.x + 1,
            115 => d.y = d.y + 1,
            97 => d.x = d.x - 1,
            119 => d.y = d.y - 1,
            _ => {}
        }
    });

    e.init();
}
