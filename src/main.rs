use json_color::Colorizer;
use pager::Pager;
use serde_json::{Serializer, Deserializer};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process;

fn handle_input<R>(reader: &mut R) -> io::Result<()>
where
    R: Read,
{
    // I don't like this but what can you do?
    colored::control::set_override(true);
    //TODO: figure out something useful for Windows.
    Pager::with_pager("less -FRSX").setup();
    let colorizer = Colorizer::arbitrary();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let mut deserializer = Deserializer::from_reader(reader);
    let mut serializer = Serializer::with_formatter(&mut lock, colorizer);
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    serializer.into_inner().flush()?;
    writeln!(&mut lock, "")?;
    Ok(())
}

fn work() -> io::Result<()> {
    match env::args_os().nth(1) {
        Some(ref arg) if arg == "--help" => {
            print!(
                "JSON pretty printer

USAGE:
    ppjson [PATH]

Pretty print the contents of the JSON file at PATH including colorizing the output
and piping it through a pager. If PATH is not provided read a JSON file from stdin.
"
            );
        }
        Some(ref arg) if arg == "--version" => {
            println!("ppjson {}", env!("CARGO_PKG_VERSION"));
        }
        Some(filename) => {
            let mut f = BufReader::new(File::open(&filename)?);
            handle_input(&mut f)?;
        }
        None => {
            let mut stdin = io::stdin();
            handle_input(&mut stdin)?;
        }
    }
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
