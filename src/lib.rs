//! # youtube_dl
//! A crate for running and parsing the JSON output of `youtube-dl`.
//! Example usage:
//! ```rust
//! use youtube_dl::YoutubeDl;
//! let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
//!   .socket_timeout("15")
//!   .run()
//!   .unwrap();
//! ```

#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error as StdError;
use std::fmt;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::time::Duration;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Exposes a function to download the latest version of youtube-dl/yt-dlp.
#[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
pub mod downloader;
pub mod model;

pub use crate::model::*;

#[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
pub use crate::downloader::download_yt_dlp;

/// Data returned by `YoutubeDl::run`. Output can either be a single video or a playlist of videos.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum YoutubeDlOutput {
    /// Playlist result
    Playlist(Box<Playlist>),
    /// Single video result
    SingleVideo(Box<SingleVideo>),
}

impl YoutubeDlOutput {
    /// Get the inner content as a single video.
    pub fn into_single_video(self) -> Option<SingleVideo> {
        match self {
            YoutubeDlOutput::SingleVideo(video) => Some(*video),
            _ => None,
        }
    }

    /// Get the inner content as a playlist.
    pub fn into_playlist(self) -> Option<Playlist> {
        match self {
            YoutubeDlOutput::Playlist(playlist) => Some(*playlist),
            _ => None,
        }
    }
}

/// Errors that can occur during executing `youtube-dl` or during parsing the output.
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(std::io::Error),

    /// Error parsing JSON
    Json(serde_json::Error),

    /// `youtube-dl` returned a non-zero exit code
    ExitCode {
        /// Exit code
        code: i32,
        /// Standard error of youtube-dl
        stderr: String,
    },

    /// Process-level timeout expired.
    ProcessTimeout,

    /// HTTP error (when fetching youtube-dl/yt-dlp)
    #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
    Http(reqwest::Error),

    /// When no GitHub release could be found to download the youtube-dl/yt-dlp executable.
    #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
    NoReleaseFound,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

#[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {}", err),
            Self::Json(err) => write!(f, "json error: {}", err),
            Self::ExitCode { code, stderr } => {
                write!(f, "non-zero exit code: {}, stderr: {}", code, stderr)
            }
            Self::ProcessTimeout => write!(f, "process timed out"),
            #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
            Self::Http(err) => write!(f, "http error: {}", err),
            #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
            Self::NoReleaseFound => write!(f, "no github release found for specified binary"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Json(err) => Some(err),
            Self::ExitCode { .. } => None,
            Self::ProcessTimeout => None,
            #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
            Self::Http(err) => Some(err),
            #[cfg(any(feature = "downloader-rustls-tls", feature = "downloader-native-tls"))]
            Self::NoReleaseFound => None,
        }
    }
}

/// The search options currently supported by youtube-dl, and a custom option to allow
/// specifying custom options, in case this library is outdated.
#[derive(Clone, Debug)]
pub enum SearchType {
    /// Search on youtube.com
    Youtube,
    /// Search with yahoo.com's video search
    Yahoo,
    /// Search with Google's video search
    Google,
    /// Search on SoundCloud
    SoundCloud,
    /// Allows to specify a custom search type, for forwards compatibility purposes.
    Custom(String),
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchType::Yahoo => write!(f, "yvsearch"),
            SearchType::Youtube => write!(f, "ytsearch"),
            SearchType::Google => write!(f, "gvsearch"),
            SearchType::SoundCloud => write!(f, "scsearch"),
            SearchType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Specifies where to search, how many results to fetch and the query. The count
/// defaults to 1, but can be changed with the `with_count` method.
#[derive(Clone, Debug)]
pub struct SearchOptions {
    search_type: SearchType,
    count: usize,
    query: String,
}

impl SearchOptions {
    /// Search on youtube.com
    pub fn youtube(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: SearchType::Youtube,
            count: 1,
        }
    }
    /// Search with Google's video search
    pub fn google(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: SearchType::Google,
            count: 1,
        }
    }
    /// Search with yahoo.com's video search
    pub fn yahoo(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: SearchType::Yahoo,
            count: 1,
        }
    }
    /// Search on SoundCloud
    pub fn soundcloud(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: SearchType::SoundCloud,
            count: 1,
        }
    }
    /// Search with a custom search provider (in case this library falls behind the feature set of youtube-dl)
    pub fn custom(search_type: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: SearchType::Custom(search_type.into()),
            count: 1,
        }
    }
    /// Set the count for how many videos at most to retrieve from the search.
    pub fn with_count(self, count: usize) -> Self {
        Self {
            search_type: self.search_type,
            query: self.query,
            count,
        }
    }
}

