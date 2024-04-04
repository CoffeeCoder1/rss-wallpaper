use std::error::Error;
use rss::Channel;
use wallpaper;
use clap::Parser;

/// Change the current wallpaper to the first image in the latest item in an RSS feed
#[derive(Parser)]
struct Cli {
    /// The URL of the RSS feed to parse
    path: String,
}

async fn feed_parser(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed_url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let media_url_binding = feed_parser(&args.path)
            .await
            .expect("Failed to parse RSS feed");
    let media_url: String = media_url_binding
            .items[0]
            .extensions
            .get("media")
            .expect("No media found")
            ["content"]
            [0]
            .attrs
            .get("url")
            .expect("Media has no URL")
            .to_string();
    tokio::task::spawn_blocking(move || {
        wallpaper::set_from_url(&media_url).unwrap();
    }).await.expect("Failed to set wallpaper from image URL");
}
