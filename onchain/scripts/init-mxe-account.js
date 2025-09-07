// Filepath: onchain/scripts/init-mxe-account.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair, SystemProgram } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const { 
  getArciumProgAddress, 
  getMXEAccAddress,
  deriveMXEPda
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

    // Get the MXE account address
    const mxeAccountAddress = getMXEAccAddress(program.programId);
    console.log(`MXE Account Address: ${mxeAccountAddress.toBase58()}`);
    
    // Get the Arcium program address
    const arciumProgramAddress = getArciumProgAddress();
    console.log(`Arcium Program Address: ${arciumProgramAddress.toBase58()}`);
    
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
    
    // Try to manually initialize the MXE account
    try {
        console.log("Attempting to initialize MXE account...");
        
        // We need to call the Arcium program's init_mxe instruction
        // But we don't have direct access to it through the client library
        
        // Let's try to use the Arcium client to build the transaction
        console.log("MXE account initialization not yet implemented.");
        console.log("This requires calling the Arcium program's init_mxe instruction directly.");
    } catch (error) {
        console.error("Failed to initialize MXE account:", error);
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