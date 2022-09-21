use std::{
    env::args,
    fs::File,
    os::unix::prelude::FileExt, 
    io::{self, Write, Read}
};

static BLUES: [u8; 42] = [
    24, 25, 26, 27, 31, 32, 33, 38, 39,
	43, 44, 45, 50, 51, 60, 61, 62, 63,
	67, 68, 69, 74, 75, 79, 80, 81, 86,
	87, 96, 97, 98, 99, 103, 104, 105,
	110, 111, 115, 116, 117, 122, 123
];

static GREENS: [u8; 27] = [
    2,    10,  14,  22,  28,
    29,   34,  36,  40,  42,
    76,   77,  78,  82,  83,
    112, 113, 114, 118, 119,
    148, 149, 154, 155, 184,
    190, 191
];

static REDS: [u8; 14] = [
    1, 9, 52, 88, 124,
    160, 196, 125, 161, 197,
    162, 198, 167, 203
];


fn get_seed(colors: &Vec<u8>) -> usize {
    let f = File::open("/dev/urandom")
                        .expect("issue opening from /dev/urandom");

    let mut buf: [u8; 5] = [0; 5];
    f.read_at(&mut buf, 0).expect("issue reading from /dev/urandom");

    let mut sum: usize = 0;
    for n in buf {
        sum += n as usize;
    }

    return sum % colors.len();
}

fn fmt_char(c: char, idx: usize, colors: &Vec<u8>) -> String {
    let i = colors[idx]; // color byte
    return format!("\x1b[38;5;{i}m{c}\x1b[0m");
}

const ERR_WR: &'static str = "err writing to stdout";

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let mut colors = BLUES.to_vec();
    if args.len() != 0 {
        match args[0].as_str() {
        "-g" => { colors = GREENS.to_vec() },
        "-r" => { colors =   REDS.to_vec() },
        _ => {}
        }
    }

    let mut idx = get_seed(&colors);
    let mut buf = String::with_capacity(500);

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        match stdin.read_to_string(&mut buf) {
            Ok(n)  => {
                if n == 0 {
                    break; 
                }
                
                for c in buf.chars() {
                    let c = fmt_char(c, idx, &colors);
                    stdout.write(c.as_bytes())
                          .expect(ERR_WR);

                    idx += 1;
                    if idx == colors.len() { 
                        idx = 0;
                    }
                }

                stdout.flush().expect("stdout flush err");
            },

            Err(e) => { println!("err => {:?}", e) }
        }
    }
}


