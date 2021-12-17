my-project Smart Contract
==================

A [smart contract] written in [Rust] for an app initialized with [create-near-app]


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]

## vote token
near delete vote.fakeless.testnet fakeless.testnet
near create-account vote.fakeless.testnet --masterAccount fakeless.testnet

near deploy --force vote.fakeless.testnet --wasmFile ./contracts/token/target/wasm32-unknown-unknown/release/token.wasm --initFunction 'new_default_meta' --initArgs '{ "total_supply": "100000000000000"}'

near call vote.fakeless.testnet give_tokens_to '{"amount": "100"}' --account_id fakeless.testnet
near call vote.fakeless.testnet stake '{"amount": "100"}' --account_id fakeless.testnet



## fakeless news 
near delete news.fakeless.testnet fakeless.testnet
near create-account news.fakeless.testnet --masterAccount fakeless.testnet

near deploy --force news.fakeless.testnet --wasmFile ./contracts/contract//target/wasm32-unknown-unknown/release/news_validator.wasm --initFunction 'new_default_meta' --initArgs '{ "vote_token_address": "vote.fakeless.testnet"}'


near call news.fakeless.testnet add '{"hash_head": "headhash", "hash_body": "bodyhash", "uri": "google.com"}' --account_id fakeless.testnet
near view news.fakeless.testnet get_all '{}' --account_id fakeless.testnet
near call news.fakeless.testnet vote '{"index": 4, "is_like": false }' --account_id fakeless.testnet
near call news.fakeless.testnet vote '{"index": 6, "is_like": true }' --account_id fakeless.testnet
near call news.fakeless.testnet nft_mint '{"index": 4}' --account_id fakeless.testnet
near call news.fakeless.testnet nft_mint '{"index": 6}' --account_id fakeless.testnet

Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
