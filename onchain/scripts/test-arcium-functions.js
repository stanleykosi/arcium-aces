// Filepath: onchain/scripts/test-arcium-functions.js
const anchor = require("@coral-xyz/anchor");
const { 
  getArciumProgAddress, 
  getArciumAccountBaseSeed, 
  getCompDefAccOffset,
  getMXEAccAddress
} = require("@arcium-hq/client");

async function main() {
    // Check what's available in the client package
    const client = require("@arcium-hq/client");
    console.log("Client package keys:", Object.keys(client));
    
    // Try to find init_comp_def or similar function
    for (const key of Object.keys(client)) {
        if (key.toLowerCase().includes("init") && key.toLowerCase().includes("comp")) {
            console.log("Found potential init function:", key);
        }
    }
    
    // Also check the arcium program ID
    console.log("Arcium program ID:", getArciumProgAddress().toBase58());
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});