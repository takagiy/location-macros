use crate::core::{locate, LocationKind};
use proc_macro::TokenStream;

mod core;
mod error;

#[proc_macro]
pub fn crate_dir(input: TokenStream) -> TokenStream {
    locate(LocationKind::CrateDir, input)
}

#[proc_macro]
pub fn workspace_dir(input: TokenStream) -> TokenStream {
    locate(LocationKind::WorkspaceDir, input)
}
