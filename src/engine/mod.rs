use std::io::*;
use std::{io, thread};
use termios::*;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        return Engine {};
    }

    pub fn init(&self) {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();


        let (mut x, mut y) = (10, 10);

        print!("\u{001B}[?25l");
        loop {
            print!("\u{001B}[1;1H\u{001B}[2J");
            print!("\u{001B}[{};{}H", x, y);
            print!("*");
        }

        print!("\u{001B}[?25h");
        tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to
    }
}

pub fn keyboard_events(key_callback: fn(u8)) {
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
            key_callback(buffer[0])
        }
    });
}
