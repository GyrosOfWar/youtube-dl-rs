# youtube-dl-rs

[<img alt="Crates.io" src="https://img.shields.io/crates/v/youtube_dl">](https://crates.io/crates/youtube_dl)

Runs youtube-dl and parses its JSON output. Example:
```rust
  let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
      .socket_timeout("15")
      .run()
      .unwrap();
  let title = match output {
      YoutubeDlOutput::SingleVideo(video) => video.title,
      _ => panic!("single video should not be a playlist")  
  };
  println!("Video title: {}", title);
```
