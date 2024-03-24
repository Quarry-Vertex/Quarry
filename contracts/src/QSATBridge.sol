// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "./Pool.sol";
import "./QSAT.sol";

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "forge-std/console.sol";

contract QSATBridge is Initializable, OwnableUpgradeable {
    address oracleAddress;
    address sharesPoolAddress;

    QSAT qsat;

    modifier onlyOracleOrSharesPool() {
        require(msg.sender == oracleAddress || msg.sender == sharesPoolAddress,
            "Only the oracleAddress or sharesPoolAddress can call this method");
        _;
    }

    event PegInQSATEvent(
        address ethAddress,
        uint256 amount
    );

    event PegOutQSATEvent(
        bytes32 btcAddress,
        uint256 amount
    );

    bytes32 [] public processedTransaction;

    struct PegOutRequest{
        bytes32 btcAddress;
        uint256 amount;
    }

    PegOutRequest[] public pegOutRequests;

    function setQSATContract(address _qsatAddress) public onlyOwner {
        qsat = QSAT(_qsatAddress);
    }

    function initialize(address _oracleAddress, address _sharesPoolAddress) public initializer {
        __Ownable_init(msg.sender);

        oracleAddress = _oracleAddress;
        sharesPoolAddress = _sharesPoolAddress;
    }

    // Oracle monitors peg in address, if there is a transction with 6+ confirmations with an OP_RETURN == eth transaction then call this method
    // SharesPool calls this method to transfer QSAT
    function pegInQSAT(address ethAddress, uint256 amount) public onlyOracleOrSharesPool {
        require(qsat.balanceOf(address(this)) >= amount,
            "Bridge contract has insufficient QSAT");

        qsat.transfer(ethAddress, amount);

        emit PegInQSATEvent(ethAddress, amount);
    }

    // Anyone can call this method to peg out (Note: MUST approve transfer of amount of tokens to the Bridge contract)
    // Oracle is responsible for subscribing to PegOutSAT events to send BTC to the recipient
    // change to push into an array
    // implement bridge this way
    // store each peg out event into an array
    function pegOutQSAT(bytes32 btcAddress, uint256 amount) public {
        require(qsat.balanceOf(msg.sender) >= amount,
            "Address has insufficient QSATs to peg out this amount");

        require(qsat.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed. Please ensure QSAT to be pegged out has been approved");

        // add the burn request to the Array
        pegOutRequests.push(PegOutRequest(btcAddress, amount));
        emit PegOutQSATEvent(btcAddress, amount);
    }

    // Function to get the total number of burn requests
    function getTotalPegOutRequests() external view returns (uint256) {
        return pegOutRequests.length;
    }

    // Function to get a specific burn request details by index
    function getPegOutRequest(uint256 _index) external view returns (PegOutRequest memory) {
        require(_index < pegOutRequests.length, "Burn request does not exist.");
        return pegOutRequests[_index];
    }
}
