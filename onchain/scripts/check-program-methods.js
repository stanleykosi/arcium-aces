// Filepath: onchain/scripts/check-program-methods.js
const anchor = require("@coral-xyz/anchor");

async function main() {
    // Set up provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    // Load the program
    const program = anchor.workspace.AcesUnknown;
    
    console.log("Available program methods:");
    console.log(Object.keys(program.methods));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});