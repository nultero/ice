use std::{
    fs::File, 
    os::unix::prelude::FileExt, 
    io::{self, Write}
};

static COLORS: [u8; 42] = [
    24, 25, 26, 27, 31, 32, 33, 38, 39,
	43, 44, 45, 50, 51, 60, 61, 62, 63,
	67, 68, 69, 74, 75, 79, 80, 81, 86,
	87, 96, 97, 98, 99, 103, 104, 105,
	110, 111, 115, 116, 117, 122, 123
];


fn get_seed() -> usize {
    let f = File::open("/dev/urandom")
                        .expect("issue opening from /dev/urandom");

    let mut buf: [u8; 5] = [0; 5];
    f.read_at(&mut buf, 0).expect("issue reading from /dev/urandom");

    let mut sum: usize = 0;
    for n in buf {
        sum += n as usize;
    }

    return sum % COLORS.len();
}

const S: &str = "\x1b[38;5;";
const E: &str = "\x1b[0m";

fn fmt_char(c: char, idx: usize) -> String {
    let i = COLORS[idx]; // color byte
    return format!("{S}{i}m{c}{E}");
}

fn main() {
    let mut idx = get_seed();
    let mut buf = String::with_capacity(150);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        match stdin.read_line(&mut buf) {
            Ok(n)  => {
                if n == 0 { break; }

                for c in buf.chars() {
                    print!("{}", fmt_char(c, idx));
                    idx += 1;
                    if idx == 42 { 
                        idx = 0;
                    }
                }

                stdout.flush().expect("stdout flush err");
            },

            Err(e) => { println!("{:?}", e) }
        }
    }
}


