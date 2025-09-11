// Filepath: onchain/scripts/init-computation-defs.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const {
    getArciumProgAddress,
    getArciumAccountBaseSeed,
    getCompDefAccOffset,
    getMXEAccAddress,
    initCompDef: initCompDefClient
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

    try {
        // Use the new v0.3.0 initCompDef function from the client library
        const tx = await initCompDefClient(program, owner, circuit.name, false);
        console.log(`Successfully initialized '${circuit.name}' computation definition.`);
        console.log(`Transaction signature: ${tx}`);
    } catch (error) {
        console.error(`Failed to initialize '${circuit.name}' computation definition:`, error);
        // Even if one fails, continue with the others
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