use super::encode_query;

pub fn construct_youtube_search_url(query: &str, command: &str) -> String {
    if query == command { // Base case, no query
        let base_maps_url = "https://www.youtube.com/";
        base_maps_url.to_string()
    }
    else { // Maps call with an argument (query[..6] should remove leading five "maps ")
        let encoded_query: String = encode_query::encode_search_query(&query, &command);
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
        let fake_command = "yt";
        assert_eq!(
            construct_youtube_search_url(fake_query, fake_command),
            "https://www.youtube.com/"
        );
    }

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "yt Ding dong song";
        let fake_command = "yt";
        assert_eq!(
            construct_youtube_search_url(fake_query, fake_command),
            "https://www.youtube.com/results?search_query=Ding%20dong%20song"
        );
    }
}