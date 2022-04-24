use std::path::{Path, PathBuf};

use crate::Error;
use log::info;
use serde::Deserialize;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

const FILE_NAME: &str = if cfg!(target_os = "windows") {
    "yt-dlp.exe"
} else {
    "yt-dlp"
};

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Deserialize, Debug)]
struct GithubAsset {
    browser_download_url: String,
    name: String,
}

struct NewestRelease {
    url: String,
    tag: String,
}

/// Handles downloading of the youtube-dl/yt-dlp binary from GitHub.
#[derive(Debug)]
pub struct YoutubeDlFetcher {
    client: reqwest::Client,
    github_org: String,
    repo_name: String,
}

/// Downloads yt-dlp per default.
impl Default for YoutubeDlFetcher {
    fn default() -> Self {
        Self {
            client: Default::default(),
            github_org: "yt-dlp".into(),
            repo_name: "yt-dlp".into(),
        }
    }
}

impl YoutubeDlFetcher {
    /// Allows specifying the GitHub user and repository to download the binary from.
    /// The `Default` implementation uses `yt-dlp` for both.
    pub fn new(user: &str, repo: &str) -> Self {
        Self {
            client: Default::default(),
            github_org: user.to_string(),
            repo_name: repo.to_string(),
        }
    }

    async fn find_newest_release(&self) -> Result<NewestRelease, Error> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.github_org, self.repo_name
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "youtube-dl-rs")
            .send()
            .await?;
        let release: GithubRelease = if log::log_enabled!(log::Level::Debug) {
            let text = response.text().await?;
            log::debug!("received response from github: {}", text);
            serde_json::from_str(&text)?
        } else {
            response.json().await?
        };

        info!("received response from github: {:?}", release);

        let url = release
            .assets
            .into_iter()
            .find(|r| r.name == FILE_NAME)
            .map(|r| r.browser_download_url)
            .ok_or(Error::NoReleaseFound)?;

        Ok(NewestRelease {
            url,
            tag: release.tag_name,
        })
    }

    /// Fetches the latest release from the GitHub API, then downloads the binary
    /// to the specified destination. `destination` can either be a directory, in which case
    /// the executable is downloaded to that directory, or a file, in which case the file is created.
    pub async fn download(&self, destination: impl AsRef<Path>) -> Result<PathBuf, Error> {
        let release = self.find_newest_release().await?;
        log::info!("found release: {} at URL {}", release.tag, release.url);
        let destination = destination.as_ref();

        if !destination.exists() {
            fs::create_dir_all(destination).await?;
        }

        let path = if destination.is_file() {
            destination.to_owned()
        } else {
            destination.join(FILE_NAME)
        };

        let mut file = create_file(&path).await?;
        let mut response = self.client.get(release.url).send().await?;

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(path)
    }
}

#[cfg(target_os = "windows")]
async fn create_file(path: impl AsRef<Path>) -> tokio::io::Result<File> {
    File::create(&path).await
}

#[cfg(not(target_os = "windows"))]
async fn create_file(path: impl AsRef<Path>) -> tokio::io::Result<File> {
    use tokio::fs::OpenOptions;

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o744)
        .open(&path)
        .await
}

/// Downloads the yt-dlp executable to the specified destination.
pub async fn download_yt_dlp(destination: impl AsRef<Path>) -> Result<PathBuf, Error> {
    YoutubeDlFetcher::default().download(destination).await
}

#[cfg(test)]
mod tests {
    use crate::{download_yt_dlp, YoutubeDl};

    fn logger() {
        std::env::set_var("RUST_LOG", "info");
        let _ = env_logger::try_init();
    }

    #[tokio::test]
    async fn test_download_yt_dlp() {
        logger();
        let path = download_yt_dlp(".").await.unwrap();
        assert!(path.is_file(), "downloaded file should exist");

        let result = YoutubeDl::new("https://www.youtube.com/watch?v=otCWfUtZ-bU")
            .youtube_dl_path(path)
            .run_async()
            .await
            .unwrap();

        assert_eq!(result.into_single_video().unwrap().id, "otCWfUtZ-bU");
        let _ = std::fs::remove_file("yt-dlp");
        let _ = std::fs::remove_file("yt-dlp.exe");
    }
}
