pragma solidity ^0.8.13;

import "@openzeppelin/contracts-upgradable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/IERC721Upgradeable.sol";

contract PoolShares is ERC721Upgradeable, OwnableUpgradeable {
    string private _baseTokenURI;

    // Event for minting a new share
    event Minted(address indexed to, uint256 indexed tokenId);

    // Event for burning a share
    event Burned(address indexed burner, uint256 indexed tokenId);

    function initialize(string memory name, string memory symbol, string memory baseTokenURI) public initializer {
        __ERC20_init(name, symbol);
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
        _mint(user, tokenId);

        emit Minted(msg.sender, tokenId);

        return tokenId;
    }

    function burnShare(uint256 tokenId) public onlyOwner {
        // Transfer the token to the zero address, effectively burning it
        _burn(tokenId);

        // Emit the burn event
        emit Burned(msg.sender, tokenId);
    }

    function getOwnerOfShare(uint256 tokenId) external view returns (address) {
        return ownerOf(tokenId);
    }
}