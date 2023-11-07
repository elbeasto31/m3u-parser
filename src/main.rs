use std::fs;
use std::path::Path;
use clap::{App, Arg};
use peg::error::ParseError;
use peg::str::LineCol;
use parser_zv::parse_m3u;

pub fn main() -> Result<(), ParseError<LineCol>> {
  let matches = App::new("M3U CLI Parser")
      .version("1.0")
      .author("Bohdan Zveriok")
      .about("A Rust CLI application for parsing M3U playlists and extracting entry titles and URLs")
      .arg(Arg::with_name("file")
          .help("The path to the file to print")
          .required(true)
          .index(1))
      .get_matches();

  let file_path = matches.value_of("file").unwrap();
  let path = Path::new(file_path);

  if path.is_file() {
    match fs::read_to_string(&path) {
      Ok(contents) => {
        let result = parse_m3u(&contents)?;

        result.iter().for_each(|el|{
          println!("{:?}", el)
        });
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