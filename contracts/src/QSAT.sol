// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/console.sol";

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";

// Synthetic satoshis on Quarry TODO: Figure out token metadata, URI
contract QSAT is ERC20Upgradeable {
    uint256 constant public TOTAL_SUPPLY = 21000000 * 100000000; // 21M BTC supply * 100M SATS per BTC

    function initialize(string memory name, string memory symbol, address bridgeAddress) public initializer {
        __ERC20_init(name, symbol);

        _mint(bridgeAddress, TOTAL_SUPPLY);
    }
}
