# PM2 GUI Manager

A lightweight native desktop application for managing [PM2](https://pm2.keymetrics.io/) processes through a graphical interface.

Built with **Rust** and [egui](https://github.com/emilk/egui), PM2 GUI Manager provides a simple overview of your running processes, basic process controls, and access to application logs without having to rely exclusively on the terminal.

## Features

### Process management

- List PM2-managed processes.
- Display process ID, name, status, uptime, and restart count.
- Start stopped processes.
- Stop running processes.
- Restart individual processes.
- Restart all processes.
- Stop all processes.
- Reset statistics for all processes.

### Log viewer

- Inspect the standard output (`stdout`) of a process.
- Inspect the error output (`stderr`) separately.
- Retrieve the latest 100 lines for each output stream.
- Scroll through logs in separate panels.

### Desktop interface

- Native desktop application.
- Built with Rust and egui.
- Automatically refreshes the process list.

## Requirements

Before running the application, make sure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed.
- [PM2](https://pm2.keymetrics.io/) installed and available in your system `PATH`.
- At least one process managed by PM2 to see useful information in the interface.

You can verify that PM2 is available by running:

```bash
pm2 ls
```

## Getting started

Clone the repository:

```bash
git clone https://github.com/elenarc-dev/pm2-gui-manager.git
cd pm2-gui-manager
```

Run the application in development mode:

```bash
cargo run
```

To build an optimized executable:

```bash
cargo build --release
```

The resulting executable will be generated under:

```text
target/release/
```

## Technology stack

| Technology | Purpose |
|---|---|
| Rust | Application logic |
| eframe | Native application framework |
| egui | Graphical user interface |
| egui_extras | Additional UI components and image support |
| PM2 CLI | Process management and log retrieval |

## How it works

The application periodically executes `pm2 ls` to retrieve the list of managed processes.

It parses the command output and updates the process table in the graphical interface. Process operations are executed through PM2 CLI commands.

When a process is selected for log inspection, the application retrieves its latest output and error lines using `pm2 logs`.

## Current limitations

This project is an early-stage implementation. Some limitations are worth keeping in mind:

- Process information is extracted by parsing PM2 CLI table output, rather than using a structured API.
- The process list refreshes periodically, but log retrieval is performed on demand.
- The log viewer displays a snapshot of the latest 100 lines; it does not provide continuous streaming.
- There is no configuration interface yet.
- The application does not currently provide process creation, deletion, or ecosystem configuration management.
- Error handling and compatibility across different PM2 output formats can be improved.

## Project status

PM2 GUI Manager is an experimental desktop utility intended to make everyday PM2 operations more accessible through a graphical interface.

Contributions, suggestions, and bug reports are welcome.

## License

No license has been specified in the repository yet.
