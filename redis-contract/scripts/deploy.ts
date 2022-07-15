import { ethers } from "hardhat";
import fs from "fs";
const fsPromises = fs.promises;
function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
  // const currentTimestampInSeconds = Math.round(Date.now() / 1000);
  // const ONE_YEAR_IN_SECS = 365 * 24 * 60 * 60;
  // const unlockTime = currentTimestampInSeconds + ONE_YEAR_IN_SECS;

  // const lockedAmount = ethers.utils.parseEther("1");

  // const Lock = await ethers.getContractFactory("Lock");
  // const lock = await Lock.deploy(unlockTime, { value: lockedAmount });

  // await lock.deployed();

  // console.log("Lock with 1 ETH deployed to:", lock.address);

  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);

  console.log("Account balance:", (await deployer.getBalance()).toString());

  const Redis = await ethers.getContractFactory("Redis");
  const redis = await Redis.deploy();

  await redis.deployed();

  console.log("Redis deployed to:", redis.address);
  await writeRust(redis.address, ['../redis-bridge/src/address.rs']);

  await redis.set("key", "rob");
  // wait block created
  await delay(120 * 1000);

  const val = await redis.get("key");

  console.log("value is " + val);

  await redis.setGreeting('hi bugu');

  await delay(120 * 1000);

  console.log("greet:", await redis.greet());
}

async function writeRust(address: string, files: string[]) {
  console.log('write ' + address + " to rust " + files);
  for (const file of files) {
    await fsPromises.writeFile(file, 'pub const ADDRESS: &str = "' + address + '";\n');
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
