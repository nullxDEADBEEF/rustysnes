mod cpu;
mod snes;
use std::env;

use crate::snes::Snes;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let mut snes = Snes::new();
            snes.run();
        }
        _ => eprintln!("Usage: cargo run <ROM>"),
    }
}
