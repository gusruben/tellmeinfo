use std::{thread::sleep, time, process::Command as SystemCommand};

use clap::{Parser, Subcommand};
use shlex::Shlex;
use sysinfo::{Disks, System};

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    RAM {
        #[arg(short, long, default_value_t = 2, help = "The number of decimal places to show")]
        decimals: usize,
        
        #[arg(short, long, default_value_t = 1, help = "The number of seconds between each refresh")]
        interval: u64,
    },
    CPU {
        #[arg(short, long, default_value_t = 0, help = "The number of decimal places to show")]
        decimals: usize,

        #[arg(short, long, default_value_t = 1, help = "The number of seconds between each refresh")]
        interval: u64,
    },
    Disk {
        disk_id: String,

        #[arg(value_parser = ["free", "used", "total"], default_value = "free", help = "The type of disk space to show")]
        r#type: String,

        #[arg(short, long, default_value_t = 1, help = "The number of seconds between each refresh")]
        interval: u64,

        #[arg(short, long, help = "Print the list of available disks and exit")]
        print_disks: bool,
    },
    Custom {
        command: String,

        #[arg(short, long, default_value_t = 1, help = "The number of seconds between each refresh")]
        interval: u64,
    }
}


fn main() {
    let args = Args::parse();

    // initialize everything
    let mut sys: Option<System> = None;
    let mut disks: Option<Disks> = None;
    let mut program: Option<String> = None;
    let mut custom_command_args: Option<Vec<String>> = None;
    match &args.command {
        Command::RAM { .. } | Command::CPU { .. } => {
            sys = Some(System::new());
        },
        Command::Custom { ref command, .. } => {
            let mut parts = Shlex::new(command.as_str());
            program = Some(parts.next().unwrap());
            custom_command_args = Some(parts.collect());
        },
        Command::Disk { print_disks, .. } => {
            disks = Some(Disks::new_with_refreshed_list());

            if *print_disks {
                for disk in disks.unwrap().list() {
                    println!("{} | {:.2?}GB/{:.2?}GB ({:.2?}GB free)", disk.name().to_str().unwrap(), 
                        (disk.total_space() - disk.available_space()) as f64 / 1_000_000_000.0, 
                        disk.total_space() as f64 / 1_000_000_000.0,
                        disk.available_space() as f64 / 1_000_000_000.0);
                }
                return;
            }
        },
    }
    
    loop {
        match args.command {
            Command::RAM { decimals, interval } => {
                let sys = sys.as_mut().unwrap();
                sys.refresh_memory();
                println!("{:.1$} GiB", (sys.used_memory() as f64) / 1_000_000_000.0, decimals);

                sleep(time::Duration::from_secs(interval));
            },
            Command::CPU { decimals, interval } => {
                let sys = sys.as_mut().unwrap();
                sys.refresh_cpu_usage();
                println!("{:.1$}%", sys.global_cpu_usage(), decimals);

                sleep(time::Duration::from_secs(interval));
            },
            Command::Disk { interval, ref disk_id, ref r#type, .. } => {
                let disks = disks.as_mut().unwrap();
                let disk = disks.list().iter().find(|disk| disk.name().to_str().unwrap() == disk_id).unwrap();
                match r#type.as_str() {
                    "free" => println!("{:.1$} GiB free", (disk.available_space() as f64) / 1_000_000_000.0, 2),
                    "used" => println!("{:.1$} GiB used", (disk.total_space() as f64 - disk.available_space() as f64) / 1_000_000_000.0, 2),
                    "total" => println!("{:.2$} GiB / {:.2$} GiB",
                        (disk.total_space() as f64 - disk.available_space() as f64) / 1_000_000_000.0,
                        (disk.total_space() as f64) / 1_000_000_000.0, 2
                    ),
                    _ => {}
                }
                sleep(time::Duration::from_secs(interval));
            },
            Command::Custom { ref command, interval} => {
                SystemCommand::new(program.as_ref().unwrap())
                .args(custom_command_args.as_ref().unwrap())
                .status()
                .expect(format!("Failed to execute: {}", command).as_str());

                sleep(time::Duration::from_secs(interval));
            },
        }
    }
}
