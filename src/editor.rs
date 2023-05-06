use std::io;
use std::io::stdout;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// pub means we can access this struct from outside editor.rs
// because we want to use it in main.rs
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    // This is a static method, meaning it doesn't need an instance
    // of the struct to be called. We can call it like this:
    // Editor::default()
    pub fn default() -> Self {
        Self {should_quit: false}
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {

        // The question mark after read_key: If there’s an error, return it,
        // if not, unwrap the value and continue.
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}


fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}


fn die(e: &io::Error) {
    panic!("{}", e);
}