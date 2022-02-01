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

        surface.draw("\u{001B}[2J");
        surface.draw(format!("\u{001B}[{};{}H@", d.y, d.x).as_str());

        for x in 0..10 {
                surface.draw(format!("\u{001B}[{};{}H%", x, x).as_str());
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
