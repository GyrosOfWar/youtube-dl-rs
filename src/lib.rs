use serde::{Deserialize, Serialize};
use std::time::Duration;

schemafy::schemafy!("schema.json");

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum YoutubeDlOutput {
    Playlist(Box<Playlist>),
    SingleVideo(Box<SingleVideo>),
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    ExitCode(i32),
    Timeout,
    NoOutput { stderr: Option<String> },
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

    pub fn process_args(&self) -> Vec<&str> {
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

pub fn youtube_dl(args: Args) -> Result<YoutubeDlOutput, Error> {
    use std::process::{Command, Stdio};

    let process_args = args.process_args();
    println!("running youtube-dl {}", process_args.join(" "));
    let output = Command::new("youtube-dl")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(process_args)
        .output()?;

    if output.status.success() {
        serde_json::from_slice(&output.stdout).map_err(From::from)
    } else {
        Err(Error::ExitCode(output.status.code().unwrap_or(1)))
    }
}
