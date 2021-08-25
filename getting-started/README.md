# getting started with terra smart contracts

setting up my local environment

following the guide here: https://docs.terra.money/contracts/tutorial/

# setting up a full node yourself

_tldr: did not get this working in a short time, found an provider for mainnet and testnet: https://datahub.figment.io/_

- general guide for aws setup
https://www.reddit.com/r/ethereum/comments/jg29m0/guide_for_full_node_on_aws_with_reasonable_amount/

- connecting to ec2 via ssh
https://us-west-1.console.aws.amazon.com/ec2/v2/home?region=us-west-1#ConnectToInstance:instanceId=i-00acd4e7a391646d5

- setting up the terra node
https://docs.terra.money/node/installation.html#building-terra-core

- fix `Makefile:37: *** gcc not installed for ledger support, please install or set LEDGER_ENABLED=false.  Stop.`
vim into the Makefile and update `LEDGER_ENABLED` from true to false

- fix `... cgg ...` not available or something
sudo yum install gcc

# connecting to that node for deployment later

- to deploy you will need to connect to your localterra, mainnet or tesnet node see here on how
https://docs.terra.money/terracli/#connecting-to-a-remote-full-node


to deploy you'll also need to add a key (account/wallet) to terrad/terracli
https://docs.terra.money/terracli/keys.html#generate-keys

# writing and deploying the contract

- install terracli (now terrad) which is the cli tool to do deployments
https://docs.terra.money/node/installation.html#hardware-requirements

```
Steven Li
Participant
12:52 PM
📚Jason Stallings for deployments we should use terracli right? for some reason I ran make install from this guide: https://docs.terra.money/node/installation.html#step-2-get-terra-core-source-code but terracli still isn't installed for me. is there another way to install terracli or another tool i can use? Thanks!
docs.terra.money
Installation | Terra Docs
Terra is a blockchain protocol that provides fundamental infrastructure for a decentralized economy and enables open participation in the creation of new financial primitives to power the innovation of money.

📚Jason Stallings
Mentor
12:53 PM
Correct! Let me review the install instructions real quick.
I installed it a while back and don't remember the steps I followed.
Are you working on Mac?

stevenli
Steven Li
Participant
12:53 PM
yes mac

📚Jason Stallings
Mentor
12:54 PM
Ah I think I know what's happening.
What do you get if you run:

echo $GOPATH
I get /Users/jason.stallings/go
The install process just installs the binaries in the go specific path, you'll need to add this to your global PATH.
You can try running this:

export PATH=$PATH:$(go env GOPATH)/bin

Then see if terracli is installed

stevenli
Steven Li
Participant
12:56 PM
thanks! let me try that

stevenli
Steven Li
Participant
12:58 PM
mm, it seems like terracli still doesn't show up for me, but I also retried make install and get this error on my mac
Screen Shot 2021-08-24 at 12.57.44 PM.png
(244.37 kB)

📚Jason Stallings
Mentor
1:00 PM
hmm, I'm seeing the same, give me a few to debug!
👍 1 
What does go version output?
I'm going to try go 1.17 to see if it resolves the issue.

stevenli
Steven Li
Participant
1:02 PM
Screen Shot 2021-08-24 at 1.02.00 PM.png
(24.89 kB)

stevenli
Steven Li
Participant
1:02 PM
i get 1.17

📚Jason Stallings
Mentor
1:02 PM
dang!
does terrad seem to be installed?

stevenli
Steven Li
Participant
1:03 PM
oh wow, terrad works
Screen Shot 2021-08-24 at 1.03.12 PM.png
(411.87 kB)

📚Jason Stallings
Mentor
1:04 PM
I'm wondering if terracli got merged into terrad

stevenli
Steven Li
Participant
1:05 PM
can i do contract deployments to a testnet with terrad by any chance?

📚Jason Stallings
Mentor
1:05 PM
It appears that way!
I would try out the terracli commands on terrad, they seem to work.
```

- writing the contract & prepping for deploy
https://docs.terra.money/contracts/tutorial/implementation.html#optimizing-your-build

you'll need to run both of below commands to generate a .wasm file to deploy 
```
cargo wasm

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
```



