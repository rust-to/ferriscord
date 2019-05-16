// originally from https://github.com/mgattozzi/ferris-says/blob/master/src/lib.rs
// ferris-says crate

extern crate smallvec;

use smallvec::*;
use std::iter::repeat;

// Constants! :D
const ENDSL: &[u8] = b"| ";
const ENDSR: &[u8] = b" |\n";
#[cfg(not(feature = "clippy"))]
const FERRIS: &[u8] = br#"
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
"#;
#[cfg(feature = "clippy")]
const CLIPPY: &[u8] = br#"
              \
               \
                  __
                 /  \
                 |  |
                 @  @
                 |  |
                 || |/
                 || ||
                 |\_/|
                 \___/
"#;
const NEWLINE: u8 = '\n' as u8;
const SPACE: u8 = ' ' as u8;
const DASH: u8 = '-' as u8;

// A decent number for SmallVec's Buffer Size, not too large
// but also big enough for most inputs
const BUFSIZE: usize = 2048;

// We need a value to add to the width which includes
// the length of ENDSL and ENDSR for the proper size
// calculation of the bar at the top and bottom of the
// box
const OFFSET: usize = 4;

pub fn say(input: &[u8], width: usize) -> Result<(String)> {
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // The top and bottom bar for the text box is calculated once here
    let bar_buffer: Vec<u8> = repeat(DASH).take(width + OFFSET).collect();

    write_buffer.extend_from_slice(&bar_buffer);
    write_buffer.push(NEWLINE);
    for i in input.split(|x| *x == '\n' as u8) {
        for j in i.chunks(width) {
            write_buffer.extend_from_slice(ENDSL);
            write_buffer.extend_from_slice(j);

            for _ in 0..width - j.len() {
                write_buffer.push(SPACE);
            }

            write_buffer.extend_from_slice(ENDSR);
        }
    }
    write_buffer.extend_from_slice(&bar_buffer);
    #[cfg(feature = "clippy")]
    write_buffer.extend_from_slice(CLIPPY);
    #[cfg(not(feature = "clippy"))]
    write_buffer.extend_from_slice(FERRIS);

    Ok(write_buffer)
}
