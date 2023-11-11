extern crate peg;

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct M3UEntry {
    pub title: String,
    pub duration: String,
    pub path: String,
}

peg::parser! {
    pub grammar m3u_parser() for str {
        rule m3u() -> Vec<M3UEntry>
            = _ "#EXTM3U" _ (!"#EXTINF:" [_])* entries:(line())* { entries }

        rule format_duration() -> String
            = seconds:digits() {
                let secs: u64 = seconds.parse().unwrap_or(0);
                let minutes = secs / 60;
                let remaining_seconds = secs % 60;
                format!("{:02}:{:02}", minutes, remaining_seconds)
            }

       pub rule url() -> String
            = protocol:$(['a'..='z' | 'A'..='Z']+) "://" rest:$([^'\n']* "\n") {
                format!("{}://{}", protocol, rest)
            }

        pub rule path() -> String
            = rest:$([^'\n']* "\n") { rest.to_string() }

        rule line() -> M3UEntry
            = _ "#EXTINF:" duration:format_duration() "," title:$([^'\n']* "\n") path:(url() / path()) {
                M3UEntry {
                    title: title.trim_end_matches('\n').trim_end_matches('\r').to_string(),
                    duration: duration,
                    path: path.trim_end_matches('\n').trim_end_matches('\r').to_string(),
                }
            }

        rule digits() -> String
            = digits:$(['0'..='9']+) { digits.to_string() }

        rule _()
            = quiet!{ [' ' | '\t' | '\r']* }

        pub rule parse_m3u() -> Vec<M3UEntry>
            = m3u()
    }
}

pub fn parse_m3u(m3u_content: &str) -> Result<Vec<M3UEntry>, ParseError<LineCol>> {
    m3u_parser::parse_m3u(m3u_content)
}