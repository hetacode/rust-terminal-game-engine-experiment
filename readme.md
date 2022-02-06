## Project status
Project is created just only to touch Rust lang a bit.

## Usage
Add `stge` as a dependency in `Cargo.toml`:
```toml
[dependencies]
stge = { git = "https://github.com/hetacode/terminal-game-engine.git" }
```

Example of a "game": 
```rust
use stge::Engine;
use stge::surface_object::SurfaceObject;
use std::sync::Arc;
use std::sync::Mutex;

struct Game {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut e = Engine::new();
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
    });

    let d = data.clone();
    e.keyboard_events(move |key_code| {
        let mut d = d.lock().unwrap();
        match key_code {
            100 => d.x = d.x + 1, // w
            115 => d.y = d.y + 1, // d
            97 => d.x = d.x - 1,  // s
            119 => d.y = d.y - 1, // a
            _ => {}
        }
    });

    e.init();
}
```
