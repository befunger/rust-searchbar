pub mod google;
pub mod maps;
pub mod youtube;
//TODO: add reddit (subreddit)

pub fn get_command_from_query_string(query_string: &str) -> &str {
    // Returns command from query string (initial text before a whitespace)
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    // If no spacebar found, return whole string as command
    return &query_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        // Test with command + parameter
        let actual = get_command_from_query_string
        ("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}