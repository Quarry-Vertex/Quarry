echo "installing base forge libs"
forge install https://github.com/foundry-rs/forge-std --no-git

echo "installing openzepplin deps"
forge install https://github.com/OpenZeppelin/openzeppelin-foundry-upgrades --no-git
forge install https://github.com/OpenZeppelin/openzeppelin-contracts-upgradeable --no-git
forge install https://github.com/OpenZeppelin/openzeppelin-contracts --no-git

echo "setup complete"
