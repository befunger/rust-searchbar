// Custom behaviour for Weather-related queries


// Would be neat to have a one-glove-fits-all solution here
// For now, just encoding common locations for own use and otherwise defaulting to google
use super::google;

pub fn construct_weather_url(query: &str, command: &str) -> String {
    let cmd_len = command.len();
    let lowercased = query[cmd_len+1..].to_ascii_lowercase();
    //let location = query[command_length+1..]; // Cuts out the "weather " part of query
    let weather_redirect_url = match lowercased.as_str() {
        "vancouver" => "https://weather.gc.ca/forecast/hourly/bc-74_metric_e.html".to_string(),
        "victoria" => "https://weather.gc.ca/forecast/hourly/bc-85_metric_e.html".to_string(),
        "stockholm" => "https://www.klart.se/se/stockholms-l%C3%A4n/v%C3%A4der-stockholm/timmar/".to_string(),
        _ => google::construct_google_search_url(&query) // Standard google search for "weather <...>"
    };
    weather_redirect_url.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_cached_query() {
        // Test for a hard-coded location
        let fake_query = "weather vancouver";
        let fake_command = "weather";
        assert_eq!(
            construct_weather_url(fake_query, fake_command),
            "https://weather.gc.ca/city/pages/bc-74_metric_e.html"
        );
    }

    #[test]
    fn test_weather_cached_query_capitalised() {
        // Test for case-insensitivity
        let fake_query = "weather Vancouver";
        let fake_command = "weather";
        assert_eq!(
            construct_weather_url(fake_query, fake_command),
            "https://weather.gc.ca/city/pages/bc-74_metric_e.html"
        );
    }

    #[test]
    fn test_weather_random_query() {
        // Test for a generic location
        let fake_query = "weather sunny wells maine";
        let fake_command = "weather";
        assert_eq!(
            construct_weather_url(fake_query, fake_command),
            google::construct_google_search_url(fake_query)
        );
    }
}