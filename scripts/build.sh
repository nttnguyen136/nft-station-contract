docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6


### Setting Up Environment

RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
TXFLAG="$NODE --chain-id $CHAIN_ID --node $RPC --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 -y"
WASM_FILE="./artifacts/$1.wasm"

echo "RPC: $RPC"
echo "CHAIN_ID: $CHAIN_ID"
echo "TXFLAG: $TXFLAG"
echo "WASM_FILE: $WASM_FILE"



TXHASH=$(aurad tx wasm store $WASM_FILE --from wallet $TXFLAG --output json | jq -r ".txhash")

echo "TXHASH: $TXHASH"
# {"txhash":"F8EF7D002F3921494549AF241A9634822555EE9C9A0A982C7FC2CF86DC272173",}
# TXHASH=$( echo $RES | jq -r ".txhash")
# TXHASH=F8EF7D002F3921494549AF241A9634822555EE9C9A0A982C7FC2CF86DC272173

echo "Fetch $RPC/tx?hash=0x$TXHASH"
# echo $(curl "$RPC/tx?hash=0x$TXHASH" | jq -r ".result.tx_result.log" | jq -r ".[0].events[-1].attributes[0].value")
# echo "CODE_ID: $CODE_ID"

