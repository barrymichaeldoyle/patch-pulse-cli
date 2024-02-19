# Patch Pulse

![GitHub Actions Continuous Deployment](https://github.com/barrymichaeldoyle/patch-pulse-cli/workflows/Continuous%20Deployment/badge.svg) ![GitHub Actions Rust](https://github.com/barrymichaeldoyle/patch-pulse-cli/workflows/Rust/badge.svg) ![License](https://img.shields.io/github/license/barrymichaeldoyle/patch-pulse-cli.svg) ![npm Version](https://img.shields.io/npm/v/patch-pulse.svg) ![npm Downloads](https://img.shields.io/npm/dm/patch-pulse.svg) ![GitHub stars](https://img.shields.io/github/stars/barrymichaeldoyle/patch-pulse-cli.svg?style=social) ![GitHub forks](https://img.shields.io/github/forks/barrymichaeldoyle/patch-pulse-cli.svg?style=social)

![Patch Pulse Banner](assets/banner.png)

Patch Pulse is a CLI tool that identifies out-of-date dependencies in your `package.json` file, ensuring your project stays up-to-date with the latest versions available on npm.

## Table of Contents

- [How to Use](#how-to-use)
- [Built with Rust](#built-with-rust)
- [Features](#features)
- [Installation](#installation)
- [Example](#example)
- [License](#license)
- [Author](#author)
- [Contributing](#contributing)
- [Slack Bot](#slack-bot)
- [Support Me](#support-me)

## How to Use

To check your project's dependencies, navigate to your project directory and run:

```bash
npx patch-pulse
```

## Built with Rust

This package is built with Rust, offering high performance. The binaries are wrapped in npx, requiring no additional installation.

## Features

- **Easy to Use:** Run a single command to check all dependencies.
- **Comprehensive:** Checks dependencies, devDependencies, peerDependencies, optionalDependencies, and bundledDependencies.
- **Up-to-Date:** Compares against the latest versions on npm.

## Installation

No installation required! Just ensure you have [Node.js](https://nodejs.org) installed to use npx.

## Example

![Example Screenshot](assets/example.png)

## License

This project is licensed under GNU GPLv3 - see the [LICENSE](LICENSE) file for details.

## Author

This Patch Pulse CLI tool is maintained by [Barry Michael Doyle](https://barrymichaeldoyle.com).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Slack Bot

If you'd like to use the Patch Pulse Slack Bot to track when `npm` packages are updated, you can add it to your workspace using the following link:

https://slack.com/oauth/v2/authorize?client_id=180374136631.6017466448468&scope=chat:write,commands,incoming-webhook

## Support Me

If you like this tool, please consider [supporting me](https://www.buymeacoffee.com/barrycg) to keep the project alive and well-maintained.
