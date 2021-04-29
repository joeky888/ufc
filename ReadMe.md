### Features

* Single binary with zero setup
* Command output highlighting via `ufc <subcommand> <args>`
* Shell completion generating via `ufc completion` (Bash, Zsh, Fish, Powershell, Elvish)
* Command alias generating via `ufc alias` or `ufc ualias` (Bash, Zsh, Fish)
* Built-in time mode via `ufc --time <subcommand>` (like the [time command](https://en.wikipedia.org/wiki/Time_(Unix))) - Timing statistics when the subprogram exits
* Built-in watch mode via `ufc --watch 3s <subcommand>` (like the [watch command](https://en.wikipedia.org/wiki/Watch_(command))) - Duration of waiting for executing subcommand periodically. Values can be `1.5h`, `2m`, `5s` or `1h2m5s`
* Written in pure and safe rust (`#![forbid(unsafe_code)]` is used)

### Platform support

* Support Linux (macOS, FreeBSD and Android Termux should also work)
* Support Cygwin/Msys2 and git-bash for Windows (CMD and Powershell are supported if the subcommand exists)

### Benchmark

* Up to 7.5x faster than [grc](https://github.com/garabik/grc) (compiled with `cargo build --release`, thanks to [buffered printer](https://github.com/BurntSushi/termcolor))
  * Bechmarks on Linux with CPU Intel i5-8250U (4C8T) 3.400GHz, same regex with [Alacritty](https://github.com/alacritty/alacritty) terminal

| journalctl --no-pager -u NetworkManager           | Time  | Ratio | Colorful |
| ------------------------------------------------- | ----- | ----- | -------- |
| `journalctl --no-pager -u NetworkManager`         | 0.87s | 1.0   | No       |
| `ufc journalctl --no-pager -u NetworkManager`     | 1.94s | 2.23  | Yes      |
| `grc -es journalctl --no-pager -u NetworkManager` | 9.10s | 10.46 | Yes      |

| journalctl --no-pager           | Time    | Ratio | Colorful |
| ------------------------------- | ------- | ----- | -------- |
| `journalctl --no-pager`         | 45.36s  | 1.0   | No       |
| `ufc journalctl --no-pager`     | 107.76s | 2.375 | Yes      |
| `grc -es journalctl --no-pager` | 817.57s | 18.02 | Yes      |

### Examples

`ufc df -h`

![df](https://i.imgur.com/nd76Tu0.png)

`ufc ping google.com`

![ping](https://i.imgur.com/FGeIjGG.png)

`sudo ufc fdisk -l`

![fdisk](https://i.imgur.com/JAtfwxb.png)

`ufc top`

![top](https://i.imgur.com/MKjZyQR.png)

bash completion: `source <(ufc completion bash)`

![ping completion](https://i.imgur.com/mlV1zuR.png)

### Milestones

This porject is still at an early stage of development.

* Version 1.0.0 should support 10 subcommands
* Version 2.0.0 should support 20 subcommands
* Version 3.0.0 should support 30 subcommands

Currently supported commands are

* df
* docker (incomplete)
* dig
* du
* env
* fdisk
* free
* id
* ifconfig
* journalctl
* ping
* top

### TODO

* Upgrade to clap-rs v3 and remove the dependency `StructOpt`
* Add Crontab mode (like the [crontab command](https://en.wikipedia.org/wiki/Cron)) - Runs the subprogram as a scheduling daemon
* Colorize from stdout E.g. `cat /path/to/file.log | ufc`
* A benchmark script
* Use async rust and show statistics of cpu and memory usage

### Credits

This project is powered and inspired by

* [grc](https://github.com/garabik/grc) - All syntax files are modified from grc (Regex definitions in Rust and Python are incompatible)
* [manpage](https://en.wikipedia.org/wiki/Man_page) - All completions and docs are copied from manpage
* [cobra](https://github.com/spf13/cobra)/[clap](https://github.com/clap-rs/clap) - Completion generating
* [fancy-regex](https://github.com/fancy-regex/fancy-regex) - Library for regular expressions
* [fish](https://github.com/fish-shell/fish-shell) ([Issue#7451](https://github.com/fish-shell/fish-shell/issues/7451))
* [ohmyzsh](https://github.com/ohmyzsh/ohmyzsh)
* [ugc](https://github.com/joeky888/ugc) - My previous work in Golang

### Licence

Public domain