use std::io::{self, Read};
// Take contiunous input from user and format it.

// Sürekli olarak girdi al. - Take continous input.
pub fn take_continous_input() {
    // Her ASCII karakter 1 byte (8 bits).
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c == 'q' {
            break;
        }
    }
}
