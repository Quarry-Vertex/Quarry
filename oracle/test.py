from web3 import Web3, Account
import json

# Connect to the Ethereum node
w3 = Web3(Web3.HTTPProvider("http://localhost:8545"))

# Load the contract ABI
contract_abi_path = "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json"
with open(contract_abi_path) as f:
    contract_abi = json.load(f)

# Set the contract address (make sure it's a valid checksum address)
contract_address = Web3.to_checksum_address(
    "0xbdEd0D2bf404bdcBa897a74E6657f1f12e5C6fb6"
)
# Create the contract instance
contract = w3.eth.contract(address=contract_address, abi=contract_abi)

# get oracle information
oracle_private_key = (
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
)
oracle_account = Account.from_key(oracle_private_key)

# Get the chain ID
chain_id = w3.eth.chain_id
print("Chain ID:", chain_id)

# Send a transaction to call the getOneHundred function
nonce = w3.eth.get_transaction_count(oracle_account.address)
value = contract.functions.getOneHundred().call()
print("Transaction receipt:", value)
# txn = contract.functions.getOneHundred().build_transaction(
    # {
        # "chainId": chain_id,
        # "gas": 5000000,
        # "gasPrice": w3.to_wei("50", "gwei"),
        # "nonce": nonce,
    # }
# )

# sign transaction
# signed_txn = w3.eth.account.sign_transaction(txn, private_key=oracle_private_key)
# send transaction
# txn_hash = w3.eth.send_raw_transaction(signed_txn.rawTransaction)
# recieve outputs
# receipt = w3.eth.wait_for_transaction_receipt(txn_hash)
# print("Transaction receipt:", receipt)
