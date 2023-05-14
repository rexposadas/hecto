#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
clippy::missing_docs_in_private_items,
clippy::implicit_return,
clippy::shadow_reuse,
clippy::print_stdout,
clippy::wildcard_enum_match_arm,
clippy::else_if_without_else,
clippy::unnecessary_cast
)]

mod document;
mod editor;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {

    // The default is there so that, if we want to add more fields
    // the Editor struct we don't have to change main or any other
    // location where we instantiate Editor.
    Editor::default().run();
}