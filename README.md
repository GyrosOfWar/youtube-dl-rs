# youtube-dl-rs
Runs youtube-dl and parses its JSON output. Example:
```rust
use youtube_dl::YoutubeDl;

let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
  .socket_timeout("15")
  .run()
  .unwrap();
```
