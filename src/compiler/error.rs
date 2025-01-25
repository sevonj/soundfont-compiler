use std::{error::Error, fmt::Display};

use crate::soundfont::SoundfontError;

#[derive(Debug, Clone)]
pub enum CompilerError {
    ProjectManifestCantOpen,
    SoundfontError { source: SoundfontError },
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::ProjectManifestCantOpen => {
                write!(f, "Can't access project manifest at given path.")
            }
            CompilerError::SoundfontError { source } => source.fmt(f),
        }
    }
}

impl Error for CompilerError {}

impl From<SoundfontError> for CompilerError {
    fn from(source: SoundfontError) -> Self {
        CompilerError::SoundfontError { source }
    }
}
