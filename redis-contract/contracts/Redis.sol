
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Import this file to use console.log
import "hardhat/console.sol";

contract Redis {

    mapping(address => mapping(string => string)) private str_kv;
    mapping(address => mapping(string => mapping(string => string))) private map_kv;
    constructor() {
    }

    function set(string memory key, string memory value) public {
        str_kv[msg.sender][key] = value;
    }

    function get(string memory key) public view returns (string memory) {
        return str_kv[msg.sender][key];
    }

    /* function scan()  public view returns (string[] memory) { */
        // return str_kv[msg.sender];
    /* } */

    /* function keys() public view returns (string[] memory) { */
    /* 	//	return str_kv[msg.sender]; */
    /* } */

    function hset(string memory map_name, string memory key, string memory value) public {
        map_kv[msg.sender][map_name][key] = value;
    }

    function hget(string memory map_name, string memory key) public view returns (string memory) {
        return map_kv[msg.sender][map_name][key];
    }
}
