use json_color::Colorizer;
use pager::Pager;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn read_input<R>(reader: &mut R) -> io::Result<String>
where
    R: Read,
{
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

fn work() -> io::Result<()> {
    let input = match env::args_os().nth(1) {
        Some(ref arg) if arg == "--help" => {
            print!(
                "JSON pretty printer

USAGE:
    ppjson [PATH]

Pretty print the contents of the JSON file at PATH including colorizing the output
and piping it through a pager. If PATH is not provided read a JSON file from stdin.
"
            );
            return Ok(());
        }
        Some(ref arg) if arg == "--version" => {
            println!("ppjson {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Some(filename) => {
            let mut f = File::open(&filename)?;
            read_input(&mut f)?
        }
        None => {
            let mut stdin = io::stdin();
            read_input(&mut stdin)?
        }
    };
    // I don't like this but what can you do?
    colored::control::set_override(true);
    //TODO: figure out something useful for Windows.
    Pager::with_pager("less -FRSX").setup();
    let colorizer = Colorizer::arbitrary();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    colorizer.colorize_to_writer(&input, &mut lock)?;
    writeln!(&mut lock, "")?;
    Ok(())
}

fn main() {
    match work() {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("{}", e);
            }
            process::exit(1);
        }
    }
}
