pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract QuarryBTC is ERC20, Ownable {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) Ownable(msg.sender) {

    }

    function mintQuarryBTC(address account, uint256 amount) public onlyOwner {
        _mint(account, amount);
    }

    function burnQuarryBTC(uint256 amount) public onlyOwner {
        require(amount <= balanceOf(msg.sender), "Insufficient balance in address");
        _burn(msg.sender, amount);
    }
}
