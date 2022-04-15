use std::io::{self, Read};
use std::os::unix::io::RawFd;
use termios::*;

// Take contiunous input from user and format it.

// Sürekli olarak girdi al. - Take continous input.
fn enable_raw_mode() {
    let fd: RawFd = 0;
    assert_eq!(fd, 0);
    let mut termios = Termios::from_fd(fd);
    match termios {
        Ok(mut termio) => tcgetattr(fd, &mut termio),
        Err() => (),
    }
}

pub fn take_continous_input() {
    // Her ASCII karakter 1 byte.
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c == 'q' {
            break;
        }
    }
}
