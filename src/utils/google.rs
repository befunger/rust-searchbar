use super::encode_query;

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = encode_query::encode_search_query(query, ""); // No command results in google query
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);

    google_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        // Test with encoded whitespace
        let fake_query = "hello world";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://google.com/search?q=hello%20world"
        );
    }
}