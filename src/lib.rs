//! A crate for running and parsing the JSON output of `youtube-dl`.

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
use std::path::{Path, PathBuf};

pub mod model;

pub use crate::model::*;

/// Data returned by `YoutubeDl::run`. Output can either be a single video or a playlist of videos.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum YoutubeDlOutput {
    /// Playlist result
    Playlist(Box<Playlist>),
    /// Single video result
    SingleVideo(Box<SingleVideo>),
}

impl YoutubeDlOutput {
    #[cfg(test)]
    fn to_single_video(self) -> SingleVideo {
        match self {
            YoutubeDlOutput::SingleVideo(video) => *video,
            _ => panic!("this is a playlist, not a single video"),
        }
    }
}

/// Tells youtube-dl what to extract from the url
/// As to what they do, refer to `man youtube-dl`
#[derive(Clone, Copy, Debug)]
pub enum YoutubeDlExtractMode {
    /// Extract json (-J)
    JSONSingle,
    /// Dump json (-j)
    JSONDump,
    /// Extract download url (-g)
    Url,
    /// Extract title (-e)
    Title,
    /// Extract id (--get-id)
    Id,
    /// Extract thumbnail url (--get-thumbnail)
    Thumbnail,
    /// Extract (--get-description)
    Description,
    /// Extract (--get-duration)
    Duration,
    /// Extract (--get-filename)
    Filename,
    /// Extract (--get-format)
    Format,
}

/// Errors that can occur during executing `youtube-dl` or during parsing the output.
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(std::io::Error),

    /// Error parsing text output
    Utf8(std::string::FromUtf8Error),

    /// Error parsing JSON
    Json(serde_json::Error),

    /// `youtube-dl` returned a non-zero exit code
    ExitCode {
        /// Exit code
        code: i32,
        /// Standard error of youtube-dl
        stderr: String,
    },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {}", err),
            Self::Utf8(err) => write!(f, "utf8 error: {}", err),
            Self::Json(err) => write!(f, "json error: {}", err),
            Self::ExitCode { code, stderr } => {
                write!(f, "non-zero exit code: {}, stderr: {}", code, stderr)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Utf8(err) => Some(err),
            Self::Json(err) => Some(err),
            Self::ExitCode { .. } => None,
        }
    }
}

/// A builder to create a `youtube-dl` command to execute.
#[derive(Clone, Debug)]
pub struct YoutubeDl {
    youtube_dl_path: Option<PathBuf>,
    format: Option<String>,
    socket_timeout: Option<String>,
    all_formats: bool,
    auth: Option<(String, String)>,
    user_agent: Option<String>,
    referer: Option<String>,
    url: String,
}

impl YoutubeDl {
    /// Create a new builder.
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            youtube_dl_path: None,
            format: None,
            socket_timeout: None,
            all_formats: false,
            auth: None,
            user_agent: None,
            referer: None,
        }
    }

    /// Set the path to the `youtube-dl` executable.
    pub fn youtube_dl_path<P: AsRef<Path>>(&mut self, youtube_dl_path: P) -> &mut Self {
        self.youtube_dl_path = Some(youtube_dl_path.as_ref().to_owned());
        self
    }

    /// Set the `-F` command line option.
    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
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

    fn path(&self) -> &Path {
        match &self.youtube_dl_path {
            Some(path) => path,
            None => Path::new("youtube-dl"),
        }
    }

    fn process_args(&self, mode: &YoutubeDlExtractMode) -> Vec<&str> {
        let mut args = vec![];
        if let Some(format) = &self.format {
            args.push("-f");
            args.push(format);
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

        if let Some(user_agent) = &self.user_agent {
            args.push("--user-agent");
            args.push(user_agent);
        }

        if let Some(referer) = &self.referer {
            args.push("--referer");
            args.push(referer);
        }
        
        use YoutubeDlExtractMode::*;
        let extract_arg = match mode {
            JSONSingle  => "-J",
            JSONDump    => "-j",
            Url         => "-g",
            Title       => "-e",
            Id          => "--get-id",
            Thumbnail   => "--get-thumbnail",
            Description => "--get-description",
            Duration    => "--get-duration",
            Filename    => "--get-filename",
            Format      => "--get-format"
        };
        args.push(extract_arg);
        args.push(&self.url);
        log::debug!("youtube-dl arguments: {:?}", args);

        args
    }
    
    /// Run youtube-dl recovering a string from the output
    /// Outputs multiple lines for multiple entries in a playlist
    pub fn extract_str(&self, mode: &YoutubeDlExtractMode) -> Result<Vec<String>, Error> {
        use std::process::{Command, Stdio};

        let process_args = self.process_args(mode);
        let path = self.path();
        let output = Command::new(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(process_args)
            .output()?;
        
        if output.status.success() {
            let value = String::from_utf8(output.stdout)?;
            
            use YoutubeDlExtractMode::*;
            match &mode {
                JSONSingle => Ok(vec![value]),
                _ => {
                    let lines: Vec<_> = value.lines().map(|s| s.to_string()).collect();
                    Ok(lines)
                }
            }
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap_or_default();
            Err(Error::ExitCode {
                code: output.status.code().unwrap_or(1),
                stderr,
            })
        }
    }

    /// Extract full json from youtube-dl then parse it
    pub fn extract_json_single(&self) -> Result<Value, Error> {
        let output = self.extract_str(&YoutubeDlExtractMode::JSONSingle)?;
        serde_json::from_str(&output[0]).map_err(|e| e.into())
    }
    
    /// Extract templated JSON from youtube-dl then parse it
    pub fn extract_json_dump(&self) -> Result<Value, Error> {
        let output = self.extract_str(&YoutubeDlExtractMode::JSONDump)?;
        serde_json::from_str(&output[0]).map_err(|e| e.into())
    }
    
    /// Extract full json from youtube-dl then serialize it into a YoutubeDlOutput
    pub fn extract_full(&self) -> Result<YoutubeDlOutput, Error> {
        use serde_json::json;
        
        let value = self.extract_json_single()?;
        let is_playlist = value["_type"] == json!("playlist");
        if is_playlist {
            let playlist: Playlist = serde_json::from_value(value)?;
            Ok(YoutubeDlOutput::Playlist(Box::new(playlist)))
        } else {
            let video: SingleVideo = serde_json::from_value(value)?;
            Ok(YoutubeDlOutput::SingleVideo(Box::new(video)))
        }
        
    }

    /// Run youtube-dl with the arguments specified through the builder
    /// Deprecated by extract_full, but kept here for api compatibility
    pub fn run(&self) -> Result<YoutubeDlOutput, Error> {
        self.extract_full()
    }
}

#[cfg(test)]
mod tests {
    use super::YoutubeDl;

    #[test]
    fn test_youtube_url() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
            .socket_timeout("15")
            .run()
            .unwrap()
            .to_single_video();
        assert_eq!(output.id, "7XGyWcuYVrg");
    }
}
