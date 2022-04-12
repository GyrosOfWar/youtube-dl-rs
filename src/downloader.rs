use std::path::{Path, PathBuf};

use log::info;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;

const FILE_NAME: &str = if cfg!(target_os = "windows") {
    "yt-dlp.exe"
} else {
    "yt-dlp"
};

use crate::Error;

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

struct NewestRelease {
    url: String,
    tag: String,
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

        let release: GithubRelease = self.client.get(url).send().await?.json().await?;
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
        use tokio::fs::{self, File};

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

        let mut file = File::create(&path).await?;
        let mut response = self.client.get(release.url).send().await?;

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(path)
    }
}

/// Downloads the yt-dlp executable to the specified destination.
pub async fn download_yt_dlp(destination: impl AsRef<Path>) -> Result<PathBuf, Error> {
    YoutubeDlFetcher::default().download(destination).await
}
