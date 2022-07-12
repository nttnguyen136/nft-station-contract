# Developing

If you have recently created a contract with this template, you probably could use some
help on how to build and test the contract, as well as prepare it for production. This
file attempts to provide a brief overview, assuming you have installed a recent
version of Rust already (eg. 1.58.1+).

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed. Currently, we are testing on 1.58.1+.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## Compiling and running tests

Now that you created your custom contract, make sure you can compile and run it before
making any changes. Go into the repository and do:

```sh
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test

# auto-generate json schema
cargo schema
```

### Understanding the tests

The main code is in `src/contract.rs` and the unit tests there run in pure rust,
which makes them very quick to execute and give nice output on failures, especially
if you do `RUST_BACKTRACE=1 cargo unit-test`.

We consider testing critical for anything on a blockchain, and recommend to always keep
the tests up to date.

## Generating JSON Schema

While the Wasm calls (`instantiate`, `execute`, `query`) accept JSON, this is not enough
information to use it. We need to expose the schema for the expected messages to the
clients. You can generate this schema by calling `cargo schema`, which will output
4 files in `./schema`, corresponding to the 3 message types the contract accepts,
as well as the internal `State`.

These files are in standard json-schema format, which should be usable by various
client side tools, either to auto-generate codecs, or just to validate incoming
json wrt. the defined schema.

## Preparing the Wasm bytecode for production

Before we upload it to a chain, we need to ensure the smallest output size possible,
as this will be included in the body of a transaction. We also want to have a
reproducible build process, so third parties can verify that the uploaded Wasm
code did indeed come from the claimed rust code.

To solve both these issues, we have produced `rust-optimizer`, a docker image to
produce an extremely small build output in a consistent manner. The suggest way
to run it is this:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.4
```

Or, If you're on an arm64 machine, you should use a docker image built with arm64.

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.4
```

To compile all contracts in the workspace deterministically, you can run:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

We must mount the contract code to `/code`. You can use a absolute path instead
of `$(pwd)` if you don't want to `cd` to the directory first. The other two
volumes are nice for speedup. Mounting `/code/target` in particular is useful
to avoid docker overwriting your local dev files with root permissions.
Note the `/code/target` cache is unique for each contract being compiled to limit
interference, while the registry cache is global.

This is rather slow compared to local compilations, especially the first compile
of a given contract. The use of the two volume caches is very useful to speed up
following compiles of the same contract.

This produces an `artifacts` directory with a `PROJECT_NAME.wasm`, as well as
`checksums.txt`, containing the Sha256 hash of the wasm file.
The wasm file is compiled deterministically (anyone else running the same
docker on the same git commit should get the identical file with the same Sha256 hash).
It is also stripped and minimized for upload to a blockchain (we will also
gzip it in the uploading process to make it even smaller).

## Deploying Contract To Aura Network

### Setting Up Environment

RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
NODE=(--node $RPC)
TX_FLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 --yes)

### Use aurad to deploy contract

```sh
# set wasm link file
CW721_WASM_FILE=("./artifacts/nft_cw721.wasm")
# store contract
RES=$(aurad tx wasm store artifacts/cw721_base.wasm --from wallet $TX_FLAG --output json)

# get the code id
CODE_ID=$(curl "$RPC/tx?hash=0x{txhash}" | jq -r ".result.tx_result.log"|jq -r ".[0].events[-1].attributes[0].value")

INIT='{"base_token_uri":"ipfs://Qme7VAFfCFFWrR2cwNk7huKrsHcU3DRew4K2t5uaZ84sVP", "cw721_code_id":180, "max_tokens_per_batch":10, "name":"NFT2", "num_tokens":100, "symbol":"2TFN"}'

# instantiate contract
# INIT='{"name":"init-flower","amount":0,"price":0}'
aurad tx wasm instantiate 179 "$INIT" --from wallet --label "NFT2-2TFN" $TX_FLAG -y --no-admin

# execute mint


CONTRACT=aura15vmtunt27pxcm40vlcrlpzz9aylnauuz5y96fvx6z6ctcn5epgvqxxuh4k
MINT='{"mint": {"token_id":3}}'

MINT='{"mint_to": {"recipient":"aura1afuqcya9g59v0slx4e930gzytxvpx2c43xhvtx", "token_id":4}}'

TRANSFER='{"transfer_nft": {"recipient":"aura1afuqcya9g59v0slx4e930gzytxvpx2c43xhvtx", "token_id":1}}'

aurad tx wasm execute $CONTRACT "$MINT" --from wallet $TX_FLAG -y

# And now access information from state:

QUERY='{"get_config": {}}'
QUERY='{"owner_of": {"token_id":"1"}}'
QUERY='{"num_tokens": {}}'
QUERY='{"tokens": {"owner":"aura1afuqcya9g59v0slx4e930gzytxvpx2c43xhvtx"}}'

aurad query wasm contract-state smart $CONTRACT "$QUERY" $NODE --output json

```
