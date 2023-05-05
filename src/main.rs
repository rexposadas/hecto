use std::io::{self, Read, stdout};

use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

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

        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
