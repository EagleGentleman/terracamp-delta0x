import { int, LCDClient, MnemonicKey, StdTx } from "@terra-money/terra.js";
import { Mirror } from "@mirror-protocol/mirror.js";

// testing using mainnet
(async () => {
  try {
    const terra = new LCDClient({
      chainID: "columbus-4",
      URL: "https://lcd.terra.dev",
    });

    const mk = new MnemonicKey({
      mnemonic: process.env.TERRA_MNEMOMIC_KEY,
    });

    const mirror = new Mirror({
      key: mk,
    });

    const result = await mirror.factory.getConfig();
    console.log("STEVENDEBUG result ", result);

    const wallet = terra.wallet(mk);

    const positionIdx = await mirror.mint.getNextPositionIdx();
    const currentPositions = await mirror.mint.getPositions(mk.accAddress);

    console.log("STEVENDEBUG positionIdx ", positionIdx);
    console.log("STEVENDEBUG currentPositions ", currentPositions);

    const austContractAddrTestnet =
      "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl";
    const mQQQContractAddrTestnet =
      "terra12s2h8vlztjwu440khpc0063p34vm7nhu25w4p9";
    const mTSLAContractAddrMainnet =
      "terra14y5affaarufk3uscy2vr6pe6w6zqf2wpjzn5sh";
    const mSLVContractAddrMainnet =
      "terra1kscs6uhrqwy6rx5kuw5lwpuqvm3t6j2d6uf2lp";
    const mEthContractAddrMainnet =
      "terra1dk3g53js3034x4v5c3vavhj2738une880yu6kx";

    const openPositionCollateral = {
      info: {
        // token: {
        //   contract_addr: mTSLAContractAddrMainnet,
        // },
        native_token: {
          denom: "uusd",
        },
      },
      // 1 UST
      amount: int`1000000`.toString(),
    };
    const openPosition = await mirror.mint.openPosition(
      openPositionCollateral,
      {
        token: {
          contract_addr: mEthContractAddrMainnet,
        },
      },
      2.1
    );
    console.log("STEVENDEBUG openPosition ", JSON.stringify(openPosition));
    const openPositionTx = await wallet?.createAndSignTx({
      msgs: [openPosition],
    });
    const openPositionTxResult = await mirror.lcd?.tx.broadcast(
      openPositionTx as StdTx
    );
    console.log("STEVENDEBUG openPositionTxResult ", openPositionTxResult);

    // take the borrowed asset and lp it

    // replace this with the mint amount (see log from openPositionTxResult)
    const mintAmount = 137
    const poolRes = await mirror.assets.mETH.pair.getPool();
    let poolRatio = parseInt(poolRes.assets[0].amount) / parseInt(poolRes.assets[1].amount);
    const poolAsset = Math.trunc(mintAmount / 2) - 1;
    const poolUst = Math.trunc(poolAsset * poolRatio);

    // source: https://github.com/Mirror-Protocol/mirror.js/blob/dbbac43e0fb71c9b6476893f9f3af71f89ff27d4/integration-test/testUserFlow.ts
    const increaseLiqAllowance = await mirror.assets['mETH'].token.increaseAllowance(mEthContractAddrMainnet, poolAsset)
    // console.log("STEVENDEBUG increaseLiqAllowance ", increaseLiqAllowance);
    const increaseLiqAllowanceTx = await wallet?.createAndSignTx({
      msgs: [increaseLiqAllowance],
    });
    const increaseLiqAllowanceResult = await mirror.lcd?.tx.broadcast(
      increaseLiqAllowanceTx as StdTx
    );
    console.log("STEVENDEBUG increaseLiqAllowanceResult ", increaseLiqAllowanceResult);

    const provideLiquidity = await mirror.assets['mETH'].pair.provideLiquidity([
      {
        info: { native_token: { denom: 'uusd' } },
        // 1 UST
        amount: poolUst.toString()
      },
      {
        info: { token: { contract_addr: mEthContractAddrMainnet } },
        amount: poolAsset.toString()
      }
    ])
    console.log("STEVENDEBUG provideLiquidity ", provideLiquidity);

    const provideLiquidityTx = await wallet?.createAndSignTx({
      msgs: [provideLiquidity],
    });
    const provideLiquidityResult = await mirror.lcd?.tx.broadcast(
      provideLiquidityTx as StdTx
    );
    console.log("STEVENDEBUG provideLiquidityResult ", provideLiquidityResult);


  } catch (e) {
    console.log("STEVENDEBUG error ", e);
  }
})();

/*
// old deposit code (not working, needs tweaking)

const asset = await mirror.mint.getAssetConfig(mTSLAContractAddrMainnet)
console.log("STEVENDEBUG asset ", asset);

// const test1 = mirror.mint.

const deposit = mirror.mint.deposit(0, {
  info: {
    // token: {
    //   contract_addr: mTSLAContractAddrMainnet,
    // },
    native_token: {
        denom: 'uusd'
    }
  },
  amount: "1",
});

console.log("STEVENDEBUG deposit ", deposit);

const depositTx = await wallet?.createAndSignTx({
    msgs: [deposit],
});
const depositTxResult = await mirror.lcd?.tx.broadcast(depositTx as StdTx);

console.log("STEVENDEBUG depositTxResult ", depositTxResult);
*/
