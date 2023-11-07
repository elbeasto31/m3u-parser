extern crate peg;

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq)]
pub struct M3UEntry {
    pub title: String,
    pub duration: String,
    pub path: String,
}

peg::parser! {
    grammar m3u_parser() for str {
        rule m3u() -> Vec<M3UEntry>
            = entries:line()* { entries }

        rule line() -> M3UEntry
            = _ "#EXTINF:" duration:digits() "," title:$(!"\n" [_]*) "\n" path:$(!"\n" [_]*) "\n" {
                M3UEntry { title: title.into(), duration: duration, path: path.into() }
            }

        rule digits() -> String
            = digits:$(['0'..='9']+) { digits.to_string() }

        // Helper rule for whitespace and comments
        rule _()
            = quiet!{ [' ' | '\t' | '\r']* }

        // The parse_m3u function to parse an M3U string
        pub rule parse_m3u() -> Vec<M3UEntry>
            = m3u()
    }
}

pub fn parse_m3u(m3u_content: &str) -> Result<Vec<M3UEntry>, ParseError<LineCol>> {
    m3u_parser::parse_m3u(m3u_content)
}