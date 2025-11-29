//! Includes the command line arguments for the application.

use std::{io::Error, path::PathBuf};

use clap::Parser;

/// Cli arguments for the application.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Relative repository path on disk given by the user, cwd if none specified.
    #[arg(
        short,
        long,
        default_value = ".",
        help = "Relative repository path on disk, cwd if none specified."
    )]
    pub repository_path: PathBuf,

    /// Relative pack directory path on disk given by the user from repository_path, cwd if none
    /// specified.
    #[arg(
        short,
        long,
        default_value = None,
        help =  "Relative pack directory path on disk, cwd if none specified.",
    )]
    pub pack_directory: Option<PathBuf>,
}

impl Cli {
    /// Gets the canon (absolute) path of the repository path.
    pub fn canon_repository_path(&self) -> Result<PathBuf, Error> {
        self.repository_path.canonicalize()
    }

    /// Gets the full absolute path of the joined repository path and pack directory (if Some).
    pub fn canon_full_path(&self) -> Result<PathBuf, Error> {
        let canon_repository_path = &self.canon_repository_path()?;

        if let Some(some_pack_directory) = &self.pack_directory {
            Ok(canon_repository_path.join(some_pack_directory))
        } else {
            Ok(canon_repository_path.clone())
        }
    }
}
