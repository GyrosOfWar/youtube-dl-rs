use failure::Fail;
use serde::{Deserialize, Serialize};
use std::time::Duration;

schemafy::schemafy!("schema.json");

#[derive(Serialize, Debug, Deserialize)]
pub enum YoutubeDlOutput {
    Playlist(Box<Playlist>),
    SingleVideo(Box<SingleVideo>),
}

impl YoutubeDlOutput {
    pub fn title(&self) -> Option<&str> {
        match self {
            YoutubeDlOutput::Playlist(playlist) => playlist.title.as_ref().map(String::as_str),
            YoutubeDlOutput::SingleVideo(video) => Some(&video.title),
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
pub struct Args {
    pub youtube_dl_path: Option<String>,
    /// -F
    pub format: Option<String>,
    /// --socket-timeout
    pub socket_timeout: Option<String>,
    /// --all-formats
    pub all_formats: bool,
    /// --username + --password
    pub auth: Option<(String, String)>,
    /// --user-agent
    pub user_agent: Option<String>,
    /// --referer
    pub referer: Option<String>,
    /// Duration after which the child process is killed
    pub child_timeout: Option<Duration>,
    /// URL argument
    pub url: String,
}

impl Args {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
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
}

pub fn run(args: Args) -> Result<YoutubeDlOutput, Error> {
    use serde_json::{json, Value};
    use std::process::{Command, Stdio};

    let process_args = args.process_args();
    let path = args.path();
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
