# TMI (Tell Me Info)

TMI is a *blazingly fast* simple but useful CLI for automating system monitors across a variety of applications. With a single command, TMI can be used to spin up a constantly updating monitor for RAM, CPU, disk space, or more. This is especially useful in ricing in conjunction with applications like [polybar](https://github.com/polybar/polybar) or [Waybar](https://github.com/Alexays/Waybar).

## Installation 🚀

TMI is cross-platform, and [available on crates.io](https://crates.io/crates/tellmeinfo).

```sh
cargo install tellmeinfo
```

## Usage 🔧

```
Usage: tmi <COMMAND>

Commands:
  ram     
  cpu     
  disk    
  custom  
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Demo 📺

![TMI Demo](https://raw.githubusercontent.com/gusruben/tellmeinfo/refs/heads/main/tellmeinfo.gif)