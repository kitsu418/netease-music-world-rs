use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use reqwest::{Client, header};

const TARGET_URL: &str = "https://music.163.com/song?id=22699066";
const REQUEST_TIMEOUT_SECS: u64 = 10;

#[derive(Parser, Debug)]
#[command(author, version, about = "Fetch NetEase Music song page with spoofed headers", long_about = None)]
struct Cli {
    /// NetEase MUSIC_U cookie (or set env MUSIC_U)
    #[arg(long, env = "MUSIC_U")]
    music_u: Option<String>,
}

fn build_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Real-IP",
        header::HeaderValue::from_static("211.161.244.70"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        ),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("en-US,en;q=0.9"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36",
        ),
    );

    println!("Default headers:");
    for (key, value) in headers.iter() {
        println!("{}: {}", key.as_str(), value.to_str().unwrap_or("<binary>"));
    }
    println!();

    Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .default_headers(headers)
        .build()
        .context("failed to build HTTP client")
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cookie_value = if let Some(music_u) = cli.music_u.as_deref() {
        format!("MUSIC_U={music_u};")
    } else {
        return Err(anyhow!("Provide --music-u/env MUSIC_U"));
    };

    let client = build_client()?;
    println!("Extra header:");
    println!("Cookie: {}", cookie_value);

    let response = client
        .get(TARGET_URL)
        .header(header::COOKIE, cookie_value)
        .header("referer", "https://music.163.com/")
        .send()
        .await
        .context("request failed")?;

    if response.status() != reqwest::StatusCode::OK {
        Err(anyhow!("unexpected response status: {}", response.status()))
    } else {
        Ok(())
    }
}
