# Rlinuem

Rlinuem is a tool for performing enumeration scanning, process monitoring, and inotify-based folder watching. This utility provides a command-line interface for initiating different functionalities and exporting results.

## Features

- Enumeration Scanning: Initiate an enumeration scan using the `--enum` flag.
- Process Monitoring: Start process scanning using the `--pss` flag.
- Export Results:
  - Export enumeration scan results in text format using the `--report` flag.
  - Export enumeration scan results in JSON format using the `--jreport` flag.
  - Export commands used during the enumeration scanning using the `--cmds_enum` flag.
- HTTP File Download:
  - Download files from URLs and save them in the `/tmp` directory using the `--http_get` flag.
- Inotify Folder Watch:
  - Monitor specified paths for file system events using the `--watch` flag.
- Walker and Watcher:
  - Start a folder walker and initiate inotify watchers for each subfolder using the `--walker` flag.

## Installation

To use Rlinuem, you need to have Rust and Cargo installed on your system.

1. Clone the repository:

```bash
git clone https://github.com/<repository-url>.git
cd rlinuem
```

2. Build the project:

```bash
cargo build --release
```

## Usage

Run Rlinuem with various command-line options to utilize its features.

```bash
rlinuem [FLAGS] [SUBCOMMAND]
```

### Flags

- `--enum`: Begin enumeration scanning.
- `--pss`: Begin process scanning.
- `--report FILENAME`: Export enumeration scan results in txt format.
- `--jreport FILENAME`: Export enumeration scan results in JSON format.
- `--cmds_enum FILENAME`: Export commands used in enumeration scanning.
- `--http_get URL`: Download a file from the URL and save it in `/tmp`.
- `--watch PATH...`: Start inotify watcher for specified paths.
- `--walker PATH...`: Start folder walker and inotify watchers for subfolders.

### Subcommands

- `echo`: Echo a file to an NC listener.
  - `--ip IP`: IP address of the NC listener.
  - `--port PORT`: Port of the NC listener.
  - `--file FILE`: File to echo to the NC listener.

## Examples

- Begin enumeration scanning:

```bash
rlinuem --enum
```

- Start inotify watcher for specific paths:

```bash
rlinuem --watch /path/to/folder1 /path/to/folder2
```

- Echo a file to an NC listener:

```bash
rlinuem echo --ip 127.0.0.1 --port 1234 --file important.txt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to modify and expand this README to include more details about the code, its functionalities, and any additional information you want to provide to users.
