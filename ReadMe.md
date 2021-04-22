Supersedes my [previous work](https://github.com/joeky888/ugc).

Rewritten in safe rust (`#![forbid(unsafe_code)]` is used).

### Features

* Single binary with zero setup
* Command output highlighting via `ufc <subcommand> <args>`
* Shell completion generating via `ufc completion` (Bash, Zsh, Fish, Powershell, Elvish)
* Command alias generating via `ufc alias` or `ufc ualias` (Bash, Zsh, Fish)
* Support macOS and Linux (Windows and FreeBSD should also work)
* Support root and non-root users

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

### TODO

* Upgrade to clap 3

### Credits

This project is inspired by

* [grc](https://github.com/garabik/grc)
* [manpage](https://en.wikipedia.org/wiki/Man_page)
* [cobra](https://github.com/spf13/cobra)/[clap](https://github.com/clap-rs/clap)
* [fish](https://github.com/fish-shell/fish-shell)
* [ohmyzsh](https://github.com/ohmyzsh/ohmyzsh)

### Licence

Public domain