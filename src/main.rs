mod terminal;
mod editor;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {

    // The default is there so that, if we want to add more fields
    // the Editor struct we don't have to change main or any other
    // location where we instantiate Editor.
    Editor::default().run();
}