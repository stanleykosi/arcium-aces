// Filepath: onchain/scripts/initialize-devnet.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AcesUnknown } from "../target/types/aces_unknown";
import { 
  getArciumProgAddress, 
  getArciumAccountBaseSeed, 
  getCompDefAccOffset, 
  buildFinalizeCompDefTx, 
  getMXEAccAddress 
} from "@arcium-hq/client";
import { PublicKey, Keypair } from "@solana/web3.js";
import * as os from "os";
import * as fs from "fs";

async function main() {
    // Set up provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    // Load the program
    const program = anchor.workspace.AcesUnknown as Program<AcesUnknown>;
    
    // Load owner keypair
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log(`Using program ${program.programId.toBase58()}`);
    console.log(`Using wallet ${owner.publicKey.toBase58()}`);

    // 1. Initialize Platform Config
    const platformConfigPda = PublicKey.findProgramAddressSync(
        [Buffer.from("platform_config")],
        program.programId
    )[0];
    
    try {
        await program.account.platformConfig.fetch(platformConfigPda);
        console.log("Platform config already initialized.");
    } catch {
        console.log("Initializing platform config...");
        await program.methods
            .initializePlatformConfig()
            .accounts({
                platformConfig: platformConfigPda,
                admin: owner.publicKey,
                treasuryVault: owner.publicKey, // Using owner as treasury for now
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .signers([owner])
            .rpc();
        console.log("Platform config initialized successfully.");
    }

    // 2. Initialize Arcium Computation Definitions
    const circuits = ["shuffle_and_deal", "reveal_community_cards", "evaluate_hands_and_payout"];
    for (const circuit of circuits) {
        await initCompDef(circuit, program, owner);
    }

    console.log("✅ Devnet initialization complete.");
}

// Helper function to initialize computation definitions
async function initCompDef(circuitName: string, program: Program<AcesUnknown>, owner: Keypair) {
    console.log(`Initializing CompDef for '${circuitName}'...`);
    
    // Get the computation definition account PDA
    const baseSeedCompDefAcc = getArciumAccountBaseSeed("ComputationDefinitionAccount");
    const offsetBuffer = getCompDefAccOffset(circuitName);
    const offset = Buffer.from(offsetBuffer).readUInt32LE();
    
    const compDefPDA = PublicKey.findProgramAddressSync(
        [baseSeedCompDefAcc, program.programId.toBuffer(), offsetBuffer],
        getArciumProgAddress()
    )[0];

    try {
        await program.account.computationDefinition.fetch(compDefPDA);
        console.log(`CompDef for '${circuitName}' already initialized.`);
        return;
    } catch {
        // Not initialized, proceed
    }

    // Get the MXE account
    const mxeAccountPda = getMXEAccAddress(program.programId);
    
    try {
        // Initialize the computation definition
        const methodName = `init${toPascalCase(circuitName)}CompDef`;
        console.log(`Calling method: ${methodName}`);
        
        await program.methods[methodName]()
            .accounts({
                payer: owner.publicKey,
                mxeAccount: mxeAccountPda,
                compDefAccount: compDefPDA,
                arciumProgram: getArciumProgAddress(),
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .signers([owner])
            .rpc();
        
        console.log(`Successfully initialized '${circuitName}' computation definition.`);
    } catch (error) {
        console.error(`Failed to initialize '${circuitName}' computation definition:`, error);
        throw error;
    }
}

function readKpJson(path: string): anchor.web3.Keypair {
    const file = fs.readFileSync(path);
    return Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())));
}

function toPascalCase(str: string): string {
    return str.replace(/_(\w)/g, (_, c) => c.toUpperCase()).replace(/^\w/, c => c.toUpperCase());
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});