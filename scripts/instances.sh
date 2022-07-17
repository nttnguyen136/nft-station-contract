RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
TXFLAG="$NODE --chain-id $CHAIN_ID --node $RPC --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 -y"

# echo "$RPC/tx?hash=0x$1"

CODE_ID=$(curl "$RPC/tx?hash=0x$1" | jq -r ".result.tx_result.log" | jq -r ".[0].events[-1].attributes[0].value")

INIT='{"name":"name","symbol":"ANS","minter":"aura103d2neftkpce223utyz9sc55a8x8ktl44ue9v2"}'

LABEL="ANS Contract"

echo "CODE_ID: $CODE_ID"
echo "INIT: $INIT"
echo "LABEL: $LABEL"

HASH=$(aurad tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "$LABEL" $TXFLAG -y --no-admin --output json | jq -r ".txhash")

echo "$RPC/tx?hash=0x$HASH"
