//@ts-check

const path = require("path");

const rootDir = path.dirname(__dirname);
const binDir = path.join(rootDir, ".bin");

/**
 * @param {string} binary
 * @returns {string}
 */
function getProgram(binary) {
  return path.join(binDir, binary);
}

/** @type {import("@metaplex-foundation/amman").AmmanConfig} */
module.exports = {
  validator: {
    matchFeatures: "mainnet-beta",
    commitment: "processed",
    accountsCluster: "https://api.mainnet-beta.solana.com/",
    programs: [
      {
        label: "Parimutuel Resolver",
        programId: "RS1njPGQsykXyyPGUiAC9dvPyoqw73vtMFPJhipibj1",
        deployPath: getProgram("resolver.so"),
      },
    ],
    accounts: [
      {
        label: "System Extras",
        accountId: "SysExL2WDyJi9aRZrXorrjHJut3JwHQ7R9bTyctbNNG",
        executable: true,
      },
      {
        label: "Token Extras",
        accountId: "TokExjvjJmhKaRBShsBAsbSvEWMA1AgUNK7ps4SAc2p",
        executable: true,
      },
    ],
  },
};
