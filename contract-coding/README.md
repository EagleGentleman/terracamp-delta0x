# Contract coding

coding the actual contract(s)

examples: https://github.com/CosmWasm/cosmwasm-examples/blob/main/contracts/escrow/src/contract.rs

# Compiling the contract for deployment

```
cargo wasm

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
```