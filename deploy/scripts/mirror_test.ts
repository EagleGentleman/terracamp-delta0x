import { LCDClient, MnemonicKey, StdTx } from "@terra-money/terra.js";
import { Mirror } from "@mirror-protocol/mirror.js";

(async () => {
  try {
    const terra = new LCDClient({
      chainID: "tequila-0004",
      URL: "https://tequila-lcd.terra.dev",
    });

    const mk = new MnemonicKey({
      mnemonic: process.env.TERRA_MNEMOMIC_KEY,
    });

    // const wallet = terra.wallet(mk);
    const mirror = new Mirror({
      lcd: terra,
      key: mk,
      mint: "terra1s9ehcjv0dqj2gsl72xrpp0ga5fql7fj7y3kq3w",
    });

    const wallet = mirror.lcd?.wallet(mirror.key);

    const positionIdx = await mirror.mint.getNextPositionIdx();
    // cosnt asset = await mirror.mint.getAssetConfig('terra18yx7ff8knc98p07pdkhm3u36wufaeacv47fuha')

    console.log("STEVENDEBUG positionIdx ", positionIdx);
    const austContractAddr = "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl";
    const mQQQContractAddr = "terra12s2h8vlztjwu440khpc0063p34vm7nhu25w4p9";

    const deposit = mirror.mint.deposit(positionIdx.next_position_idx, {
      info: {
        // token: {
        //   contract_addr: austContractAddr,
        // },
        native_token: {
            denom: 'uusd'
        }
      },
      amount: "100",
    });

    console.log("STEVENDEBUG deposit ", deposit);


    const depositTx = await wallet?.createAndSignTx({
        msgs: [deposit],
    });
    const depositTxResult = await mirror.lcd?.tx.broadcast(depositTx as StdTx);

    console.log("STEVENDEBUG depositTxResult ", depositTxResult);
  } catch (e) {
    console.log("STEVENDEBUG error ", e);
  }
})();
