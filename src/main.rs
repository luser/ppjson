extern crate json_color;
extern crate pager;

use json_color::Colorizer;
use pager::Pager;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn read_input<R>(reader: &mut R) -> io::Result<String>
    where R: Read,
{
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

fn work() -> io::Result<()> {
    let input = match env::args_os().nth(1) {
        Some(filename) => {
            let mut f = File::open(&filename)?;
            read_input(&mut f)?
        }
        None => {
            let mut stdin = io::stdin();
            read_input(&mut stdin)?
        }
    };
    Pager::with_pager("less -FRSX").setup();
    let colorizer = Colorizer::arbitrary();
    let mut stdout = io::stdout();
    colorizer.colorize_to_writer(&input, &mut stdout)?;
    println!("");
    Ok(())
}

fn main() {
    match work() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
