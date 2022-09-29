use clap::Args;
use std::path::PathBuf;
use crate::chunk_type::ChunkType;

// TODO: remove the 'pub's
#[derive(Args, Debug)]
pub struct Encode {
    pub file_path: Option<PathBuf>,
    pub chunk_type: Option<String>,
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct Decode {
    pub file_path: Option<PathBuf>,
    pub chunk_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct Remove {
    pub file_path: Option<PathBuf>,
    pub chunk_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct Print {
    pub file_path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_sub_command() {
        // assert_eq!( ,  );
    }
}