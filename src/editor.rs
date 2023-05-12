use crate::Terminal;
use std::io::{self, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,

    // This is here, and not the Terminal struct, because we want to
    // keep track of the cursor position in *our current document*, which
    // is different from the terminal.
    cursor_position: Position,

}

pub struct Position {
    // and not u16 because that's too small. we want to take into account large documents.
    // usize depends on the machine's architecture.
    pub x: usize,
    pub y: usize,
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
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Keep planting.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key){
        let Position { mut y, mut x } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }

    fn draw_welcome_message(&self){
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }


    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height-1  {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("Error: {e:?}\r");
}