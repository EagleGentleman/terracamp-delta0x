# Creating and deploying a contract

- Get some devtools
https://docs.terra.money/contracts/tutorial/setup.html#download-localterra

- Create a template (boilerplate) for contract creation
```
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --branch 0.10 --name my-first-contract
cd my-first-contract
```
source: https://docs.terra.money/contracts/tutorial/implementation.html#start-with-a-template

generate the .wasm file for deployment
```
cargo wasm

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
```
source: https://docs.terra.money/contracts/tutorial/implementation.html#optimizing-your-build

- Now, deployment. Best process rn is using terra js to do the deployments
source: 

- 