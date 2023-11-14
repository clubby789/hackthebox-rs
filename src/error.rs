use thiserror::Error;

pub type Result<T> = std::result::Result<T, HackTheBoxError>;

#[derive(Error, Debug)]
pub enum HackTheBoxError {
    #[error("this action requires authentication")]
    RequiresAuthentication,
    #[error("there was an http error")]
    Http(#[from] reqwest::Error),
    #[error("invalid json")]
    JSONParseError(reqwest::Error),
}
