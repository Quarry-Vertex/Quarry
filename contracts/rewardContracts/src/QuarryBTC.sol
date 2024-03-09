// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/console.sol";

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Synthetic BTC on Quarry TODO: Figure out token metadata, URI
contract QuarryBTC is ERC20, Ownable {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) Ownable(msg.sender) public {
    }

    event MintedQuarryBTC(
        address indexed to,
        uint256 amount
    );

    event BurnedQuarryBTC(
        address indexed burner,
        uint256 amount
    );

    function mintQuarryBTC(address account, uint256 amount) public onlyOwner {
        _mint(account, amount);

        emit MintedQuarryBTC(account, amount);
    }

    function burnQuarryBTC(uint256 amount) public onlyOwner {
        require(amount <= balanceOf(msg.sender), "Insufficient balance in address");
        _burn(msg.sender, amount);

        emit BurnedQuarryBTC(msg.sender, amount);
    }

    function getBalanceOf(address account) public view returns (uint256) {
        return balanceOf(account);
    }
}
