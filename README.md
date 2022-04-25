# youtube-dl-rs
Runs youtube-dl and parses its JSON output. Example:
```rust
use youtube_dl::YoutubeDl;

let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
  .socket_timeout("15")
  .run()
  .unwrap();
let title = output.into_single_video().unwrap().title;
println!("Video title: {}", title);
```

Or, if you want to it to run asynchronously (enable the feature `tokio`):
```rust
let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
    .socket_timeout("15")
    .run_async()
    .await?;
let title = output.into_single_video().unwrap().title;
println!("Video title: {}", title);
Ok(())
```