use std::io::*;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io, thread};
use termios::*;

mod surface;

pub struct Engine {
    draw_callback: Option<Box<dyn Fn(&mut surface::Surface)>>,
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

        ctrlc::set_handler(move || {
            // Show cursor
            print!("\u{001B}[?25h");
            tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to
            exit(0);
        })
        .unwrap();

        let mut delta_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let surface = &mut surface::Surface{to_draw: vec![]};
        // Hide cursor
        print!("\u{001B}[?25l");
        loop {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            if current_time - delta_time >= 1000 / 60 {
                self.draw_callback.as_ref().unwrap()(surface);
                delta_time = current_time;
                print!("{}", surface.render());
                surface.cleanup();
            }
        }
    }

    pub fn draw(&mut self, draw_callback: impl Fn(&mut surface::Surface) -> () + 'static) {
        self.draw_callback = Some(Box::new(draw_callback));
    }

    pub fn keyboard_events(&mut self, mut key_callback: impl FnMut(u8) + Send + 'static) {
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
