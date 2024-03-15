// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/console.sol";

import "@openzeppelin/contracts/utils/Strings.sol";

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

// A pool share corresponding to 5 exahashes of work
// TODO: Figure out token metadata, URI
contract Share is ERC721Upgradeable, OwnableUpgradeable {
    using Strings for uint256;

    event Mint(
        address indexed to,
        uint256 indexed tokenId
    );

    event Burn(
        address indexed burner,
        uint256 indexed tokenId
    );

    function initialize(string memory name, string memory symbol, address poolAddress) public initializer {
        __ERC721_init(name, symbol);
        __Ownable_init(poolAddress);
    }

    function mint(address user, uint256 tokenId) public onlyOwner returns (uint256)
    {
        _safeMint(user, tokenId);

        emit Mint(user, tokenId);

        return tokenId;
    }

    function burn(uint256 tokenId) public onlyOwner {
        require(tokenExists(tokenId), "Token does not exist");

        // Transfer the token to the zero address, effectively burning it
        _burn(tokenId);

        emit Burn(msg.sender, tokenId);
    }

    function tokenExists(uint256 tokenId) public view returns (bool) {
        return _ownerOf(tokenId) != address(0);
    }
}
