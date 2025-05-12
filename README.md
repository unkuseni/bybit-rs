# AN API CONNECTOR FOR BYBIT V5 API IN RUST

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Installation](#Installation)
- [Usage](#usage)


### DESCRIPTION
This is simply a bybit V5 api connector using binance-rs implementation, Some parts of the api have not been completed yet...Anyone is welcome to branch/fork the repository and add their own upgrades. If you think you've made substantial improvements to the module, submit a pull request and we'll gladly take a look.

### FEATURES

- **REST API:**
  - **Market Data:** Access to K-line, tickers, order book, and more. (see [`tests/market_test.rs`](https://github.com/unkuseni/bybit-rs/tests/market_test.rs))
  - **Trade:** Functionality for placing, amending, and canceling orders. (see [`tests/trade_test.rs`](https://github.com/unkuseni/bybit-rs/tests/trade_test.rs))
  - **Position:** Manage your trading positions. (see [`tests/position_test.rs`](https://github.com/unkuseni/bybit-rs/tests/position_test.rs))
  - **Account & Asset:** These sections are currently under active development. See the [Under Development](#under-development) section for more details. (see [`tests/account_test.rs`](https://github.com/unkuseni/bybit-rs/tests/account_test.rs) for progress)
- **Websocket API:**
  - Support for subscribing to real-time public and private data streams. (see [`tests/ws_test.rs`](https://github.com/unkuseni/bybit-rs/tests/ws_test.rs))

### INSTALLATION

this module can be installed by adding the following to your `Cargo.toml`:

```
[dependencies]
rs_bybit = "*"

```
### USAGE  

Take a look at tests for usage.


### CONTACT
if you have any issues contact me on twitter @unkuseni


## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.


## UNDER DEVELOPMENT
Some part of the project is still under development. Please check back later for updates. 

...ACCOUNT RELATED FUNCTIONS ARE STILL UNDER DEVELOPMENT.
...ASSET RELATED FUNCTIONS ARE STILL UNDER DEVELOPMENT.


## CREDIT

I like the project design of binance-rs and decided to use it. You might stumble upon some changes where both projects differ.

## SPECIAL THANKS

A special thank you to [Sajjon](https://github.com/Sajjon) and [enseed](https://github.com/enseed-dev) for their valuable contributions to this project!
