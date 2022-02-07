# youtube-dl-rs
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
