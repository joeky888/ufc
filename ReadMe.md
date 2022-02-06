### 💡 Features

* Single binary with almost zero setup
* Command output highlighting
* Shell completion generating (Bash, Zsh, Fish, Powershell, Elvish)
* Built-in time mode via `--time` (like the [time command](https://en.wikipedia.org/wiki/Time_(Unix))) - Timing statistics when the subprogram exits
* Built-in watch mode via `--watch 3s` (like the [watch command](https://en.wikipedia.org/wiki/Watch_(command))) - Duration of waiting for executing subcommand periodically. Values can be `1.5h`, `2m`, `5s`, `5` or `1.5h2m5s`
* Boost mode via `--boost` - Make mass stdout/stderr print faster
* Written in safe rust

#### 🤔 *In a nutshell, ufc = [grc](https://github.com/garabik/grc) + [time](https://en.wikipedia.org/wiki/Time_(Unix)) + [watch](https://en.wikipedia.org/wiki/Watch_(command)) + shell completion + cross platform.*

### 💻 Platform support

* Support Linux (macOS, FreeBSD and Android Termux should also work)
* Support Cygwin/Msys2 and git-bash for Windows (CMD and Powershell are supported if the subcommand exists)

### ⚡️ Benchmark

* Up to 12x faster than [grc](https://github.com/garabik/grc) (compiled with `cargo build --release`)
  * Bechmarks on Linux with CPU Intel i5-8250U (4C8T) 3.400GHz, same regex on [Alacritty](https://github.com/alacritty/alacritty) terminal

#### Benchmark colorizer

| journalctl --no-pager               | Time    | Ratio | Boost | Colorful |
| ----------------------------------- | ------- | ----- | ----- | -------- |
| `journalctl --no-pager`             | 1m:02s  | 1.0   | No    | No       |
| `ufc --boost journalctl --no-pager` | 1m:07s  | 1.08  | Yes   | Yes      |
| `ufc journalctl --no-pager`         | 1m:15s  | 1.21  | No    | Yes      |
| `grc -es journalctl --no-pager`     | 12m:52s | 12.45 | No    | Yes      |

### 📖 Examples

`ufc df -h`

![df](https://i.imgur.com/nd76Tu0.png)

`ufc ping google.com`

![ping](https://i.imgur.com/FGeIjGG.png)

`sudo ufc fdisk -l`

![fdisk](https://i.imgur.com/JAtfwxb.png)

`ufc top`

![top](https://i.imgur.com/MKjZyQR.png)

### 🐚 Shell completion (Optional)

#### Bash: add this line to `~/.bashrc`

```sh
source <(ufc completion bash)
```

![bash](https://i.imgur.com/Uxzslae.png)

#### Zsh: add this line to `~/.zshrc`

```sh
source <(ufc completion zsh)
```

![zsh](https://i.imgur.com/BknF2At.png)

#### Fish: add this line to `config.fish`

```sh
eval (ufc completion fish)
```

![fish](https://i.imgur.com/1jUj0uH.png)

#### PowerShell: add this line to `profile.ps1`

```sh
ufc completion powershell | Out-String | Invoke-Expression
```

![powershell](https://i.imgur.com/38L2ne3.png)

### 🏆 Milestones

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

### 📔 TODO

* A built-in [timeout](https://linux.die.net/man/1/timeout) command functionality
* A built-in [gamemode](https://github.com/FeralInteractive/gamemode)
* Github badges
* Add more [ValueHints](https://docs.rs/clap/3.0.0-beta.2/clap/enum.ValueHint.html) for better shell completion generating
* Add Crontab mode and Daemon mode (like the [crontab command](https://en.wikipedia.org/wiki/Cron)) - Runs the subprogram as a scheduling daemon
* Colorize from stdout E.g. `cat /path/to/file.log | ufc`
* A benchmark script
* A built-in colored [pager](https://en.wikipedia.org/wiki/Terminal_pager)
* Unit test for the `fn colored_output()`
* Use async rust and show statistics of cpu and memory usage

### ❤️ Credits

This project is powered and inspired by

* [grc](https://github.com/garabik/grc) - All syntax files are modified from grc (Regex definitions in Rust and Python are incompatible)
* [termcolor](https://github.com/BurntSushi/termcolor) - Library for cross-platform terminal color
* [manpage](https://en.wikipedia.org/wiki/Man_page) - All completions and docs are copied from manpage
* [cobra](https://github.com/spf13/cobra)/[clap](https://github.com/clap-rs/clap) - Completion generating
* [fancy-regex](https://github.com/fancy-regex/fancy-regex) - Library for regular expressions
* [fish](https://github.com/fish-shell/fish-shell) ([Issue#7451](https://github.com/fish-shell/fish-shell/issues/7451))
* [ohmyzsh](https://github.com/ohmyzsh/ohmyzsh)
* [ugc](https://github.com/joeky888/ugc) - My previous work in Golang

### 📜 Licence

Public domain
