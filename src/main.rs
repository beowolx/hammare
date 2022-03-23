use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b11010
    // 0b1111010
    // byte & 0b0001_1111
}

fn main() {
    let _stdout = stdout()
        .into_raw_mode()
        .expect("Error while entering raw mode");

    for b in io::stdin().bytes() {
        let b = b.expect("Error while unpacking stdin bytes");
        let c = b as char;
        if c.is_control() {
            println!("{:#b}", b);
            println!("{b} \r")
        } else {
            println!("{:#b}", b);
            println!("{b}, ({c})\r")
        }
        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
