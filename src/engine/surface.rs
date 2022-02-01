pub struct Surface {
    pub to_draw: Vec<String>,
}

impl Surface {
    pub fn draw(&mut self, what: &str) {
        self.to_draw.push(String::from(what));
    }
    
    // Combine all escape codes as one string
    // That string will be print at terminal output entirely
    pub fn render(&mut self) -> String {
        let res = self
            .to_draw
            .iter()
            .map(|m| m.to_string())
            .reduce(|accum, item| format!("{}{}", accum, item));

        return Some(res).unwrap().unwrap();
    }

    pub fn cleanup(&mut self) {
        self.to_draw.clear();
    }
}
