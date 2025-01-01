use std::{thread::sleep, time};

use clap::{Parser, Subcommand};
use sysinfo::System;

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
        #[arg(short, long, default_value_t = 2)]
        decimals: usize,
    },
    CPU {
        #[arg(short, long, default_value_t = 0)]
        decimals: usize,
    },
}


fn main() {
    let args = Args::parse();

    // initialize everything
    let mut sys;
    match args.command {
        Command::RAM { .. } | Command::CPU { .. } => {
            sys = System::new();
        },
    }
    
    loop {
        sleep(time::Duration::from_secs(args.interval));

        match args.command {
            Command::RAM { decimals } => {
                sys.refresh_memory();
                println!("{:.1$} GiB", (sys.used_memory() as f64) / 1_000_000_000.0, decimals);
            },
            Command::CPU { decimals } => {
                sys.refresh_cpu_usage();
                println!("{:.1$}%", sys.global_cpu_usage(), decimals);
            },
        }
    }
}
