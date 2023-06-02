use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
        .socket_timeout("15")
        .run_async()
        .await?;
    let title = output.into_single_video().unwrap().title;
    println!("Video title: {title:?}");
    Ok(())
}
