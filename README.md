# youtube-dl-rs

[<img alt="Crates.io" src="https://img.shields.io/crates/v/youtube_dl">](https://crates.io/crates/youtube_dl)

**NOTE**: The name for this library is a bit misleading, it currently does not support `youtube-dl` as its development seems to be very slow recently,
it does support `yt-dlp`, which has diverged from `youtube-dl` in some ways, but sees a lot more development.

Runs yt-dlp and parses its JSON output. Example:

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

## Feature flags

- **tokio**: Enables the `async` variants of the `run`, `run_raw` and `download_to` methods.
- **downloader-native-tls** / **downloader-rustls-tls**: Enables the `download_yt_dlp` method and `YoutubeDlFetcher` struct to download the `yt-dlp` executable with the given TLS backend used for reqwest.
