mod terminal;
mod editor;
mod row;
mod document;

pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;

fn main() {

    // The default is there so that, if we want to add more fields
    // the Editor struct we don't have to change main or any other
    // location where we instantiate Editor.
    Editor::default().run();
}