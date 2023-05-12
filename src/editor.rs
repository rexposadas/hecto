use crate::Document;
use crate::Terminal;
use crate::Row;

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

    offset: Position,
    document: Document,
}

#[derive(Default)]
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
        let args: Vec<String> = std::env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1]; // args[0] is the name of the program.
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };


        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Keep planting.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position{
                // saturation means we go beyond the allowed value for the type.
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self){
        let Position {x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height){
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width){
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }


    fn move_cursor(&mut self, key: Key){
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>
                {
                    if y < height {
                        y = y.saturating_add(1)
                    }
                },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
             if x < width {
                 x = x.saturating_add(1)
             }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
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

    pub fn draw_row(&self, row:&Row){
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;


        let row = row.render(start, end);
        println!("{}\r", row);
    }


    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height-1  {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y){
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
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