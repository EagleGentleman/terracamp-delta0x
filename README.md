# getting started

for a quick intro to terra contracts and setting see [getting started](./getting-started/README.md)


to compile the smart contract go to `/contract-coding/delta0x` then run

```
cargo wasm
docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.10.3
```

this will compile a .wasm file in the `/contract-coding/delta0x/artifacts` folder.

then, copy the .wasm file over to `/deploy/artifacts` 

add a `TERRA_MNEMOMIC_KEY` to your `.bash_profile` or `.zshrc` as a wallet for deployments, get some UST into the wallet for tequila testnet deployment

then, inside the `/deploy/scripts` folder run the below to deploy the .wasm smart contract

```
ts-node delta0x_deploy
```

and run the below to interact with the smart contract
```
ts-node delta0x_test 
```

# smart contracts

for further info on the smart contracts, our smart contracts are located within [contract-coding](./contract-coding/README.md) 

the [delta-0x folder](./delta-0x/README.md) contains the delta0x terra smart contract

# deployments & testing

for more info on testing & deployments of terra contracts see our [deploy folder](./deploy/README.md) 

