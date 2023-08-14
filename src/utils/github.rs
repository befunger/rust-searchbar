use super::encode_query;

pub fn construct_github_url(query: &str, command: &str) -> String {
    let encoded_query = encode_query::encode_search_query(query, command);
    let github_user_url = format!("https://github.com/{}", encoded_query);
    github_user_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_query_repo() {
        let fake_query = "git befunger/redblacktree";
        let fake_command = "git";
        assert_eq!(
            construct_github_url(fake_query, fake_command),
            "https://github.com/befunger/redblacktree"
        );
    }

    #[test]
    fn test_github_query_profile() {
        // Test with encoded whitespace
        let fake_query = "git befunger";
        let fake_command = "git";
        assert_eq!(
            construct_github_url(fake_query, fake_command),
            "https://github.com/befunger"
        );
    }
}