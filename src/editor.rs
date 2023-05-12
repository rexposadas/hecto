use crate::Terminal;
use std::io::{self, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {

    // Called in main to start the editor. Whil this is alive,
    // we are inside the editor.
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        // keeps the editor running until we quit.
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            // Set when we press Ctrl+Q.
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    // Instantiated in main to get a handle to the editor. then we call
    // run().
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Keep planting.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("Error: {e:?}\r");
}