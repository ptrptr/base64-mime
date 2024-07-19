# base64-mime

The base64-mime library codes and decodes base64-encoded content. Also provided is the base64-cli command line utility for encoding and decoding data using the command line.

## Documentation

Usage can be displayed using `base64-cli --help`.

## Examples

### Encoding

Example of encoding data:
```bash
echo -n "Foo"|base64-cli
```

### Decoding

Example of decoding data:
```bash
echo -n "Rm9v"|base64-cli -d
```

## Building and Testing

### Building the `base64-cli` commandline utility

Build from source using the following steps
1. Clone this repository
2. Run `cargo install --path base64-cli`.

### Testing

Run tests with `cargo test`.

## Contributing

Contributions are welcome on Github via pull requests.

## License

The base64-mime library is licensed under Apache License, version 2.0.
