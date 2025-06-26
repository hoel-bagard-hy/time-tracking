# Time Tracking Tool

CLI tool to help keep track of what time is spent on.

### Index

- [Installation](#installation)
- [User Guide](#usage)
- [File format](#file-format)
- [Make Release](RELEASE-CHECKLIST.md)

## Installation

### Download

Precompiled binaries can be downloaded [here](https://github.com/hoel-bagard-hy/time-tracking/releases).

### Building

This program is written in Rust, so you will need to [install Rust](https://www.rust-lang.org/) in order to compile it.

```console
git clone git@github.com:hoel-bagard-hy/time-tracking.git
cd time-tracking
cargo build --release
```

The resulting binary will be in `./target/release/time-tracking`. You can check that the binary works with:

```console
./target/release/time-tracking --version
```

### Add binary to path

For convenience, you can place the binary in a folder in the PATH. For example:

```console
sudo cp ./target/release/time-tracking /usr/local/bin
```

or

```bash
cp ./target/release/time-tracking ~/.local/bin
```

If using `~/.local/bin`, you might need to add it to the PATH.

## Usage

By default the data is stored in/read from `~/.local/share/time-tracking/timesheet.csv`, this can be changed using the `--timesheet-path`.

### Log mode

The log mode is used to recording start/end time of each task.

In the morning/after lunch/after a break:

```bash
time-tracking log start <task name>
```

When switching to another task:

```bash
time-tracking log switch <task name>
```

When taking a break or at the end of the day:

```bash
time-tracking log end
```

### Recommended aliases

### Report mode

TODO

## File format

The data is store in a simple CSV format and can therefore be edited by hand if required. The timestamps use the [`%Y-%m-%d %H:%M:%S` format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

```code
<task name1>,<start time 1>,<end time 1>
<task name2>,<start time 2>,<end time 2>
```

For example

```code
emails,2025-06-29 09:30:00,2025-06-29 10:30:00
meeting,2025-06-29 10:30:00,2025-06-29 11:30:00
```

## TODO:

- Make Windows binary using [cross](https://github.com/cross-rs/cross) ? (see [here](https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml) for an example)
