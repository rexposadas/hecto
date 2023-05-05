use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("byte code: {:?} \r", b);
        } else {
            println!("byte code: {:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}
