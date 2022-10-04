mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use clap::{Args, Parser, Subcommand};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use anyhow::{Result, Error};
use std::fs;
// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = commands::Cli::parse();
    match &cli.command {
        commands::Commands::Encode(encode) => {
            // file input
            let path = &encode.file_path;
            let data = fs::read(path)?;
            let mut p = png::Png::try_from(data.as_slice()).unwrap();
            p.append_chunk(Chunk::new(ChunkType::from_str(encode.chunk_type.as_str()).unwrap(), encode.message.clone().into_bytes()));
            fs::write(path, p.as_bytes())?;
            println!("your message injected!")
        },
        commands::Commands::Decode(decode) => {
            let path = &decode.file_path;
            let data = fs::read(path)?;
            let mut p = png::Png::try_from(data.as_slice()).unwrap();
            if let Some(msg) = p.chunk_by_type(&decode.chunk_type) {
                println!("message: {}", String::from_utf8(Vec::from(msg.data()))?)
            } else {
                println!("nothing found")
            }
        },
        commands::Commands::Remove(remove) => {
            let path = &remove.file_path;
            let data = fs::read(path)?;
            let mut p = png::Png::try_from(data.as_slice()).unwrap();
            p.remove_chunk(&remove.chunk_type)?;
            fs::write(path, p.as_bytes())?;
            println!("your message removed!")
        },
        commands::Commands::Print(print) => {
            let path = &print.file_path;
            let data = fs::read(path).unwrap();
            let p = png::Png::try_from(data.as_slice()).unwrap();
            println!("{:?}", p.as_bytes().as_slice())
        },
    }
    Ok(())
}