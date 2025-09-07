// Filepath: onchain/scripts/call-arcium-init-mxe.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair, SystemProgram } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const { 
  getArciumProgAddress, 
  getMXEAccAddress
} = require("@arcium-hq/client");

async function main() {
    // Set up provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    // Load the program
    const program = anchor.workspace.AcesUnknown;
    
    // Load owner keypair
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log(`Using program ${program.programId.toBase58()}`);
    console.log(`Using wallet ${owner.publicKey.toBase58()}`);

    // Get the Arcium program
    const arciumProgramAddress = getArciumProgAddress();
    console.log(`Arcium Program Address: ${arciumProgramAddress.toBase58()}`);
    
    // Get the MXE account address
    const mxeAccountAddress = getMXEAccAddress(program.programId);
    console.log(`MXE Account Address: ${mxeAccountAddress.toBase58()}`);
    
    // Try to fetch the MXE account
    try {
        const accountInfo = await provider.connection.getAccountInfo(mxeAccountAddress);
        if (accountInfo) {
            console.log("MXE Account already exists.");
            return;
        }
    } catch (error) {
        console.log("Error checking MXE account:", error.message);
    }
    
    // Try to call the Arcium program's init_mxe instruction
    try {
        console.log("Attempting to call Arcium program's init_mxe instruction...");
        
        // This is a simplified version - we would need to properly construct the instruction
        console.log("Manual call to Arcium program's init_mxe instruction not yet implemented.");
        console.log("This requires constructing the instruction with the correct accounts and data.");
    } catch (error) {
        console.error("Failed to call Arcium program's init_mxe instruction:", error);
    }
}

function readKpJson(path) {
    const file = fs.readFileSync(path);
    return Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});