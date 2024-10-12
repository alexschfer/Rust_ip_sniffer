# Multi-Threaded Port Scanner

This is a multi-threaded port scanning tool written in Rust. It takes an IP address and a number of threads as input arguments and scans the ports of the given IP address to identify open ports.

## Features

- **Multi-threaded**: The program uses multiple threads to scan ports concurrently, speeding up the process.
- **Customizable thread count**: You can specify the number of threads to use for the scan.
- **IPv4/IPv6 support**: The tool supports both IPv4 and IPv6 addresses.

## Usage

To run the tool, you need to provide an IP address and optionally specify the number of threads to use for port scanning. If no thread count is provided, the program defaults to 4 threads.

### Command-line Options:

- `-j <threads>`: Specify the number of threads to use for scanning.
- `-h` or `-help`: Display the help message.

### Examples:

Replace `203.0.113.0` with any other IP address.

1. Scan the ports of an IP address using the default number of threads:
   ```bash
   cargo run -- 203.0.113.0
   ```
2. Scan the ports of an IP address using 100 threads:
   ```bash
   cargo run -- -j 100 203.0.113.0
   ```

## Installation

To compile and run the tool, you need to have Rust installed. Then, clone this repository and run the following commands:

```bash
git clone https://github.com/alexschfer/rust_ip_sniffer.git
cd rust_ip_sniffer
cargo run <arguments>
```

## How It Works
The program parses the command-line arguments and validates the input.
It spawns a number of threads to scan the ports concurrently, each thread handling a portion of the port range.
The results are collected via a channel and printed at the end, showing which ports are open.

## License
MIT License ([`LICENSE-MIT`](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
