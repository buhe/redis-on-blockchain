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
    string private greeting;

    function greet() public view returns (string memory) {
        return greeting;
    }

    function setGreeting(string memory _greeting) public {
        console.log("Changing greeting from '%s' to '%s'", greeting, _greeting);
        greeting = _greeting;
    }
}
