### Features

* Single binary with zero setup
* Command output highlighting via `ufc <subcommand> <args>`
* Shell completion generating via `ufc completion` (Bash, Zsh, Fish, Powershell, Elvish)
* Command alias generating via `ufc alias` or `ufc ualias` (Bash, Zsh, Fish)
* Support macOS and Linux (Windows, FreeBSD and Android Termux should also work)
* Support root and non-root users
* Written in safe rust (`#![forbid(unsafe_code)]` is used)

### Examples

`ufc ping google.com`

![ping](https://i.imgur.com/tmjoQa0.png)

`source <(ufc completion bash)`

![ping completion](https://i.imgur.com/mlV1zuR.png)

`ufc df -h`

![df](https://i.imgur.com/0OP1hbW.png)

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
* ping
* top

### TODO

* Upgrade to clap-rs v3
* Add Watch mode (like the [watch command](https://en.wikipedia.org/wiki/Watch_(command))) - runs the subprogram every N seconds
* Add Crontab mode (like the [crontab command](https://en.wikipedia.org/wiki/Cron)) - runs the subprogram as a scheduling daemon
* Add Time mode (like the [time command](https://en.wikipedia.org/wiki/Time_(Unix))) - timing statistics when the subprogram exits

### Credits

This project is powered and inspired by

* [grc](https://github.com/garabik/grc) - All syntax files are modified from grc (Regex definitions in Rust and Python are incompatible)
* [manpage](https://en.wikipedia.org/wiki/Man_page) - All completions and docs are copied from manpage
* [cobra](https://github.com/spf13/cobra)/[clap](https://github.com/clap-rs/clap) - Completion generating
* [fish](https://github.com/fish-shell/fish-shell) ([Issue#7451](https://github.com/fish-shell/fish-shell/issues/7451))
* [ohmyzsh](https://github.com/ohmyzsh/ohmyzsh)
* [powerlevel10k](https://github.com/romkatv/powerlevel10k)/[gitstatus](https://github.com/romkatv/gitstatus) - The idea of combining zsh theme and external `gitstatus` command is inspiring
* [ugc](https://github.com/joeky888/ugc) - My previous work in Golang

### Licence

Public domain