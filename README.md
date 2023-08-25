lsport is a simple port scanner.

# Example


# Installation

If you have rust installed, you can install lsport with cargo:

```
cargo install --git https://github.com/nooralu/lsport
```

# Usage

Once installed, you can run lsport with `lsport <IPAddress> <Ports>`, where `<IPAddress>` is the IP address of the host you want to scan, and `<Ports>` is a comma-separated list of ports you want to scan. For example:

```
lsport 127.0.0.1 80,443,8080
```

## Options

The option `--threads` (or `-n` for short) can be used to specify the number of threads to use for the scan. The default is 5. For example:

e.g.

```
lsport 127.0.0.1 80,443,8080 -n 10
```

The option `--timeout` (or `-t` for short) can be used to specify the timeout for each port scan. The default is 500ms. For example:

e.g.

```
lsport 127.0.0.1 80,443,8080 -t 1000
```
