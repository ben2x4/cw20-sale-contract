# CW20 Sale Contract

This contract enables the sale of CW20 tokens for native tokens. CW20 tokens are deposited and a price in native tokens is set by the instantiator of the contract. The insantiator can change price and withdraw remaining CW20 tokens at any time. Funds used to purchase tokens are automatically transferred to the instantiator's bank balance. 

# Using the contract

## Instantiate 

`wasmd tx wasm instantiate <code_id> '{"cw20_address":"<cw20_contract_address>", "denom":"<denom>", "price":"<price>"}'  --from <address> --label="<label>" --gas="auto" --chain-id="<chain_id>"`

## Deposit CW20 Tokens

`wasmd tx wasm execute <cw20_contract_address> '{"send":{"amount":"<amount>","contract":"<sale_contract_address>","msg":""}}' --from <address> --chain-id="<chain_id>"`

## Set Price

Can only be called by the instantiator.

`wasmd tx wasm execute <sale_contract_address> '{"set_price":{"price":"<amount>","denom":"<denom>"}}' --amount "<funds (ie 1uatom)>" --from <creator address> --chain-id="<chain_id>"`

## Get Price

`wasmd query wasm contract-state smart <sale_contract_address> '{"get_price":{}}' --chain-id="<chain_id>"`

## Get CW20 Token Balance

`wasmd query wasm contract-state smart <sale_contract_address> '{"get_balance":{}}' --chain-id="<chain_id>"`

## Buy 

Please check price and CW20 token balance before making a purchase.

The contract determines correct quantity of CW20 tokens to transfer to the buyer based off the price and amount of native tokens sent. After a purchase, the native token funds are automatically transferred to the creators bank balance. 

`wasmd tx wasm execute <sale_contract_address> '{"buy":{}}' --amount "<funds (ie 1uatom)>" --from <address> --chain-id="<chain_id>"`

## Withraw Tokens

Can only be called by the instantiator.

`wasmd tx wasm execute <sale_contract_address> '{"withdraw_all":{}}' --from <creator address> --chain-id="<chain_id>"`


