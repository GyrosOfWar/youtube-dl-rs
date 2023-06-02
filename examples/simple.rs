use youtube_dl::YoutubeDl;

fn main() {
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
        .socket_timeout("15")
        .run()
        .unwrap();
    let title = output.into_single_video().unwrap().title;
    println!("Video title: {title:?}");
}
