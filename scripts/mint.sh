# pub struct Metadata {
#     pub image: Option<String>,
#     pub image_data: Option<String>,
#     pub external_url: Option<String>,
#     pub description: Option<String>,
#     pub name: Option<String>,
#     pub background_color: Option<String>,
#     pub animation_url: Option<String>,
#     pub youtube_url: Option<String>,
#     pub twitter_id: Option<String>,
#     pub discord_id: Option<String>,
#     pub telegram_id: Option<String>,
#     pub facebook_id: Option<String>,
#     pub attributes: Option<Vec<Trait>>,
# }

RPC="https://rpc.serenity.aura.network:443"
CHAIN_ID=serenity-testnet-001
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas-prices 0.0025uaura --gas auto --gas-adjustment 1.3 -y"

MINT='{"mint":{
    "token_id":"name",
    "owner":"aura103d2neftkpce223utyz9sc55a8x8ktl44ue9v2",
    "extension": {
        "name":"New Nme"
    }
}}'

CONTRACT=aura19vu8aa8l5fd44yzqus86snkz5fkn8akymdq42azrdetzxkrn9jzs28zmvg

aurad tx wasm execute $CONTRACT "$MINT" --from wallet $TXFLAG -y