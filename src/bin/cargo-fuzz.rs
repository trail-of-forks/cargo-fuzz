use anyhow::Result;
use cargo_fuzz::{Command, RunCommand};
use clap::Parser;

fn main() -> Result<()> {
    Command::parse().run_command()
}
