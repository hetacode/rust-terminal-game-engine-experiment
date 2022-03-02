#[derive(Clone, Copy)]
pub enum ForegroundColor {
    Black = 30,
    Blue = 34,
    Cyan = 36,
    Green = 32,
    Magenta = 35,
    Red = 31,
    White = 37,
    Yellow = 33,
}

pub enum SurfaceObject<'a> {
    ClearScr,
    Color(ForegroundColor),
    Reset,
    Point {
        x: i32,
        y: i32,
        value: &'a str,
    },
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        value: &'a str,
    },
}

impl SurfaceObject<'_> {
    pub fn draw(&mut self) -> String {
        let result = match self {
            Self::ClearScr => "\u{001B}[2J".to_owned(),
            Self::Color(color) => format!("\u{001b}[{}m", *color as u8).to_owned(),
            Self::Reset => "\u{001B}[0m".to_owned(),
            Self::Point { x, y, value } => format!("\u{001B}[{};{}H{}", y, x, value).to_owned(),
            &mut Self::Line {
                x1,
                y1,
                x2,
                y2,
                value,
            } => self.line(x1, y1, x2, y2, value),
        };
        return result;
    }

    fn line<'a>(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, value: &'a str) -> String {
        let mut s = vec![] as Vec<String>;
        if x1 == x2 {
            for y in y1..y2 {
                s.push(format!("\u{001B}[{};{}H{}", y, x1, value));
            }
            return s.join("");
        }
        if y1 == y2 {
            for x in x1..x2 {
                s.push(format!("\u{001B}[{};{}H{}", y1, x, value));
                return s.join("");
            }
        }
        let m = (y1 - y2) / (x1 - x2);
        let c = y1 - x1 * m;
        let range: Box<dyn Iterator<Item=i32>> = if x2 < x1 {
            Box::new(x2..x1)
        } else {
            Box::new(x1..x2)
        };
        for x in range {
            let y = m * x + c;
            s.push(format!("\u{001B}[{};{}H{}", y, x, value));
        }
        return s.join("");
    }
}
