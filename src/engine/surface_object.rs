pub enum SurfaceObject<'a> {
    ClearScr,
    Point { x: i32, y: i32, value: &'a str },
}

impl SurfaceObject<'_> {
    pub fn draw(&mut self) -> String {
        let result = match self {
            Self::ClearScr => "\u{001B}[2J".to_owned(),
            Self::Point { x, y, value } => format!("\u{001B}[{};{}H{}", y, x, value).to_owned()
        };
        return result; 
    }
}
