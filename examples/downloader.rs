use youtube_dl::{YoutubeDl, download_yt_dlp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt_dlp_path = download_yt_dlp(".").await?;

    let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
        .youtube_dl_path(yt_dlp_path)
        .run_async()
        .await?;
    let title = output.into_single_video().unwrap().title;
    println!("Video title: {title:?}");
    Ok(())
}
