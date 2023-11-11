use std::fs;
use std::path::Path;
use clap::{App, Arg};
use peg::error::ParseError;
use peg::str::LineCol;
use serde_json::{Result as JsonResult};
use m3u_cli_parser::{M3UEntry, parse_m3u};

pub fn main() -> Result<(), ParseError<LineCol>> {
  let matches = App::new("M3U CLI Parser")
      .version("0.1.0")
      .author("Bohdan Zveriok")
      .about("A Rust CLI application for parsing M3U playlists and extracting entry titles and URLs")
      .arg(Arg::with_name("import file path")
          .short("p")
          .long("path")
          .help("The path to the .m3u file to parse")
          .required(true)
          .index(1))
      .arg(Arg::with_name("output file path")
          .short("o")
          .long("output")
          .value_name("FILE")
          .help("Sets the output file to save JSON data")
          .required(true)
          .takes_value(true))
      .get_matches();

  let file_path = matches.value_of("import file path").unwrap();
  let path = Path::new(file_path);
  let output_file_path = matches.value_of("output file path").unwrap();

  if path.is_file() {
    match fs::read_to_string(&path) {
      Ok(contents) => {
        let result = parse_m3u(&contents)?;

        result.iter().for_each(|el|{
          println!("{:?}", el)
        });

        save_entries_as_json(result, output_file_path).unwrap();
      }
      Err(err) => {
        eprintln!("Error reading the file: {}", err);
      }
    }
  } else {
    eprintln!("The specified path is not a file or does not exist.");
  }

  Ok(())
}

fn save_entries_as_json(entries: Vec<M3UEntry>, output_file_path: &str) -> JsonResult<()> {
  let entries_json = serde_json::to_string_pretty(&entries)?;

  fs::write(output_file_path, entries_json).unwrap();

  println!("JSON data saved to: {}", output_file_path);

  Ok(())
}