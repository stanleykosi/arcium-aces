const anchor = require("@coral-xyz/anchor");
const { PublicKey } = require("@solana/web3.js");

async function main() {
    // Program ID
    const programId = new PublicKey("6oH465yHSL7FXd4a76nFkRRPeNgVhJa5BY6E8Tt2121w");
    
    // Find the platform config PDA
    const [platformConfigPda, bump] = PublicKey.findProgramAddressSync(
        [Buffer.from("platform_config")],
        programId
    );
    
    console.log("Platform Config PDA:", platformConfigPda.toBase58());
    console.log("Bump:", bump);
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});