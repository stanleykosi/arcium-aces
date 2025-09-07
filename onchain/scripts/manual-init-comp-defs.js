// Filepath: onchain/scripts/manual-init-comp-defs.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const { 
  getArciumProgAddress, 
  getArciumAccountBaseSeed, 
  getCompDefAccOffset,
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

    // Initialize Arcium Computation Definitions
    const circuits = [
        { name: "shuffle_and_deal", initMethod: "initShuffleAndDealCompDef" },
        { name: "reveal_community_cards", initMethod: "initRevealCommunityCardsCompDef" },
        { name: "evaluate_hands_and_payout", initMethod: "initEvaluateHandsAndPayoutCompDef" }
    ];
    
    for (const circuit of circuits) {
        await initCompDef(circuit, program, owner);
    }

    console.log("✅ Computation definitions initialization complete.");
}

// Helper function to initialize computation definitions
async function initCompDef(circuit, program, owner) {
    console.log(`Initializing CompDef for '${circuit.name}'...`);
    
    // Get the computation definition account PDA
    const baseSeedCompDefAcc = getArciumAccountBaseSeed("ComputationDefinitionAccount");
    const offset = getCompDefAccOffset(circuit.name);
    
    const compDefPDA = PublicKey.findProgramAddressSync(
        [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
        getArciumProgAddress()
    )[0];

    try {
        await program.account.computationDefinitionAccount.fetch(compDefPDA);
        console.log(`CompDef for '${circuit.name}' already initialized.`);
        return;
    } catch {
        // Not initialized, proceed
    }

    // Get the MXE account
    const mxeAccountPda = getMXEAccAddress(program.programId);
    
    try {
        // Initialize the computation definition
        console.log(`Calling initialization method '${circuit.initMethod}' for '${circuit.name}'...`);
        
        const tx = await program.methods[circuit.initMethod]()
            .accounts({
                payer: owner.publicKey,
                mxeAccount: mxeAccountPda,
                compDefAccount: compDefPDA,
                arciumProgram: getArciumProgAddress(),
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([owner])
            .rpc();
        
        console.log(`Successfully initialized '${circuit.name}' computation definition.`);
        console.log(`Transaction signature: ${tx}`);
    } catch (error) {
        console.error(`Failed to initialize '${circuit.name}' computation definition:`, error);
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