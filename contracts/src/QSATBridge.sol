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

    event PegInQSAT(
        address ethAddress,
        uint256 amount
    );

    event PegOutQSAT(
        bytes32 btcAddress,
        uint256 amount
    );

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

        emit PegInQSAT(ethAddress, amount);
    }

    // Anyone can call this method to peg out (Note: MUST approve transfer of amount of tokens to the Bridge contract)
    // Oracle is responsible for subscribing to PegOutSAT events to send BTC to the recipient
    function pegOutQSAT(bytes32 btcAddress, uint256 amount) public {
        require(qsat.balanceOf(msg.sender) >= amount,
            "Address has insufficient QSATs to peg out this amount");

        require(qsat.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed. Please ensure QSAT to be pegged out has been approved");

        emit PegOutQSAT(btcAddress, amount);
    }
}
