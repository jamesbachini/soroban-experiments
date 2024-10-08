## Basic Deployment

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup target add wasm32-unknown-unknown

cargo install --locked stellar-cli --features opt

stellar network add --global testnet --rpc-url https://soroban-testnet.stellar.org:443  --network-passphrase "Test SDF Network ; September 2015"

stellar keys generate --global james --network testnet

stellar keys address james

cargo test

cargo build --target wasm32-unknown-unknown --release

stellar contract deploy  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm --source james --network testnet

stellar contract invoke --id CONTRACT_HERE --source james --network testnet -- hello --to RPC


## Optimization

Soroban contracts have a maximum size of 64KB. The following command can attempt to optimize the build: 

stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world.wasm


## Example Contracts

https://developers.stellar.org/docs/build/smart-contracts/example-contracts


## Misc Notes

stellar contract invoke --id CBJUHDREKER4OCMZHRAGS7B7PWWVNM5NFC64WEU4FJSKDAMYNN5VVL5G --source james --network testnet -- data_types

https://stellarchain.io/
https://stellar.expert/explorer/public

https://okashi.dev/playground

Wallet https://rabet.io/

https://friendbot.stellar.org?addr=


