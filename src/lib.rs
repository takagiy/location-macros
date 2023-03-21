//! # location-macros
//!
//! A collection of macros for obtaining the location of the project root.
use crate::core::{locate, LocationKind};
use proc_macro::TokenStream;

mod core;
mod error;

/// Expands to a string literal that contains the absolute path of the root directory of the current crate.
///
/// Even if the crate is a workspace member, this expands to the path to the crate root, not the workspace root.
/// To obtain the path to the workspace root, you can use [`workspace_dir!`].
///
/// # Examples
///
/// ```
/// use location_macros::crate_dir;
///
/// let crate_dir = crate_dir!();
/// println!("The current crate root is {}", crate_dir);
/// ```
#[proc_macro]
pub fn crate_dir(input: TokenStream) -> TokenStream {
    locate(LocationKind::CrateDir, input)
}

/// Expands to a string literal that contains the absolute path of the root directory of the current workspace.
///
/// If the current crate is not in a workspace, this expands as same as [`crate_dir!`].
///
/// # Examples
///
/// ```
/// use location_macros::workspace_dir;
///
/// let workspace_dir = workspace_dir!();
/// println!("The current workspace root is {}", workspace_dir);
/// ```
#[proc_macro]
pub fn workspace_dir(input: TokenStream) -> TokenStream {
    locate(LocationKind::WorkspaceDir, input)
}
