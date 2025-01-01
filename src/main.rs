use std::{thread::sleep, time};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
#[clap(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    interval: u64,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    RAM {
    },
    GPU {
    },
}


fn main() {
    let args = Args::parse();
    
    loop {
        sleep(time::Duration::from_secs(args.interval));

        match args.command {
            Command::RAM { .. } => {
                println!("RAM");
            },
            Command::GPU { .. } => {
                println!("GPU");
            },
        }
    }
}
