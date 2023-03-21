use std::{
    fmt::{self, Debug},
    io,
    path::PathBuf,
    process::Command,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocateError {
    #[error("faild to execute cargo (command: {:?}, error: {error})", CommandFormat(.command))]
    ExecutionError {
        command: Command,
        #[source]
        error: io::Error,
    },
    #[error("cargo exited with non-zero status (command: {:?}, error: {error})", CommandFormat(.command))]
    CargoError { command: Command, error: String },
    #[error("directory path contains invalid UTF-8 (path: {0})")]
    InvalidPath(PathBuf),
}

struct CommandFormat<'a>(&'a Command);

impl Debug for CommandFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command = self.0.get_program();
        write!(f, "{:?}", command)?;
        for arg in self.0.get_args() {
            write!(f, " {:?}", arg)?;
        }
        Ok(())
    }
}
