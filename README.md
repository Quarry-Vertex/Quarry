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
