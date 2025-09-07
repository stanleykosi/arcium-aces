// Filepath: onchain/scripts/debug-arcium-init.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const { 
  getArciumProgAddress, 
  getMXEAccAddress,
  buildInitMxeTx
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
    
    // Try to build the init MXE transaction manually
    try {
        console.log("Attempting to build init MXE transaction...");
        const initMxeTx = await buildInitMxeTx(
            provider,
            program.programId,
            owner.publicKey
        );
        
        console.log("Successfully built init MXE transaction");
        console.log("Transaction instructions:", initMxeTx.instructions.length);
        
        // Sign and send the transaction
        console.log("Signing and sending transaction...");
        const latestBlockhash = await provider.connection.getLatestBlockhash();
        initMxeTx.recentBlockhash = latestBlockhash.blockhash;
        initMxeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
        initMxeTx.sign(owner);
        
        const signature = await provider.sendAndConfirm(initMxeTx, [owner], {
            commitment: "confirmed"
        });
        
        console.log("Successfully initialized MXE account");
        console.log("Transaction signature:", signature);
    } catch (error) {
        console.error("Failed to initialize MXE account:", error);
        
        // Let's check what accounts are involved in the error
        if (error.logs) {
            console.log("Error logs:");
            error.logs.forEach(log => console.log("  ", log));
        }
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