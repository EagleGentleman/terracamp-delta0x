# Deploying to terra columbus-4 testnet (tequila)

Set up the following package versions:
```
Terra.js 1.8.9
Cosmwasm 0.10
```
Only the above versions work with columbus-4 testnet (tequila). The newer versions are for columbus-5

Add the .wasm compiled smart contract to `/artifacts`

Inside the `/scripts`, do:

```
npm install
npm install -g ts-node
```

Update the main.ts to do the correct instantiation msg, see the line below for what to update

```
const instantiate = new MsgInstantiateContract(
    wallet.key.accAddress,
    code_id_num,
    {
    count: 5,
    },
    { uluna: 100, ukrw: 100 } // init coins
);
```

Now inside `/scripts` run `ts-node main` and your smart contract will be uploaded and instantiated on terra