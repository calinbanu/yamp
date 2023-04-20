use ::parser::Parser as MapParser;
use clap::Parser;
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// mapfile path
    #[arg(short, long, value_name = "PATH")]
    mapfile: String,
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    println!("{}", cli.mapfile);
    let file = File::open(cli.mapfile)?;
    let data = std::io::read_to_string(file)?;

    let mut parser = MapParser::new();
    parser.parse(&data);

    Ok(())
}
