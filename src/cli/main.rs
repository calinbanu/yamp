use log::LevelFilter;
use parser::excelwriter::{ExcelWriter, ToExcelWriter};
// use ::parser::xmlwriter::XmlWriter;
use ::parser::xmlwriter::{ToXmlWriter, XmlWriter};
use ::parser::Parser as MapParser;
use clap::Parser as CliParser;
// use parser::xmlwriter::ToXmlWriter;
use std::{fs::File, io::Write};

#[derive(CliParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to input Map file
    #[arg(short, long, value_name = "PATH")]
    mapfile: String,

    /// Path to output XLSX file. If not specified, outputs to "mapfile.xlsx"
    #[arg(
        long,
        value_name = "PATH",
        default_value = "mapfile.xlsx",
        default_missing_value = "mapfile.xlsx",
        require_equals = true,
        num_args = 0..=1
    )]
    xlsfile: Option<String>,

    /// Path to output XML file. If not specified, outputs to "mapfile.xml"
    #[arg(
        long,
        value_name = "PATH",
        default_missing_value = "mapfile.xml",
        require_equals = true,
        num_args = 0..=1,
    )]
    xmlfile: Option<String>,

    /// Set log level
    #[arg(short, long, value_name = "LEVEL", default_value = "error", value_parser= ["off", "0", "error", "1", "warn", "2", "info", "3", "debug", "4", "trace", "5"])]
    loglevel: String,
}

fn config_log_level(loglevel: &str) {
    let level = match loglevel {
        "off" | "0" => LevelFilter::Off,
        "error" | "1" => LevelFilter::Error,
        "warn" | "2" => LevelFilter::Warn,
        "info" | "3" => LevelFilter::Info,
        "debug" | "4" => LevelFilter::Debug,
        "trace" | "5" => LevelFilter::Trace,
        _ => panic!("Invalid Log Level!"),
    };

    env_logger::Builder::new().filter(None, level).init();
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    config_log_level(&cli.loglevel);

    let file = File::open(&cli.mapfile)?;
    let data = std::io::read_to_string(file)?;

    let parser = MapParser::parse(&data);

    if let Some(path) = cli.xmlfile {
        let file: Box<dyn Write> = match path.eq("stdout") {
            true => Box::new(std::io::stdout()),
            false => Box::new(std::fs::File::create(&path)?),
        };

        let mut xmlwriter = XmlWriter::new(file, &cli.mapfile);
        xmlwriter.set_skip_data(true);
        parser.to_xml_writer(&mut xmlwriter);
    }

    if let Some(path) = cli.xlsfile {
        let mut excelwriter = ExcelWriter::new(&path).unwrap();
        parser.to_excel_writer(&mut excelwriter);
    }

    Ok(())
}
