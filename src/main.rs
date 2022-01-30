mod engine;
fn main() {
    let mut e = engine::Engine::new();
    e.draw(|| {
        let (mut x, mut y) = (10, 10);
        print!("\u{001B}[1;1H\u{001B}[2J");
        print!("\u{001B}[{};{}H", x, y);
        print!("*");
    });
    e.init(); 
}
