use std::io;

use rand::prelude::*;

const ICE_CHARS: [u8; 42] = [
    24, 25, 26, 27, 31, 32, 33, 38, 39, 
    43, 44, 45, 50, 51, 60, 61, 62, 63, 
    67, 68, 69, 74, 75, 79, 80, 81, 86, 
    87, 96, 97, 98, 99, 103, 104, 105, 
    110, 111, 115, 116, 117, 122, 123,
];

const BEGIN: &'static str = "\x1b[";
const END: &'static str = "\x1b[0m";

fn main() {
    let mut rng = thread_rng();
    let mut idx: usize = rng.gen_range(0..42); // ice char index
    
    let stdin = io::stdin();
    let mut chunk = String::with_capacity(150);

    loop {
        let bytes = stdin.read_line(&mut chunk);
        match bytes {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }

                let chars: Vec<char> = chunk.chars().collect();
                for c in chars {
                    print!("{}", fmt(c, idx));
                    idx += 1;
                    if idx == 42 {
                        idx = 0;
                    }
                }
                print!(" ");

            },
            Err(e) => {
                panic!("{}", e);
            },
        }
    }
}

fn fmt(c: char, r: usize) -> String {
    return format!(
        "{}38;5;{}m{}{}",
        BEGIN,
        ICE_CHARS[r], 
        c,
        END,
    )
}