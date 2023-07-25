extern crate percent_encoding; // For dealing with whitespace encoding and such

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`');         // Adds relevant characters to encode

pub fn construct_youtube_search_url(query: &str) -> String {
    if query == "yt" { // Base case, no query
        let base_maps_url = "https://www.youtube.com/";
        base_maps_url.to_string()
    }
    else { // Maps call with an argument (query[..6] should remove leading five "maps ")
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string(); 
        let maps_search_url = format!("https://www.youtube.com/results?search_query={}", encoded_query);
        maps_search_url
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_search_url_basic() {
        let fake_query = "yt";
        assert_eq!(
            construct_youtube_search_url(fake_query),
            "https://www.youtube.com/"
        );
    }

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "yt Ding dong song";
        assert_eq!(
            construct_youtube_search_url(fake_query),
            "https://www.youtube.com/results?search_query=Ding%20dong%20song"
        );
    }
}