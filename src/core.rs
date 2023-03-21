use crate::error::LocateError;
use proc_macro::{Literal, Span, TokenStream, TokenTree};
use std::{
    env,
    ffi::OsStr,
    fmt::{self, Display},
    os::unix::prelude::OsStrExt,
    path::Path,
    process::Command,
};
use syn::{parse::Nothing, parse_macro_input};

#[derive(Eq, PartialEq)]
pub enum LocationKind {
    CrateDir,
    WorkspaceDir,
}

impl Display for LocationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationKind::CrateDir => write!(f, "crate directory"),
            LocationKind::WorkspaceDir => write!(f, "workspace directory"),
        }
    }
}

pub fn locate(kind: LocationKind, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as Nothing);
    let dir = match locate_impl(LocationKind::CrateDir) {
        Ok(dir) => dir,
        Err(err) => {
            return syn::Error::new(
                Span::call_site().into(),
                format!("failed to locate {} ({})", kind, err),
            )
            .to_compile_error()
            .into()
        }
    };
    TokenTree::from(Literal::string(&dir)).into()
}

fn locate_impl(kind: LocationKind) -> Result<String, LocateError> {
    let cargo = env::var("CARGO").unwrap_or("cargo".to_owned());
    let mut command = Command::new(cargo);
    command.args(["locate-project", "--message-format", "plain"]);
    if kind == LocationKind::WorkspaceDir {
        command.arg("--workspace");
    }
    let output = match command.output() {
        Ok(output) => output,
        Err(error) => return Err(LocateError::ExecutionError { command, error }),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into();
        return Err(LocateError::CargoError {
            command,
            error: stderr,
        });
    }
    let output = OsStr::from_bytes(&output.stdout);
    let manifest_path = Path::new(output);
    manifest_path
        .parent()
        .expect("Path::parent()")
        .to_str()
        .ok_or(LocateError::InvalidPath(manifest_path.to_owned()))
        .map(|s| s.to_owned())
}
