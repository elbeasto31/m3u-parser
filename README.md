# M3U CLI Parser

## Table of Contents

- [Description](#description)
- [Parsing process](#parsing-process)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Result usage](#results-usage)
- [License](#license)
- [Contributing](#contributing)

## Description

M3U Parser CLI is a Rust command-line application that parses M3U playlists, extracts valuable information such as entry titles and URLs, and provides the parsed results for further use. It is designed for users who need to work with M3U playlists, whether for media playback, playlist management, or other purposes

## Parsing Process

### 1. M3U File Structure
An M3U file typically starts with `#EXTM3U` and consists of entries, each starting with an `#EXTINF` tag followed by metadata like duration and title, and the corresponding media file path.

`#EXTM3U
#EXTINF:111, Sample artist name - Sample track title
C:\Music\SampleMusic.mp3
#EXTINF:222,Example Artist name - Example track title
C:\Music\ExampleMusic.mp3
`

### 2. Parsing Rules
The parser uses the PEG crate to define rules that match the structure of each entry. The rules identify and extract relevant information such as duration, title, and file path.

### 3. Resulting Data Structure
The parsed information is structured as M3UEntry objects, capturing details like title, duration, and file path.

`struct M3UEntry { 
    title: String,
    duration: String,
    path: String,
}`

## Installation

To use this parser, follow these steps:

$ git clone https://github.com/elbeasto31/rust_parser.git
$ cd rust_parser
$ cargo build` 

## Usage

To use the M3U Parser CLI, you can install it and run it from your command line. You can specify the path to the M3U file you want to parse as a command-line argument. The parsed results will be displayed in the console and saved as a JSON by specified path.

## Examples

`cargo run assets/test.m3u -o file.json`

## Results Usage
The parsed results can be utilized from a technical perspective in the following ways:

- **Playlist Management**: The parsed M3UEntry objects can be stored in a data structure or database for efficient playlist management. You can implement features like adding, removing, or reordering entries within the playlist.

- **Metadata Extraction and Cataloging**: Utilize the extracted metadata such as track titles, durations for cataloging media files. This information can be stored in a database or used to generate reports on media content.

- **Integration with Other Systems**: The parsed results can be integrated with other systems or applications written in Rust or other programming languages. This enables seamless interoperability with existing software.

Feel free to customize the M3U parser to suit your specific technical requirements and integrate the parsed results into your Rust applications seamlessly.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Make your changes and test them thoroughly.
4.  Submit a pull request with a clear description of the changes.

#### Crates.io URL: https://crates.io/crates/m3u_cli_parser
