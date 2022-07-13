// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Import this file to use console.log
import "hardhat/console.sol";

contract Redis {

    mapping(address => mapping(string => string)) private str_kv;
    constructor() {
    }

    function set(address owner, string memory key, string memory value) public {
        require(owner != address(0), "redis: address zero is not a valid owner");
        str_kv[owner][key] = value;
    }

    function get(address owner, string memory key) public view returns (string memory) {
        return str_kv[owner][key];
    }
}
