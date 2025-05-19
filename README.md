# Bybit API v5 client in Rust

This is simply a bybit V5 api connector using binance-rs implementation, Some parts of the api have not been completed yet...Anyone is welcome to branch/fork the repository and add their own upgrades, see [Development](development) section for details.

> ![CAUTION]
> This is a personal project, use at your own risk. Neither the original author,
> nor any of the contributors of this software shall ever be held responsible
> for your investment losses. Cryptocurrency investment is subject to high market risk.

# Table of Contents

-   [Description](#description)
-   [Features](#features)
-   [Development](#Development)
-   [Usage](#usage)
-   [Contact](#contact)
-   [Acknowledgments](#acknowledgments)

# Features

-   **REST API:**
    -   **Market Data:** Access to K-line, tickers, order book, and more. (see [`tests/market_test.rs`](https://github.com/unkuseni/rs_bybit/tests/market_test.rs))
    -   **Trade:** Functionality for placing, amending, and canceling orders. (see [`tests/trade_test.rs`](https://github.com/unkuseni/rs_bybit/tests/trade_test.rs))
    -   **Position:** Manage your trading positions. (see [`tests/position_test.rs`](https://github.com/unkuseni/rs_bybit/tests/position_test.rs))
    -   **Account & Asset:** These sections are currently under active development. See the [Under Development](#under-development) section for more details. (see [`tests/account_test.rs`](https://github.com/unkuseni/rs_bybit/tests/account_test.rs) for progress)
-   **Websocket API:**
    -   Support for subscribing to real-time public and private data streams. (see [`tests/ws_test.rs`](https://github.com/unkuseni/rs_bybit/tests/ws_test.rs))

# Development

If you want to contribubute please make sure to follow this setup. Install the precommit tool if you don't have it installed already and make sure to install the pre-commit hooks

## Precommit

Install the [`pre-commit` CLI tool](https://pre-commit.com/) and in this repo install the hooks.

### Install tool

```sh
brew install pre-commit
```

### Install hooks

```sh
pre-commit install
```

## Work in progress

Some part of the project is still under development. Please check back later for updates.

### Roadmap

-   [x] Order endpoints
-   [x] Position endpoints
-   [ ] Account endpoints
-   [ ] Asset endpoints

# Usage

This crate can be installed by adding the following to your `Cargo.toml`:

```toml
[dependencies]
rs_bybit = "*"
```

Take a look at tests for usage.

# Contact

if you have any issues contact me on X (twitter) @unkuseni

# Acknowledgments

## Credit

I like the project design of binance-rs and decided to use it. You might stumble upon some changes where both projects differ.

## Special thanks

A special thank you to [Sajjon](https://github.com/Sajjon) and [enseed](https://github.com/enseed-dev) for their valuable contributions to this project!
