use clap::Args;
use std::path::PathBuf;
use structopt;
use structopt::StructOpt;
use crate::chunk_type::ChunkType;

// TODO: remove the 'pub's
#[derive(StructOpt, Args, Debug)]
#[structopt(name = "basic")]
pub struct Encode {
    pub file_path: PathBuf,
    #[structopt(short, long, default_value = "ruSt")]
    pub chunk_type: String,
    pub message: String,
}

#[derive(StructOpt, Args, Debug)]
#[structopt(name = "basic")]
pub struct Decode {
    pub file_path: PathBuf,
    #[structopt(short, long, default_value = "ruSt")]
    pub chunk_type: String,
}

#[derive(StructOpt, Args, Debug)]
#[structopt(name = "basic")]
pub struct Remove {
    pub file_path: PathBuf,
    #[structopt(short, long, default_value = "ruSt")]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Print {
    pub file_path: PathBuf,
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