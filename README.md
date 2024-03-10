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

curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/     -X POST     -H "Content-Type: application/json"     --data '{"method": "getrawtransaction", "params": ["fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33", 1]}'
```
