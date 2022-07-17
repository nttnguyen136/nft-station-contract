RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 -y"


CONTRACT=aura19vu8aa8l5fd44yzqus86snkz5fkn8akymdq42azrdetzxkrn9jzs28zmvg
QUERY='{"nft_info":{"token_id":"name"}}'

echo "CONTRACT=$CONTRACT"
echo "QUERY=$QUERY"

# echo "aurad query wasm contract-state smart $CONTRACT "$QUERY" --node $RPC --output json"
aurad query wasm contract-state smart $CONTRACT "$QUERY" --node $RPC --output json
