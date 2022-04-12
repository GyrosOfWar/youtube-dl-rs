use std::path::Path;


#[derive(Debug)]
pub struct YoutubeDlDownloader {
    client: reqwest::Client
}

impl YoutubeDlDownloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download(&self, destination: impl AsRef<Path>){

    }
}