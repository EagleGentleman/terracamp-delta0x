# getting started with terra smart contracts

setting up my local environment

following the guide here: https://docs.terra.money/contracts/tutorial/

# setting up a full node yourself

_tldr: did not get this working in a short time, found an provider: https://datahub.figment.io/_

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
