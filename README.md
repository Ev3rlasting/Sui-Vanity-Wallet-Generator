
# Vanity Sui Address Generator

This is a high-performance, multi-threaded **vanity address generator** for the Sui blockchain. It generates Sui wallet addresses based on a user-defined prefix using Rust and the Sui SDK. This tool uses Rayon to parallelize the work across multiple threads, making it suitable for generating vanity addresses efficiently.
**Use at your own risk as it's an experimental project!**

## Installation

To build this tool, you will need to have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

1. Clone the repository:

   ```bash
   git clone https://github.com/Ev3rlasting/Sui-Vanity-Wallet-Generator
   cd vanity-sui-generator
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the program with the desired prefix and number of threads:

   ```bash
   cargo run --release -- --prefix 0x1234 --threads 8
   ```

## Usage

The tool can be customized with the following command-line options:

```bash
USAGE:
    vanity-sui-generator [OPTIONS]

OPTIONS:
    -p, --prefix <PREFIX>    The desired prefix for the Sui address (e.g., 0x1234)
    -t, --threads <THREADS>  Number of threads to use for address generation (default: 8)
```

For example:

```bash
cargo run --release -- --prefix 0x1234 --threads 4
```

This will generate addresses starting with `0x1234` using 4 threads.

## Example Output

```
Found matching Sui address: 0x1234abcd1234...
Private Key: suiprivkey1xxxxxxxxxxxxxx
Thread SomeThreadID Attempts: 10000000
```

## Dependencies

- **Sui SDK**: Provides cryptographic functions and address handling.
- **Rayon**: Enables parallelism across CPU cores.
- **Clap**: For command-line argument parsing.
- **Regex**: To match the desired prefix pattern.
- 
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
