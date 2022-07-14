import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "hardhat-abi-exporter";

const ALCHEMY_API_KEY = "bjJEnGWhmiAGmUm7fMp9BE9oyJq4OSMO";

const ROPSTEN_PRIVATE_KEY = "1a4240d88f6f90f5ca8f73345a825b09154f2ddcdf4269a703350e507454f7ac";
const config: HardhatUserConfig = {
  solidity: "0.8.9",
  networks: {
    ropsten: {
      url: `https://eth-ropsten.alchemyapi.io/v2/${ALCHEMY_API_KEY}`,
      accounts: [`0x${ROPSTEN_PRIVATE_KEY}`]
    }
  },
  abiExporter: [
    {
      path: '../redis-bridge/src/abi',
      runOnCompile: true,
    }
  ]
};

export default config;
