extern crate percent_encoding; // For dealing with whitespace encoding and such

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`');         // Adds relevant characters to encode

pub fn construct_maps_search_url(query: &str) -> String {
    if query == "maps" { // Base case, no query
        let base_maps_url = "https://www.google.com/maps";
        base_maps_url.to_string()
    }
    else { // Maps call with an argument (query[..6] should remove leading five "maps ")
        let encoded_query = utf8_percent_encode(&query[5..], FRAGMENT).to_string(); 
        let maps_search_url = format!("https://www.google.com/maps/search/{}", encoded_query);
        maps_search_url
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_maps_search_url_basic() {
        let fake_query = "maps";
        assert_eq!(
            construct_maps_search_url(fake_query),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_construct_maps_search_url() {
        let fake_query = "maps Vancouver";
        assert_eq!(
            construct_maps_search_url(fake_query),
            "https://www.google.com/maps/search/Vancouver"
        );
    }

    #[test]
    fn test_construct_maps_search_url_with_encoding() {
        // Test with encoded whitespace
        let fake_query = "maps 1200 Burrard Street";
        assert_eq!(
            construct_maps_search_url(fake_query),
            "https://www.google.com/maps/search/1200%20Burrard%20Street"
        );
    }
}