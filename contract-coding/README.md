# Contract coding

coding the actual contract(s)

# Get a template

examples: https://github.com/CosmWasm/cosmwasm-examples/blob/main/contracts/escrow/src/contract.rs

go back to the cosmwasm version you need i.e. this commit
https://github.com/CosmWasm/cosmwasm-examples/commit/a50a020bbe7d4d8bb6969ac6c059d34cf316769e

# Compiling the contract for deployment

path used: `/Users/stevenli/Documents/github/terracamp-delta0x/contract-coding/cosmwasm-examples/escrow`

to generate the compiled .wasm file, then run,

```
cargo wasm

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.10.3

// or

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
```

to see the messages to send/use, run 

```
cargo schema
```

# Deployment

Copy the .wasm file in `/Users/stevenli/Documents/github/terracamp-delta0x/contract-coding/cosmwasm-examples/escrow/artifacts` to the `../deploy/artifacts` folder

Update the `escrow_deploy.ts` script with the right msg (figure it out from the `/schema` folder)  & deploy

# Packages

Mirror v2 (columbus-4) was imported from here: https://github.com/Mirror-Protocol/mirror-contracts/releases