impl fmt::Display for SearchOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}:{}", self.search_type, self.count, self.query)
    }
}

/// A builder to create a `youtube-dl` command to execute.
#[derive(Clone, Debug)]
pub struct YoutubeDl {
    youtube_dl_path: Option<PathBuf>,
    format: Option<String>,
    flat_playlist: bool,
    socket_timeout: Option<String>,
    all_formats: bool,
    auth: Option<(String, String)>,
    cookies: Option<String>,
    cookies_from_browser: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
    url: String,
    process_timeout: Option<Duration>,
    playlist_reverse: bool,
    date_before: Option<String>,
    date_after: Option<String>,
    date: Option<String>,
    extract_audio: bool,
    playlist_items: Option<String>,
    max_downloads: Option<String>,
    extra_args: Vec<String>,
    output_template: Option<String>,
    output_directory: Option<String>,
    #[cfg(test)]
    debug: bool,
    ignore_errors: bool,
}

impl YoutubeDl {
    /// Create a new builder.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            youtube_dl_path: None,
            format: None,
            flat_playlist: false,
            socket_timeout: None,
            all_formats: false,
            auth: None,
            cookies: None,
            cookies_from_browser: None,
            user_agent: None,
            referer: None,
            process_timeout: None,
            date: None,
            date_after: None,
            date_before: None,
            playlist_reverse: false,
            extract_audio: false,
            playlist_items: None,
            max_downloads: None,
            extra_args: Vec::new(),
            output_template: None,
            output_directory: None,
            #[cfg(test)]
            debug: false,
            ignore_errors: false,
        }
    }

    /// Performs a search with the given search options.
    pub fn search_for(options: &SearchOptions) -> Self {
        Self::new(options.to_string())
    }

    /// Set the path to the `youtube-dl` or `yt-dlp executable.
    pub fn youtube_dl_path<P: AsRef<Path>>(&mut self, youtube_dl_path: P) -> &mut Self {
        self.youtube_dl_path = Some(youtube_dl_path.as_ref().to_owned());
        self
    }

    /// Set the `-f` command line option.
    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }

    /// Set the `--flat-playlist` command line flag.
    pub fn flat_playlist(&mut self, flat_playlist: bool) -> &mut Self {
        self.flat_playlist = flat_playlist;
        self
    }

    /// Set the `--socket-timeout` command line flag.
    pub fn socket_timeout<S: Into<String>>(&mut self, socket_timeout: S) -> &mut Self {
        self.socket_timeout = Some(socket_timeout.into());
        self
    }

    /// Set the `--user-agent` command line flag.
    pub fn user_agent<S: Into<String>>(&mut self, user_agent: S) -> &mut Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set the `--playlist-reverse` flag. Useful with break-on-reject and date_before
    /// for faster queries.
    pub fn playlist_reverse(&mut self, playlist_reverse: bool) -> &mut Self {
        self.playlist_reverse = playlist_reverse;
        self
    }

    /// Sets the `--date` command line flag only downloading/viewing videos on this date
    pub fn date<S: Into<String>>(&mut self, date_string: S) -> &mut Self {
        self.date = Some(date_string.into());
        self
    }

    /// Set the `--datebefore` flag only downloading/viewing videos on or before this date
    pub fn date_before<S: Into<String>>(&mut self, date_string: S) -> &mut Self {
        self.date_before = Some(date_string.into());
        self
    }

    /// Set the `--dateafter` flag only downloading/viewing vidieos on or after this date
    pub fn date_after<S: Into<String>>(&mut self, date_string: S) -> &mut Self {
        self.date_after = Some(date_string.into());
        self
    }

    /// Set the `--referer` command line flag.
    pub fn referer<S: Into<String>>(&mut self, referer: S) -> &mut Self {
        self.referer = Some(referer.into());
        self
    }

    /// Set the `--all-formats` command line flag.
    pub fn all_formats(&mut self, all_formats: bool) -> &mut Self {
        self.all_formats = all_formats;
        self
    }

    /// Set the `-u` and `-p` command line flags.
    pub fn auth<S: Into<String>>(&mut self, username: S, password: S) -> &mut Self {
        self.auth = Some((username.into(), password.into()));
        self
    }

    /// Specify a file with cookies in Netscape cookie format.
    pub fn cookies<S: Into<String>>(&mut self, cookie_path: S) -> &mut Self {
        self.cookies = Some(cookie_path.into());
        self
    }

    /// Set the `--cookies-from-browser` command line flag.
    pub fn cookies_from_browser<S: Into<String>>(
        &mut self,
        browser_name: S,
        browser_keyring: Option<S>,
        browser_profile: Option<S>,
        browser_container: Option<S>,
    ) -> &mut Self {
        self.cookies_from_browser = Some(format!(
            "{}{}{}{}",
            browser_name.into(),
            if let Some(keyring) = browser_keyring {
                format!("+{}", keyring.into())
            } else {
                String::from("")
            },
            if let Some(profile) = browser_profile {
                format!(":{}", profile.into())
            } else {
                String::from("")
            },
            if let Some(container) = browser_container {
                format!("::{}", container.into())
            } else {
                String::from("")
            }
        ));
        self
    }

    /// Set a process-level timeout for youtube-dl. (this controls the maximum overall duration
    /// the process may take, when it times out, `Error::ProcessTimeout` is returned)
    pub fn process_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.process_timeout = Some(timeout);
        self
    }

    /// Set the `--extract-audio` command line flag.
    pub fn extract_audio(&mut self, extract_audio: bool) -> &mut Self {
        self.extract_audio = extract_audio;
        self
    }

    /// Set the `--playlist-items` command line flag.
    pub fn playlist_items(&mut self, index: u32) -> &mut Self {
        self.playlist_items = Some(index.to_string());
        self
    }

    /// Set the `--max-downloads` command line flag.
    pub fn max_downloads(&mut self, max_downloads: u32) -> &mut Self {
        self.max_downloads = Some(max_downloads.to_string());
        self
    }

    /// Add an additional custom CLI argument.
    ///
    /// This allows specifying arguments that are not covered by other
    /// configuration methods.
    pub fn extra_arg<S: Into<String>>(&mut self, arg: S) -> &mut Self {
        self.extra_args.push(arg.into());
        self
    }

    /// Specify the filename template. Only relevant for downloading.
    /// (referred to as "output template" by [youtube-dl docs](https://github.com/ytdl-org/youtube-dl#output-template))
    pub fn output_template<S: Into<String>>(&mut self, arg: S) -> &mut Self {
        self.output_template = Some(arg.into());
        self
    }

    /// Specify the output directory. Only relevant for downloading.
    /// (the `-P` command line switch)
    pub fn output_directory<S: Into<String>>(&mut self, arg: S) -> &mut Self {
        self.output_directory = Some(arg.into());
        self
    }

    #[cfg(test)]
    pub fn debug(&mut self, arg: bool) -> &mut Self {
        self.debug = arg;
        self
    }

    /// Specify whether to ignore errors (exit code & flag)
    pub fn ignore_errors(&mut self, arg: bool) -> &mut Self {
        self.ignore_errors = arg;
        self
    }

    fn path(&self) -> &Path {
        match &self.youtube_dl_path {
            Some(path) => path,
            None => Path::new("yt-dlp"),
        }
    }

    fn common_args(&self) -> Vec<&str> {
        let mut args = vec![];
        if let Some(format) = &self.format {
            args.push("-f");
            args.push(format);
        }

        if self.flat_playlist {
            args.push("--flat-playlist");
        }

        if let Some(timeout) = &self.socket_timeout {
            args.push("--socket-timeout");
            args.push(timeout);
        }

        if self.all_formats {
            args.push("--all-formats");
        }

        if let Some((user, password)) = &self.auth {
            args.push("-u");
            args.push(user);
            args.push("-p");
            args.push(password);
        }

        if let Some(cookie_path) = &self.cookies {
            args.push("--cookies");
            args.push(cookie_path);
        }

        if let Some(cookies_from_browser) = &self.cookies_from_browser {
            args.push("--cookies-from-browser");
            args.push(cookies_from_browser);
        }

        if let Some(user_agent) = &self.user_agent {
            args.push("--user-agent");
            args.push(user_agent);
        }

        if let Some(referer) = &self.referer {
            args.push("--referer");
            args.push(referer);
        }

        if self.extract_audio {
            args.push("--extract-audio");
        }

        if let Some(playlist_items) = &self.playlist_items {
            args.push("--playlist-items");
            args.push(playlist_items);
        }

        if let Some(max_downloads) = &self.max_downloads {
            args.push("--max-downloads");
            args.push(max_downloads);
        }

        if let Some(output_template) = &self.output_template {
            args.push("-o");
            args.push(output_template);
        }

        if let Some(output_dir) = &self.output_directory {
            args.push("-P");
            args.push(output_dir);
        }

        if let Some(date) = &self.date {
            args.push("--date");
            args.push(date);
        }

        if let Some(date_after) = &self.date_after {
            args.push("--dateafter");
            args.push(date_after);
        }

        if let Some(date_before) = &self.date_before {
            args.push("--datebefore");
            args.push(date_before);
        }

        if self.ignore_errors {
            args.push("--ignore-errors");
        }

        for extra_arg in &self.extra_args {
            args.push(extra_arg);
        }

        args
    }

    fn process_args(&self) -> Vec<&str> {
        let mut args = self.common_args();

        if let Some(output_dir) = &self.output_directory {
            args.push("-P");
            args.push(output_dir);
        }

        args.push("-J");
        args.push(&self.url);
        log::debug!("youtube-dl arguments: {:?}", args);

        args
    }

    fn process_download_args<'a>(&'a self, folder: &'a str) -> Vec<&'a str> {
        let mut args = self.common_args();

        args.push("-P");
        args.push(folder);
        args.push("--no-simulate");
        args.push("--no-progress");
        args.push(&self.url);
        log::debug!("youtube-dl arguments: {:?}", args);

        args
    }

    fn run_process(&self, args: Vec<&str>) -> Result<ProcessResult, Error> {
        use std::io::Read;
        use std::process::{Command, Stdio};
        use wait_timeout::ChildExt;

        let path = self.path();
        #[cfg(not(target_os = "windows"))]
        let mut child = Command::new(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()?;
        #[cfg(target_os = "windows")]
        let mut child = Command::new(path)
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()?;

        // Continually read from stdout so that it does not fill up with large output and hang forever.
        // We don't need to do this for stderr since only stdout has potentially giant JSON.
        let mut stdout = Vec::new();
        let child_stdout = child.stdout.take();
        std::io::copy(&mut child_stdout.unwrap(), &mut stdout)?;

        let exit_code = if let Some(timeout) = self.process_timeout {
            match child.wait_timeout(timeout)? {
                Some(status) => status,
                None => {
                    child.kill()?;
                    return Err(Error::ProcessTimeout);
                }
            }
        } else {
            child.wait()?
        };

        let mut stderr = vec![];
        if let Some(mut reader) = child.stderr {
            reader.read_to_end(&mut stderr)?;
        }

        Ok(ProcessResult {
            stdout,
            stderr,
            exit_code,
        })
    }

    #[cfg(feature = "tokio")]
    async fn run_process_async(&self, args: Vec<&str>) -> Result<ProcessResult, Error> {
        use std::process::Stdio;
        use tokio::io::AsyncReadExt;
        use tokio::process::Command;
        use tokio::time::timeout;

        let path = self.path();
        #[cfg(not(target_os = "windows"))]
        let mut child = Command::new(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()?;
        #[cfg(target_os = "windows")]
        let mut child = Command::new(path)
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()?;

        // Continually read from stdout so that it does not fill up with large output and hang forever.
        // We don't need to do this for stderr since only stdout has potentially giant JSON.
        let mut stdout = Vec::new();
        let child_stdout = child.stdout.take();
        tokio::io::copy(&mut child_stdout.unwrap(), &mut stdout).await?;

        let exit_code = if let Some(dur) = self.process_timeout {
            match timeout(dur, child.wait()).await {
                Ok(n) => n?,
                Err(_) => {
                    child.kill().await?;
                    return Err(Error::ProcessTimeout);
                }
            }
        } else {
            child.wait().await?
        };
        let mut stderr = vec![];
        if let Some(mut reader) = child.stderr {
            reader.read_to_end(&mut stderr).await?;
        }

        Ok(ProcessResult {
            stdout,
            stderr,
            exit_code,
        })
    }

    fn process_json_output(&self, stdout: Vec<u8>) -> Result<YoutubeDlOutput, Error> {
        use serde_json::json;

        #[cfg(test)]
        if self.debug {
            let string = std::str::from_utf8(&stdout).expect("invalid utf-8 output");
            eprintln!("{}", string);
        }

        let value: Value = serde_json::from_reader(stdout.as_slice())?;

        let is_playlist = value["_type"] == json!("playlist");
        if is_playlist {
            let playlist: Playlist = serde_json::from_value(value)?;
            Ok(YoutubeDlOutput::Playlist(Box::new(playlist)))
        } else {
            let video: SingleVideo = serde_json::from_value(value)?;
            Ok(YoutubeDlOutput::SingleVideo(Box::new(video)))
        }
    }

    /// Run yt-dlp with the arguments specified through the builder and parse its
    /// JSON ouput into `YoutubeDlOutput`. Note: This can fail when the JSON output
    /// is not compatible with the struct definitions in this crate.
    pub fn run(&self) -> Result<YoutubeDlOutput, Error> {
        let args = self.process_args();
        let ProcessResult {
            stderr,
            stdout,
            exit_code,
        } = self.run_process(args)?;

        if exit_code.success() || self.ignore_errors {
            self.process_json_output(stdout)
        } else {
            let stderr = String::from_utf8(stderr).unwrap_or_default();
            Err(Error::ExitCode {
                code: exit_code.code().unwrap_or(1),
                stderr,
            })
        }
    }

    /// Run yt-dlp with the arguments through the builder and parse its JSON output
    /// into a `serde_json::Value`. This is meant as a fallback for when the JSON
    /// output is not compatible with the struct definitions in this crate.
    pub fn run_raw(&self) -> Result<Value, Error> {
        let args = self.process_args();
        let ProcessResult {
            stderr,
            stdout,
            exit_code,
        } = self.run_process(args)?;

        if exit_code.success() || self.ignore_errors {
            let value: Value = serde_json::from_reader(stdout.as_slice())?;
            Ok(value)
        } else {
            let stderr = String::from_utf8(stderr).unwrap_or_default();
            Err(Error::ExitCode {
                code: exit_code.code().unwrap_or(1),
                stderr,
            })
        }
    }

    /// Run yt-dlp asynchronously with the arguments specified through the builder.
    #[cfg(feature = "tokio")]
    pub async fn run_async(&self) -> Result<YoutubeDlOutput, Error> {
        let args = self.process_args();
        let ProcessResult {
            stderr,
            stdout,
            exit_code,
        } = self.run_process_async(args).await?;

        if exit_code.success() || self.ignore_errors {
            self.process_json_output(stdout)
        } else {
            let stderr = String::from_utf8(stderr).unwrap_or_default();
            Err(Error::ExitCode {
                code: exit_code.code().unwrap_or(1),
                stderr,
            })
        }
    }

    /// Run yt-dlp asynchronously with the arguments through the builder and parse its JSON output
    /// into a `serde_json::Value`. This is meant as a fallback for when the JSON
    /// output is not compatible with the struct definitions in this crate.
    #[cfg(feature = "tokio")]
    pub async fn run_raw_async(&self) -> Result<Value, Error> {
        let args = self.process_args();
        let ProcessResult {
            stderr,
            stdout,
            exit_code,
        } = self.run_process_async(args).await?;

        if exit_code.success() || self.ignore_errors {
            let value: Value = serde_json::from_reader(stdout.as_slice())?;
            Ok(value)
        } else {
            let stderr = String::from_utf8(stderr).unwrap_or_default();
            Err(Error::ExitCode {
                code: exit_code.code().unwrap_or(1),
                stderr,
            })
        }
    }

    /// Download the file to the specified destination folder.
    pub fn download_to(&self, folder: impl AsRef<Path>) -> Result<ProcessResult, Error> {
        let folder_str = folder.as_ref().to_string_lossy();
        let args = self.process_download_args(&folder_str);
        self.run_process(args)
    }

    /// Download the file to the specified destination folder asynchronously.
    #[cfg(feature = "tokio")]
    pub async fn download_to_async(
        &self,
        folder: impl AsRef<Path>,
    ) -> Result<ProcessResult, Error> {
        let folder_str = folder.as_ref().to_string_lossy();
        let args = self.process_download_args(&folder_str);
        self.run_process_async(args).await
    }
}

#[derive(Debug)]
/// Represents the result of a process execution.
pub struct ProcessResult {
    /// Standard output from the process as a vector of bytes.
    pub stdout: Vec<u8>,
    /// Standard error output from the process as a vector of bytes.
    pub stderr: Vec<u8>,
    /// The exit status of the process. This can be used to determine if the process
    /// exited successfully or if there was an error.
    pub exit_code: ExitStatus,
}

impl fmt::Display for ProcessResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stdout_str = std::str::from_utf8(&self.stdout).unwrap_or_default();
        let stderr_str = std::str::from_utf8(&self.stderr).unwrap_or_default();
        let exit_code_str = match self.exit_code.success() {
            true => "Success".to_string(),
            false => format!("Error with exit code: {}", self.exit_code),
        };
        write!(
            f,
            "ProcessResult:\n  stdout: {}\n  stderr: {}\n  exit_code: {}",
            stdout_str, stderr_str, exit_code_str
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Protocol, SearchOptions, YoutubeDl};

    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn test_youtube_url() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
            .socket_timeout("15")
            .run()
            .unwrap()
            .into_single_video()
            .unwrap();
        assert_eq!(output.id, "7XGyWcuYVrg");
    }

    #[test]
    fn test_with_timeout() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
            .socket_timeout("15")
            .process_timeout(Duration::from_secs(15))
            .run()
            .unwrap()
            .into_single_video()
            .unwrap();
        assert_eq!(output.id, "7XGyWcuYVrg");
    }

    #[test]
    fn test_unknown_url() {
        YoutubeDl::new("https://www.rust-lang.org")
            .socket_timeout("15")
            .process_timeout(Duration::from_secs(15))
            .run()
            .unwrap_err();
    }

    #[test]
    fn test_search() {
        let output = YoutubeDl::search_for(&SearchOptions::youtube("Never Gonna Give You Up"))
            .socket_timeout("15")
            .process_timeout(Duration::from_secs(15))
            .run()
            .unwrap()
            .into_playlist()
            .unwrap();
        assert_eq!(output.entries.unwrap().first().unwrap().id, "dQw4w9WgXcQ");
    }

    #[test]
    fn correct_format_codec_parsing() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=WhWc3b3KhnY")
            .run()
            .unwrap()
            .into_single_video()
            .unwrap();

        let mut none_counter = 0;
        for format in output.formats.unwrap() {
            assert_ne!(Some("none".to_string()), format.acodec);
            assert_ne!(Some("none".to_string()), format.vcodec);
            if format.acodec.is_none() || format.vcodec.is_none() {
                none_counter += 1;
            }
        }
        assert!(none_counter > 0);
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_async() {
        use tokio::runtime::Runtime;
        let runtime = Runtime::new().unwrap();
        let output = runtime.block_on(async move {
            YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
                .socket_timeout("15")
                .run_async()
                .await
                .unwrap()
                .into_single_video()
                .unwrap()
        });
        assert_eq!(output.id, "7XGyWcuYVrg");
    }

    #[test]
    fn test_with_yt_dlp() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
            .run()
            .unwrap()
            .into_single_video()
            .unwrap();
        assert_eq!(output.id, "7XGyWcuYVrg");
    }

    #[test]

    fn test_download_with_yt_dlp() {
        // yee
        YoutubeDl::new("https://www.youtube.com/watch?v=q6EoRBvdVPQ")
            .debug(true)
            .output_template("yee")
            .download_to(".")
            .unwrap();
        assert!(Path::new("yee.webm").is_file() || Path::new("yee").is_file());
        let _ = std::fs::remove_file("yee.webm");
        let _ = std::fs::remove_file("yee");
    }

    #[test]
    #[ignore]
    fn test_timestamp_parse_error() {
        let output = YoutubeDl::new("https://www.reddit.com/r/loopdaddy/comments/baguqq/first_time_poster_here_couldnt_resist_sharing_my")
            .output_template("video")
            .run()
            .unwrap();
        assert_eq!(output.into_single_video().unwrap().width, Some(608.0));
    }

    #[test]
    fn test_protocol_fallback() {
        let parsed_protocol: Protocol = serde_json::from_str("\"http\"").unwrap();
        assert!(matches!(parsed_protocol, Protocol::Http));

        let unknown_protocol: Protocol = serde_json::from_str("\"some_unknown_protocol\"").unwrap();
        assert!(matches!(unknown_protocol, Protocol::Unknown));
    }

    #[test]
    fn test_download_to_destination() {
        let dir = tempfile::tempdir().unwrap();

        YoutubeDl::new("https://www.youtube.com/watch?v=q6EoRBvdVPQ")
            .download_to(&dir)
            .unwrap();

        let files: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
        assert_eq!(1, files.len());
        assert!(files[0].as_ref().unwrap().path().is_file());
    }
}
