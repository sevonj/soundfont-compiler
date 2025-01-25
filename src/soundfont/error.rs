use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum SoundfontError {
    StringNonAscii,
    StringLimit { limit: usize, len: usize },

    SampleTooShort,
    SampleLoopTooShort,
    SampleLoopNotEnoughLead,
    SampleLoopNotEnoughTail,
    SampleTerminalNotNull,
}

impl Display for SoundfontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundfontError::StringNonAscii => write!(f, "Non-ascii string."),
            SoundfontError::StringLimit { limit, len } => {
                write!(
                    f,
                    "This string is too long. It must fit into {limit} bytes, but was {len} bytes long."
                )
            }

            SoundfontError::SampleTooShort => {
                write!(f, "Sample data must be at least 48 data points long.")
            }
            SoundfontError::SampleLoopTooShort => {
                write!(f, "The loop must be at least 32 data points long.")
            }
            SoundfontError::SampleLoopNotEnoughLead => {
                write!(f, "There must be at least 8 data points before startloop")
            }
            SoundfontError::SampleLoopNotEnoughTail => {
                write!(f, "There must be at least 8 data points after endloop")
            }
            SoundfontError::SampleTerminalNotNull => {
                write!(f, "Terminal sample must be null.")
            }
        }
    }
}

impl Error for SoundfontError {}
