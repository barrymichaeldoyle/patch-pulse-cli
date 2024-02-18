# Patch Pulse

![GitHub Actions](https://github.com/barrymichaeldoyle/patch-pulse-cli/workflows/Continuous%20Deployment/badge.svg)
![GitHub Actions](https://github.com/barrymichaeldoyle/patch-pulse-cli/workflows/Rust/badge.svg)
![License](https://img.shields.io/github/license/barrymichaeldoyle/patch-pulse-cli.svg)
![npm](https://img.shields.io/npm/v/patch-pulse.svg)
![npm](https://img.shields.io/npm/dm/patch-pulse.svg)
![GitHub stars](https://img.shields.io/github/stars/barrymichaeldoyle/patch-pulse-cli.svg?style=social)
![GitHub forks](https://img.shields.io/github/forks/barrymichaeldoyle/patch-pulse-cli.svg?style=social)

Patch Pulse is a CLI tool that identifies out of date dependencies in your `package.json` file.

## How to use

Just run the following command on a directory that has a `package.json` file in it:

```bash
npx patch-pulse
```

This will command with fetch and compare your version of each depency with the latest version available on npm.

## Built with Rust

This package is built with Rust but the binaries are wrapped in `npx` so you don't need to install anything.

## License

GNU GPLv3

## Author

This Patch Pulse CLI tool is maintained by [Barry Michael Doyle](https://barrymichaeldoyle.com).

## Contributing

If you would like to contribute to this project, please open an issue or a pull request. I would love help from the community to make this tool better.

Currently I'd appreciate help in cleaning up the codebase and adding more tests to the project. I'm relatively new to Rust and would appreciate any help in making the codebase more idiomatic.

To run the project locally, just add a `package.json` to the root of the repo to test changes and run `cargo run` to see the output.

Ultimately the most helpful thing right now would be raising issues and feature requests so I can understand what the community needs from this tool.

## Current Good First Issues

- Private NPM Registries are not supported yet. The tool works around this by warning that it isn't able to parse the latest version of a dependencies.
