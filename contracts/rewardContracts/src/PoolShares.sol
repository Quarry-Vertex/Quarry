// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/console.sol";

import "@openzeppelin/contracts/utils/Strings.sol";

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

// A pool share corresponding to 5 exahashes of work
contract PoolShares is ERC721Upgradeable, OwnableUpgradeable {
    using Strings for uint256;

    string private _baseTokenURI;

    event MintedShare(
        address indexed to,
        uint256 indexed tokenId
    );

    event BurnedShare(
        address indexed burner,
        uint256 indexed tokenId
    );

    function initialize(string memory name, string memory symbol, string memory baseTokenURI) public initializer {
        __ERC721_init(name, symbol);
        __Ownable_init(msg.sender);

        _baseTokenURI = baseTokenURI;
    }

    function setBaseURI(string memory baseTokenURI) external onlyOwner {
        _baseTokenURI = baseTokenURI;
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    }

    // TODO: Figure out details of how we want to set up storing the metadata,
    // part of this is the images, etc. but not important now
    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        string memory baseURI = _baseURI();
        return bytes(baseURI).length > 0 ? string(abi.encodePacked(baseURI, tokenId.toString(), ".json")) : "";
    }

    function awardShare(address user, uint256 tokenId) public onlyOwner returns (uint256)
    {
        _safeMint(user, tokenId);

        emit MintedShare(user, tokenId);

        return tokenId;
    }

    function burnShare(uint256 tokenId) public onlyOwner {
        require(ownerOf(tokenId) != address(0), "Token does not exist");

        // Transfer the token to the zero address, effectively burning it
        _burn(tokenId);

        emit BurnedShare(msg.sender, tokenId);
    }

    function getOwnerOfShare(uint256 tokenId) external view returns (address) {
        return ownerOf(tokenId);
    }

    function tokenExists(uint256 tokenId) external view returns (bool) {
        return _ownerOf(tokenId) != address(0);
    }
}