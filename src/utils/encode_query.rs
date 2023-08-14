// Implements encoding of queries with special characters (whitespace, #, etc.)
extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`')
        .add(b'#');         // Adds relevant characters to encode

pub fn encode_search_query(full_query: &str, command: &str) -> String{
    let command_length = command.len();
    if command_length == 0 {
        let encoded_query = utf8_percent_encode(&full_query, FRAGMENT).to_string();
        encoded_query
    }
    else {
        let encoded_query = utf8_percent_encode(&full_query[command_length+1..], FRAGMENT).to_string();
        encoded_query
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_search_query() {
        let fake_full_query = "maps Bung Street 16";
        let fake_command = "maps";
        assert_eq!(
            encode_search_query(fake_full_query, fake_command),
            "Bung%20Street%2016"
        );
    }

    #[test]
    fn test_encode_search_query_with_hashtag() {
        let fake_full_query = "yt C# tutorial delegates";
        let fake_command: &str = "yt";
        assert_eq!(
            encode_search_query(fake_full_query, fake_command),
            "C%23%20tutorial%20delegates"
        );
    }
}