use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to mount file or directory: {}", 0)]
    FailedMount(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::FailedMount(format!("{}", err))
    }
}
