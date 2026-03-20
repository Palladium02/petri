use std::{fmt, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum Engine {
    Dot,
    Neato,
    Fdp,
    Sfdp,
    Circo,
    Twopi,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Dot => "dot",
            Self::Neato => "neato",
            Self::Fdp => "fdp",
            Self::Sfdp => "sfdp",
            Self::Circo => "circo",
            Self::Twopi => "twopi",
        };

        write!(f, "{string}")
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Export {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long, value_enum, default_value_t=Engine::Dot, ignore_case=true)]
        engine: Engine,
    },
    Interactive {
        #[arg(short, long)]
        input: PathBuf,
    },
}

impl Command {
    pub const fn input(&self) -> &PathBuf {
        match self {
            Self::Export { input, .. } | Self::Interactive { input } => input,
        }
    }
}
