use super::encode_query;

pub fn construct_reddit_url(query: &str, command: &str) -> String {
    if query == command { // Base case, no query
        let base_maps_url = "https://www.reddit.com/";
        base_maps_url.to_string()
    }
    else {
        let encoded_query = encode_query::encode_search_query(query, command);
        let reddit_url = format!("https://www.reddit.com/r/{}", encoded_query);
        reddit_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reddit_query_sub() {
        let fake_query = "reddit sweden";
        let fake_command = "reddit";
        assert_eq!(
            construct_reddit_url(fake_query, fake_command),
            "https://www.reddit.com/r/sweden"
        );
    }

    #[test]
    fn test_reddit_query_blank() {
        // Test with encoded whitespace
        let fake_query = "reddit";
        let fake_command = "reddit";
        assert_eq!(
            construct_reddit_url(fake_query, fake_command),
            "https://www.reddit.com/"
        );
    }
}