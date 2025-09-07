// Filepath: onchain/scripts/verify-mxe-address.js
const anchor = require("@coral-xyz/anchor");
const { getMXEAccAddress, deriveMXEPda } = require("@arcium-hq/client");

async function main() {
    // Set up provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    // Load the program
    const program = anchor.workspace.AcesUnknown;
    
    // Get the MXE account address using the client function
    const mxeAccountAddress = getMXEAccAddress(program.programId);
    
    console.log(`Program ID: ${program.programId.toBase58()}`);
    console.log(`MXE Account Address (getMXEAccAddress): ${mxeAccountAddress.toBase58()}`);
    
    // Try to derive the MXE account address manually
    try {
        // This might not work if deriveMXEPda is not exported
        // const derivedAddress = deriveMXEPda(program.programId);
        // console.log(`MXE Account Address (derived): ${derivedAddress.toBase58()}`);
    } catch (error) {
        console.log("Could not derive MXE account address manually:", error.message);
    }
    
    // Try to fetch the MXE account
    try {
        const accountInfo = await provider.connection.getAccountInfo(mxeAccountAddress);
        if (accountInfo) {
            console.log(`MXE Account exists:`);
            console.log(`  Balance: ${accountInfo.lamports} lamports`);
            console.log(`  Owner: ${accountInfo.owner.toBase58()}`);
            console.log(`  Executable: ${accountInfo.executable}`);
            console.log(`  Data length: ${accountInfo.data.length} bytes`);
        } else {
            console.log("MXE Account does not exist yet");
        }
    } catch (error) {
        console.log("Error checking MXE account:", error.message);
    }
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});