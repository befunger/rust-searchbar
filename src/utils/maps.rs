// Custom behaviour for Google Maps queries
use super::encode_query;

pub fn construct_maps_search_url(query: &str, command: &str) -> String {
    if query == command { // Base case, no query
        let base_maps_url = "https://www.google.com/maps";
        base_maps_url.to_string()
    }
    else { // Maps call with an argument (query[..6] should remove leading five "maps ")
    let encoded_query: String = encode_query::encode_search_query(&query, &command);
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
        let fake_command = "maps";
        assert_eq!(
            construct_maps_search_url(fake_query, fake_command),
            "https://www.google.com/maps"
        );
    }

    #[test]
    fn test_construct_maps_search_url() {
        let fake_query = "maps Vancouver";
        let fake_command = "maps";
        assert_eq!(
            construct_maps_search_url(fake_query, fake_command),
            "https://www.google.com/maps/search/Vancouver"
        );
    }

    #[test]
    fn test_construct_maps_search_url_with_encoding() {
        // Test with encoded whitespace
        let fake_query = "maps 1200 Burrard Street";
        let fake_command = "maps";
        assert_eq!(
            construct_maps_search_url(fake_query, fake_command),
            "https://www.google.com/maps/search/1200%20Burrard%20Street"
        );
    }
}