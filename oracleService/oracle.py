import requests

# set the QuickNode endpoint
endpoint = 'https://yolo-holy-sheet.btc-testnet.quiknode.pro/920ff3a045338aaeebaec4a4ea5ee278ee4e737e/'

def get_best_hash():
    # get the best block hash
    response = requests.post(endpoint, json={"method": "getbestblockhash"}, headers={"Content-Type": "application/json"})
    response_data = response.json()
    tip_hash = response_data.get('result')
    print('Best Block Hash:', tip_hash)
    return tip_hash

def get_best_block():
    tip_hash = get_best_hash()
    # get the best block
    response = requests.post(endpoint, json={"method": "getblock", "params": [tip_hash]}, headers={"Content-Type": "application/json"})
    response_data = response.json()
    tip_block = response_data.get('result')
    return tip_block

def test():
    tip_block = get_best_block()
    print('Block Data:', tip_block)

test()

