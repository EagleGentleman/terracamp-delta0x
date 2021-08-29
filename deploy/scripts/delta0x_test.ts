import {
  MsgExecuteContract,
  MsgInstantiateContract,
} from "@terra-money/terra.js";
import {
  LCDClient,
  MsgStoreCode,
  MnemonicKey,
  isTxError,
} from "@terra-money/terra.js";
import * as fs from "fs";

const DEPLOYED_DELTA0X_ADDR = "terra1q8w3qe44dzus9ew84qpjq4dnzg5j734gx0az07";

(async () => {
  try {
    const mk = new MnemonicKey({
      mnemonic: process.env.TERRA_MNEMOMIC_KEY,
    });

    // Old local terra deployment format
    // connect to localterra
    // const terra = new LCDClient({
    //   URL: 'http://localhost:1317',
    //   chainID: 'localterra'
    // })

    const terra = new LCDClient({
      chainID: "tequila-0004",
      URL: "https://tequila-lcd.terra.dev",
    });

    const wallet = terra.wallet(mk);

    const deposit = new MsgExecuteContract(
      wallet.key.accAddress,
      DEPLOYED_DELTA0X_ADDR,
      {
        deposit: {},
      }

      // coins
      // {
      //   [`${market}`]: new Int(new Dec(amount).mul(1000000)).toString(),
      // },
    );
    const depositTx = await wallet.createAndSignTx({
      msgs: [deposit],
    });
    const depositTxResult = await terra.tx.broadcast(depositTx);
    console.log("STEVENDEBUG depositTxResult ", depositTxResult);

    /*
    [
        {
        "msg_index":0,
        "log":"",
        "events":[{
            "type":"execute_contract",
            "attributes":[{
                "key":"sender",
                "value":"terra1pk90dep5axrqcj9pj0vm7ted6pcz3t9d5vhpph"
            },{
                "key":"contract_address",
                "value":"terra1q8w3qe44dzus9ew84qpjq4dnzg5j734gx0az07"
            }]},{
                "type":"message",
                "attributes":[{"key":"action","value":"execute_contract"},
            {"key":"module","value":"wasm"},{"key":"sender",
            "value":"terra1pk90dep5axrqcj9pj0vm7ted6pcz3t9d5vhpph"}]}]}]
    */
  } catch (e) {
    console.log("STEVENDEBUG error ", e);
  }
})();
