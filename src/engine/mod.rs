use std::io::*;
use std::{io, thread};
use termios::*;

pub struct Engine {
    draw_callback: Option<Box<dyn Fn()>>,
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            draw_callback: None,
        };
    }

    pub fn init(&mut self) {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

        print!("\u{001B}[?25l");
        loop {
            self.draw_callback.as_ref().unwrap()();
            thread::sleep(std::time::Duration::from_millis(16));
        }
        print!("\u{001B}[?25h");
        tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to
    }

    pub fn draw(&mut self, draw_callback: impl Fn() -> () + 'static) {
        self.draw_callback = Some(Box::new(draw_callback));
    }

    pub fn keyboard_events(&mut self, mut key_callback: impl FnMut(u8) + Send + 'static) {
        //ctrlc::set_handler(|| {
        //    print!("\u{001B}[?25h");
        //    exit(0);
        //})
        // .unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1]; // read exactly one byte
        thread::spawn(move || loop {
            stdout.lock().flush().unwrap();
            reader.read_exact(&mut buffer).unwrap();
            if !buffer.is_empty() {
                 key_callback(buffer[0]);
            }
        });
    }
}
