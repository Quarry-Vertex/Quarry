# Quickstart

Installing contract dependencies
```shell
cd contracts
forge install https://github.com/OpenZeppelin/openzeppelin-foundry-upgrades --no-git
forge install https://github.com/OpenZeppelin/openzeppelin-contracts-upgradeable --no-git
forge install https://github.com/OpenZeppelin/openzeppelin-contracts --no-git
```

# Testing
## Running Anvil Node
- run anvil
```sh
anvil
```
- deploy the smart contract
```sh
forge create --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url http://localhost:8545 --legacy --unlocked src/PoolShares.sol:PoolShares --constructor-args "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" "Quarry" "QRY" ""
```
- the address passed in is the oracle address
    - this should be the first account shown from anvil
    - this first address is the oracle address
    - the first private key is the oracle private key
        - these are values in `serverConnect.js`

## tooling
- **run.py**

## QuickNode endpoints
```sh
curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/ \
    -X POST \
    -H "Content-Type: application/json" \
    --data '{ "method": "getbestblockhash" }'

curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/ \
  -X POST \
  -H "Content-Type: application/json" \
  --data '{"method": "getblock", "params": ["0000000000000000000139685895880c667cd5424e816cc5dd8f0b37d72904dc"]}'

curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/ \
    -X POST \
    -H "Content-Type: application/json" \
    --data '{"method": "getrawtransaction", "params": ["fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33", 0]}'

curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/     -X POST     -H "Content-Type: application/json"     --data '{"method": "getrawtransaction", "params": ["daae30696577cde2117c7391ece3b445885a4d21f10cf24a57c23762d9dd396c", 1]}'
```

- the last command should return address and values 
    - first tx address is always coinbase transaction
        - only one we look at
    - values need to be mult by 100mil for satoshi's
    - set chain tip needs to provide a block struct

index <- transaction index in the block (quicknode call)
471
txid <- initial blockstream get txs call
0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba
block hash <- initial blockstream get txs call
00000000003094b8263777a801e959d8280ef5cfc7f94a6a2f936f0ffb9d4803
merkle root <- get block header call
3bf0614fc39756da9bd31f28a0e9568e57893a28ee2be5777f163da2e330f8c2
proof <- get txout proof call
00e0ff3ffa92350f47e83163e051fceb3efdd02998d5606e0c01691cf64f05a200000000c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b137dff65ffff001d85dc0ab62e0200000bfa4847b767a2a99f70d90b4ff779e0f09729a53e098e76a17d9a87d94fb152459a175af157234839a541f62545dbff7ed2e7dda993bff773718e270ad538bc5703e7c5b6ff141abc19fdea4fe67bca803d383f923ab80407d96d61315d155a1c221c70b1e4d8a2a19d3fa03c5442749bde9090259fa7b9da49557df3f9f20dae1db5067c62e63f0dd527d9bf4a69f01342b5bb246cbdef86fe3bbb775a1b1432249be97c0c03adbc6e759c0154110723403feefb966fb0f2b5f0d83bc5c1a64db859616f97418d68bcefbba895de59b6927ab6d1f8440bfc7b5ad039fa271acbba25ab121aa5925d28467028c616396652e710e0a799f02d7e570d003f732a0ff46a2674e0be7ba6c204d4901801431009c221e8c8a96f323e4c0af3b7603aa2d3294ed64cdbe78d7786268f4c5de209eb0fbe25e5a9f196cadd4d0d5b04ed20b8300a807bfaacd6512dc08065d7142cab058c9cd9c11c48a816ecd19e1c842d03abad02

curl https://billowing-fittest-feather.btc-testnet.quiknode.pro/4613ae0ad4238d1261f99cc2cd8baa48f6b96d83/ \
    -X POST \
    -H "Content-Type: application/json" \
    --data '{"method": "gettxoutproof", "params": [["0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba"],"00000000003094b8263777a801e959d8280ef5cfc7f94a6a2f936f0ffb9d4803"]}'

curl https://billowing-fittest-feather.btc-testnet.quiknode.pro/4613ae0ad4238d1261f99cc2cd8baa48f6b96d83/ \
    -X POST \
    -H "Content-Type: application/json" \
    --data '{"method": "gettxoutproof", "params": [["6f173ff5d55a4613c6837633dbac06485be9f5a8bb22d08451c5fd9321911309"],"00000000003094b8263777a801e959d8280ef5cfc7f94a6a2f936f0ffb9d4803"]}'

{"result":"00e0ff3ffa92350f47e83163e051fceb3efdd02998d5606e0c01691cf64f05a200000000c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b137dff65ffff001d85dc0ab62e0200000bfa4847b767a2a99f70d90b4ff779e0f09729a53e098e76a17d9a87d94fb152459a175af157234839a541f62545dbff7ed2e7dda993bff773718e270ad538bc5703e7c5b6ff141abc19fdea4fe67bca803d383f923ab80407d96d61315d155a1c221c70b1e4d8a2a19d3fa03c5442749bde9090259fa7b9da49557df3f9f20dae1db5067c62e63f0dd527d9bf4a69f01342b5bb246cbdef86fe3bbb775a1b1432249be97c0c03adbc6e759c0154110723403feefb966fb0f2b5f0d83bc5c1a64db859616f97418d68bcefbba895de59b6927ab6d1f8440bfc7b5ad039fa271acbba25ab121aa5925d28467028c616396652e710e0a799f02d7e570d003f732a0ff46a2674e0be7ba6c204d4901801431009c221e8c8a96f323e4c0af3b7603aa2d3294ed64cdbe78d7786268f4c5de209eb0fbe25e5a9f196cadd4d0d5b04ed20b8300a807bfaacd6512dc08065d7142cab058c9cd9c11c48a816ecd19e1c842d03abad02","error":null,"id":null}
