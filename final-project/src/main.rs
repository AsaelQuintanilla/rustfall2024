mod config;
mod checker;
mod reporter;
mod models;

use crate::config::Config;
use crate::checker::check_websites;
use crate::reporter::report_statuses;

fn main() {
    let config = Config::new();

    // let urls = vec![
    //     "https://www.google.com".to_string(),
    //     "https://www.rust-lang.org".to_string(),
    // ];
    let urls = vec![
        "https://www.google.com".to_string(),
        "https://www.rust-lang.org".to_string(),
        "https://www.microsoft.com".to_string(),
        "https://www.apple.com".to_string(),
        "https://www.amazon.com".to_string(),
        "https://www.facebook.com".to_string(),
        "https://www.twitter.com".to_string(),
        "https://www.linkedin.com".to_string(),
        "https://www.instagram.com".to_string(),
        "https://www.youtube.com".to_string(),
        "https://www.wikipedia.org".to_string(),
        "https://www.stackoverflow.com".to_string(),
        "https://www.github.com".to_string(),
        "https://www.medium.com".to_string(),
        "https://www.reddit.com".to_string(),
        "https://www.quora.com".to_string(),
        "https://www.twitch.tv".to_string(),
        "https://www.netflix.com".to_string(),
        "https://www.spotify.com".to_string(),
        "https://www.slack.com".to_string(),
        "https://www.dropbox.com".to_string(),
        "https://www.salesforce.com".to_string(),
        "https://www.adobe.com".to_string(),
        "https://www.ibm.com".to_string(),
        "https://www.oracle.com".to_string(),
        "https://www.paypal.com".to_string(),
        "https://www.uber.com".to_string(),
        "https://www.airbnb.com".to_string(),
        "https://www.pinterest.com".to_string(),
        "https://www.tesla.com".to_string(),
        "https://www.bbc.com".to_string(),
        "https://www.cnn.com".to_string(),
        "https://www.nytimes.com".to_string(),
        "https://www.wsj.com".to_string(),
        "https://www.theguardian.com".to_string(),
        "https://www.forbes.com".to_string(),
        "https://www.bloomberg.com".to_string(),
        "https://www.weather.com".to_string(),
        "https://www.walmart.com".to_string(),
        "https://www.target.com".to_string(),
        "https://www.bestbuy.com".to_string(),
        "https://www.ebay.com".to_string(),
        "https://www.homedepot.com".to_string(),
        "https://www.lowes.com".to_string(),
        "https://www.costco.com".to_string(),
        "https://www.nasa.gov".to_string(),
        "https://www.fbi.gov".to_string(),
        "https://www.nih.gov".to_string(),
        "https://www.whitehouse.gov".to_string(),
    ];


    let statuses = check_websites(urls, std::time::Duration::from_secs(config.timeout), config.num_threads);

    report_statuses(&statuses);
}
