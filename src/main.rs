use std::{thread::sleep, time, process::Command as SystemCommand};

use clap::{Parser, Subcommand};
use shlex::Shlex;
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
    Custom {
        command: String,
    }
}


fn main() {
    let args = Args::parse();

    // initialize everything
    let mut sys: Option<System> = None;
    let mut program: Option<String> = None;
    let mut custom_command_args: Option<Vec<String>> = None;
    match args.command {
        Command::RAM { .. } | Command::CPU { .. } => {
            sys = Some(System::new());
        },
        Command::Custom { ref command } => {
            let mut parts = Shlex::new(command.as_str());
            program = Some(parts.next().unwrap());
            custom_command_args = Some(parts.collect());
        }
    }
    
    loop {
        match args.command {
            Command::RAM { decimals } => {
                let sys = sys.as_mut().unwrap();
                sys.refresh_memory();
                println!("{:.1$} GiB", (sys.used_memory() as f64) / 1_000_000_000.0, decimals);
            },
            Command::CPU { decimals } => {
                let sys = sys.as_mut().unwrap();
                sys.refresh_cpu_usage();
                println!("{:.1$}%", sys.global_cpu_usage(), decimals);
            },
            Command::Custom { ref command } => {
                SystemCommand::new(program.as_ref().unwrap())
                .args(custom_command_args.as_ref().unwrap())
                .status()
                .expect(format!("Failed to execute: {}", command).as_str());
            }
        }

        sleep(time::Duration::from_secs(args.interval));
    }
}
