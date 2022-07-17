RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 -y"

UPDATE_METADATA='{"update_meta_data":{"token_id":"name", "metadata":{"name":"Updated Name"}}}'

CONTRACT=aura19vu8aa8l5fd44yzqus86snkz5fkn8akymdq42azrdetzxkrn9jzs28zmvg

echo "CONTRACT=$CONTRACT"
echo "UPDATE_METADATA=$UPDATE_METADATA"


# echo "aurad tx wasm execute $CONTRACT "$UPDATE_METADATA" --from wallet $TXFLAG -y"
aurad tx wasm execute $CONTRACT "$UPDATE_METADATA" --from wallet $TXFLAG -y