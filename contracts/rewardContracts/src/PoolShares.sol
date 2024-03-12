// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/console.sol";

import "@openzeppelin/contracts/utils/Strings.sol";

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

// A pool share corresponding to 5 exahashes of work
// TODO: Figure out token metadata, URI
contract PoolShares is ERC721Upgradeable, OwnableUpgradeable {
    using Strings for uint256;

    event MintedShare(
        address indexed to,
        uint256 indexed tokenId
    );

    event BurnedShare(
        address indexed burner,
        uint256 indexed tokenId
    );

    function initialize(string memory name, string memory symbol, address sharesPoolAddress) public initializer {
        __ERC721_init(name, symbol);
        __Ownable_init(sharesPoolAddress);
    }

    function awardShare(address user, uint256 tokenId) public onlyOwner returns (uint256)
    {
        _safeMint(user, tokenId);

        emit MintedShare(user, tokenId);

        return tokenId;
    }

    function burnShare(uint256 tokenId) public onlyOwner {
        require(tokenExists(tokenId), "Token does not exist");

        // Transfer the token to the zero address, effectively burning it
        _burn(tokenId);

        emit BurnedShare(msg.sender, tokenId);
    }

    function getOwnerOfShare(uint256 tokenId) public view returns (address) {
        return ownerOf(tokenId);
    }

    function tokenExists(uint256 tokenId) public view returns (bool) {
        return _ownerOf(tokenId) != address(0);
    }
}
