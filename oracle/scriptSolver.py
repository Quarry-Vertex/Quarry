from bitcoin import SelectParams
from bitcoin.core import x
from bitcoin.core.script import CScript
from bitcoin.wallet import P2WPKHBitcoinAddress

SelectParams('mainnet')
# Create an address from that private key.
scriptPubKey = CScript(x('001435f6de260c9f3bdee47524c473a6016c0c055cb9'))
address = P2WPKHBitcoinAddress.from_scriptPubKey(scriptPubKey)
print(address)
