use clap::{Parser, Subcommand};
use crate::args;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(args::Encode),
    Decode(args::Decode),
    Remove(args::Remove),
    Print(args::Print),
}