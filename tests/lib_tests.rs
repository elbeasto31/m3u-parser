extern crate peg;
use parser_zv::parse_m3u;
use parser_zv::M3UEntry;

#[test]
fn test_parse_m3u_with_valid_input() {
    let m3u_content = "#EXTM3U\n#EXTINF:22,Sample Song\nhttp://example.com/sample.mp3\n";
    let result = parse_m3u(m3u_content);
    assert_eq!(
        result,
        Ok(vec![M3UEntry {
            title: "Sample Song".to_string(),
            duration: "22".to_string(),
            path: "http://example.com/sample.mp3".to_string()
        }])
    );
}

#[test]
fn test_parse_m3u_with_invalid_input() {
    let m3u_content = "This is not a valid M3U file.";
    let result = parse_m3u(m3u_content);
    assert!(result.is_err(), "Expected an error");
}

#[test]
fn test_parse_m3u_with_multiple_entries() {
    let m3u_content = "#EXTM3U\n#EXTINF:31,Sample1\nhttp://example.com/sample1.mp3\n#EXTINF:2,Sample2\nhttp://example.com/sample2.mp3\n";
    let result = parse_m3u(m3u_content);
    assert_eq!(
        result,
        Ok(vec![
            M3UEntry {
                title: "Sample1".to_string(),
                duration: "31".to_string(),
                path: "http://example.com/sample1.mp3".to_string()
            },
            M3UEntry {
                title: "Sample2".to_string(),
                duration: "2".to_string(),
                path: "http://example.com/sample2.mp3".to_string()
            }
        ])
    );
}

#[test]
fn test_parse_m3u_with_whitespace() {
    let m3u_content = "  #EXTM3U  \n  #EXTINF:0, Sample Song  \n  http://example.com/sample.mp3  \n";
    let result = parse_m3u(m3u_content);
    assert_eq!(
        result,
        Ok(vec![M3UEntry {
            title: " Sample Song  ".to_string(),
            duration: "0".to_string(),
            path: "  http://example.com/sample.mp3  ".to_string()
        }])
    );
}