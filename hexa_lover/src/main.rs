use structopt::StructOpt;

use std::io;
use std::io::prelude::*;
use std::fs::File;


/// Well i do not really know what to write here.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let mut content = File::open(&args.path)?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    content.read(&mut buffer)?;
    println!("{:X?}", buffer);

    Ok(())
}