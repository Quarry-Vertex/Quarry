pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";

contract QuarryBTC is ERC20Upgradeable, OwnableUpgradeable {
    function initialize(string memory name, string memory symbol) public initializer {
        __ERC20_init(name, symbol);
        __Ownable_init(msg.sender);
    }

    function mintQuarryBTC(address account, uint256 amount) public onlyOwner {
        _mint(account, amount);
    }

    function burnQuarryBTC(uint256 amount) public onlyOwner {
        require(amount <= balanceOf(msg.sender), "Insufficient balance in address");
        _burn(msg.sender, amount);
    }
}
