// Filepath: onchain/scripts/init-platform-config.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");

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

    // Initialize Platform Config
    const platformConfigPda = PublicKey.findProgramAddressSync(
        [Buffer.from("platform_config")],
        program.programId
    )[0];
    
    try {
        await program.account.platformConfig.fetch(platformConfigPda);
        console.log("Platform config already initialized.");
        return;
    } catch {
        console.log("Initializing platform config...");
    }

    // Create a simple treasury vault account (using the owner account for now)
    const treasuryVault = owner.publicKey;
    
    try {
        const tx = await program.methods
            .initializePlatformConfig()
            .accounts({
                platformConfig: platformConfigPda,
                admin: owner.publicKey,
                treasuryVault: treasuryVault,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([owner])
            .rpc();
        
        console.log("Platform config initialized successfully.");
        console.log(`Transaction signature: ${tx}`);
    } catch (error) {
        console.error("Failed to initialize platform config:", error);
        throw error;
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