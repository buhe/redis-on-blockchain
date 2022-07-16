// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Import this file to use console.log
import "hardhat/console.sol";

contract Redis {

    mapping(address => mapping(string => string)) private str_kv;
    constructor() {
    }

    function set(string memory key, string memory value) public {
        str_kv[msg.sender][key] = value;
    }

    function get(string memory key) public view returns (string memory) {
        return str_kv[msg.sender][key];
    }

    function scan()  public view returns (string[] memory) {
        // return str_kv[msg.sender];
    } 
}
