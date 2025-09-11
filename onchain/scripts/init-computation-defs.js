// Filepath: onchain/scripts/init-computation-defs.js
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

    // Load the program with explicit devnet program ID
    const programId = new PublicKey("4ir9eYNjfVJggq19Su6DzAD4e24Yi4THesJjpBbAonVV");
    const idl = require("../target/idl/aces_unknown.json");

    // Create a minimal program client with just the init methods we need
    const program = new anchor.Program({
        ...idl,
        instructions: idl.instructions.filter(ix =>
            ix.name.includes('init') && ix.name.includes('CompDef')
        ),
        accounts: [] // Don't use account definitions to avoid size issues
    }, programId, provider);

    // Load owner keypair
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log(`Using program ${program.programId.toBase58()}`);
    console.log(`Using wallet ${owner.publicKey.toBase58()}`);

    // Initialize Arcium Computation Definitions
    const circuits = [
        { name: "shuffle_and_deal", initMethod: "init_shuffle_and_deal_comp_def" },
        { name: "reveal_community_cards", initMethod: "init_reveal_community_cards_comp_def" },
        { name: "evaluate_hands_and_payout", initMethod: "init_evaluate_hands_and_payout_comp_def" }
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
    const offsetBuffer = getCompDefAccOffset(circuit.name);

    const compDefPDA = PublicKey.findProgramAddressSync(
        [baseSeedCompDefAcc, program.programId.toBuffer(), offsetBuffer],
        getArciumProgAddress()
    )[0];

    try {
        // Try to fetch the account to see if it exists
        const accountInfo = await program.provider.connection.getAccountInfo(compDefPDA);
        if (accountInfo) {
            console.log(`CompDef for '${circuit.name}' already initialized.`);
            return;
        }
    } catch (e) {
        // Account doesn't exist, proceed
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
