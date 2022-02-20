#[derive(Clone, Copy)]
pub enum ForegroundColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
}

pub enum SurfaceObject<'a> {
    ClearScr,
    Color(ForegroundColor),
    Reset,
    Point { x: i32, y: i32, value: &'a str },
}

impl SurfaceObject<'_> {
    pub fn draw(&mut self) -> String {
        let result = match self {
            Self::ClearScr => "\u{001B}[2J".to_owned(),
            Self::Color(color) => format!("\u{001b}[{}m", *color as u8).to_owned(), 
            Self::Reset => "\u{001B}[0m".to_owned(),
            Self::Point { x, y, value } => format!("\u{001B}[{};{}H{}", y, x, value).to_owned(),
        };
        return result;
    }
}
