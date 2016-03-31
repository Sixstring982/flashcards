extern crate rand;
use rand::{thread_rng, Rng};

use std::convert::From;
use std::env;
use std::fs::File;
use std::io::{Read, self};

struct FlashCard {
    pub word: String,
    pub def: String
}

impl From<String> for FlashCard {
    fn from(s: String) -> Self {
        let mut fc = FlashCard { word: "".to_string(), def: "".to_string() };
        let mut i = 0;
        for p in s.split("=") {
            if i == 0 {
                i += 1;
                fc.word = p.to_string();
            } else {
                fc.def = p.to_string();
            }
        }
        return fc;
    }
}

fn main() {
    let mut filename = "".to_string();
    for arg in env::args() {
        filename = arg;
    }

    let mut f = File::open(filename)
        .expect("Can't open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Can't read file");

    let mut v: Vec<FlashCard> = Vec::new();
    for line in buffer.lines() {
        /* Ignore comments and blank lines */
        let st = line.to_string();
        if st.chars().nth(0) != Some('#') && st.len() > 0 {
            v.push(FlashCard::from(st.to_string()));
        }
    }

    let mut rng = thread_rng();
    let mut i = 0;
    rng.shuffle(&mut v);

    let stdin = io::stdin();

    println!("{}", v[i].word);
    while let Ok(_line) = stdin.read_line(&mut buffer) {
        println!("{}", v[i].def);
        i += 1;
        if i >= v.len() {
            i = 0;
            rng.shuffle(&mut v);
        }
        println!("");
        println!("{}", v[i].word);
    }
}
