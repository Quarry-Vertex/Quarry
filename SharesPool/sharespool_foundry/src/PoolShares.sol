pragma solidity ^0.8.13;

// import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
// import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {ERC721, ERC721Enumerable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {ERC721Burnable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract PoolShares is ERC721, ERC721Enumerable, ERC721Burnable, Ownable {
    string private _baseTokenURI;

    // Event for minting a new share
    event Minted(address indexed to, uint256 indexed tokenId);

    // Event for burning a share
    event Burned(address indexed burner, uint256 indexed tokenId);

    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
        _baseTokenURI = baseTokenURI;
    }

    function setBaseURI(string memory baseTokenURI) external onlyOwner {
        _baseTokenURI = baseTokenURI;
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    }

    function tokenURINew(uint256 tokenId) public view override returns (string memory) {
        require(_exists(tokenId), "Token does not exist");
        string memory baseURI = _baseURI();
        return bytes(baseURI).length > 0 ? string(abi.encodePacked(baseURI, tokenId.toString(), ".json")) : "";
    }

    function awardShare(address user, uint256 tokenId) public onlyOwner returns (uint256)
    {
        string tokenURI = tokenURINew(tokenId);

        _mint(user, tokenId);
        _setTokenURI(tokenId, tokenURI);

        emit Minted(msg.sender, tokenId);

        return tokenId;
    }

    function burnShare(uint256 tokenId) public onlyOwner {
        require(_exists(tokenId), "Token does not exist");

        // Transfer the token to the zero address, effectively burning it
        _burn(tokenId);

        // Emit the burn event
        emit Burned(msg.sender, tokenId);
    }

    function getOwnerOfShare(uint256 tokenId) external view returns (address) {
        require(_exists(tokenId), "Token does not exist");
        return ownerOf(tokenId);
    }
}