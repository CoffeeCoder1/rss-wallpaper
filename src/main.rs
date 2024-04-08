use std::error::Error;
use rss::Channel;
use wallpaper;
use clap::Parser;
use log::info;

/// Change the current wallpaper to the first image in the latest item in an RSS feed
#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    /// The URL of the RSS feed to parse
    path: String,
}

async fn feed_parser(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
    info!("Downloading feed `{:}`", feed_url);
    let content = reqwest::get(feed_url)
        .await?
        .bytes()
        .await?;
    info!("Parsing feed");
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    
    let args = Cli::parse();
    let media_url_binding = feed_parser(&args.path)
            .await
            .expect("Failed to parse RSS feed");
    info!("Finding media URL");
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
    info!("Found media URL `{:}`", media_url);
    info!("Setting wallpaper");
    tokio::task::spawn_blocking(move || {
        wallpaper::set_from_url(&media_url).unwrap();
    }).await.expect("Failed to set wallpaper from image URL");
    info!("Wallpaper set");
}
