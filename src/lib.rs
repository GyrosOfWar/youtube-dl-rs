use failure::Fail;
use serde::{Deserialize, Serialize};

pub mod model;

pub use crate::model::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum YoutubeDlOutput {
    Playlist(Box<Playlist>),
    SingleVideo(Box<SingleVideo>),
}

impl YoutubeDlOutput {
    #[cfg(test)]
    fn to_playlist(self) -> Playlist {
        match self {
            YoutubeDlOutput::Playlist(playlist) => *playlist,
            _ => panic!("this is a single video, not a playlist"),
        }
    }

    #[cfg(test)]
    fn to_single_video(self) -> SingleVideo {
        match self {
            YoutubeDlOutput::SingleVideo(video) => *video,
            _ => panic!("this is a playlist, not a single video"),
        }
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    Io(std::io::Error),

    #[fail(display = "json error: {}", _0)]
    Json(serde_json::Error),

    #[fail(display = "non-zero exit code: {}", _0)]
    ExitCode(i32),
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

#[derive(Clone, Debug, Default)]
pub struct YoutubeDl {
    youtube_dl_path: Option<String>,
    /// -F
    format: Option<String>,
    /// --socket-timeout
    socket_timeout: Option<String>,
    /// --all-formats
    all_formats: bool,
    /// --username + --password
    auth: Option<(String, String)>,
    /// --user-agent
    user_agent: Option<String>,
    /// --referer
    referer: Option<String>,
    /// URL argument
    url: String,
}

impl YoutubeDl {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub fn youtube_dl_path<S: Into<String>>(&mut self, youtube_dl_path: S) -> &mut Self {
        self.youtube_dl_path = Some(youtube_dl_path.into());
        self
    }

    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }

    pub fn socket_timeout<S: Into<String>>(&mut self, socket_timeout: S) -> &mut Self {
        self.socket_timeout = Some(socket_timeout.into());
        self
    }

    pub fn user_agent<S: Into<String>>(&mut self, user_agent: S) -> &mut Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn referer<S: Into<String>>(&mut self, referer: S) -> &mut Self {
        self.referer = Some(referer.into());
        self
    }

    pub fn all_formats(&mut self, all_formats: bool) -> &mut Self {
        self.all_formats = all_formats;
        self
    }

    pub fn auth<S: Into<String>>(&mut self, auth: (S, S)) -> &mut Self {
        self.auth = Some((auth.0.into(), auth.1.into()));
        self
    }

    fn path(&self) -> &str {
        match &self.youtube_dl_path {
            Some(path) => path,
            None => "youtube-dl",
        }
    }

    fn process_args(&self) -> Vec<&str> {
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
        args.push("-J");
        args.push(&self.url);
        args
    }

    pub fn run(self) -> Result<YoutubeDlOutput, Error> {
        use serde_json::{json, Value};
        use std::process::{Command, Stdio};

        let process_args = self.process_args();
        let path = self.path();
        let output = Command::new(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(process_args)
            .output()?;

        if output.status.success() {
            let value: Value = serde_json::from_slice(&output.stdout)?;

            let is_playlist = value["_type"] == json!("playlist");
            if is_playlist {
                let playlist: Playlist = serde_json::from_value(value)?;
                Ok(YoutubeDlOutput::Playlist(Box::new(playlist)))
            } else {
                let video: SingleVideo = serde_json::from_value(value)?;
                Ok(YoutubeDlOutput::SingleVideo(Box::new(video)))
            }
        } else {
            Err(Error::ExitCode(output.status.code().unwrap_or(1)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::YoutubeDl;

    #[test]
    fn test_youtube_url() {
        let output = YoutubeDl::new("https://www.youtube.com/watch?v=7XGyWcuYVrg")
            .run()
            .unwrap()
            .to_single_video();
        assert_eq!(output.id, "7XGyWcuYVrg");
    }
}
