// Filepath: onchain/scripts/manual-init-mxe.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair, SystemProgram, Transaction } = require("@solana/web3.js");
const fs = require("fs");
const os = require("os");
const {
  ARCIUM_IDL,
  getArciumProgram,
  getArciumProgAddress,
  getMXEAccAddress,
  getMempoolAccAddress,
  getExecutingPoolAccAddress,
  getClusterAccAddress,
  getCompDefAccAddress,
  getComputationAccAddress,
} = require("@arcium-hq/client");

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Workspace program (your MXE program ID)
  const program = anchor.workspace.AcesUnknown;

  const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

  console.log(`Using program ${program.programId.toBase58()}`);
  console.log(`Using wallet ${owner.publicKey.toBase58()}`);

  // Arcium program
  const arciumProgram = getArciumProgram(provider);
  console.log(`Arcium Program Address: ${arciumProgram.programId.toBase58()}`);

  // Accounts required by init_mxe
  const mxePda = getMXEAccAddress(program.programId);
  const mempoolPda = getMempoolAccAddress(program.programId);
  const execpoolPda = getExecutingPoolAccAddress(program.programId);

  // Cluster PDA derived from cluster_offset
  const clusterOffset = 1116522165; // same as used in deploy command
  const clusterPda = getClusterAccAddress(clusterOffset);

  // Per ARCIUM_IDL, mxe_keygen def/computation use offset 1
  const compDefPda = getCompDefAccAddress(program.programId, 1);
  const computationPda = getComputationAccAddress(program.programId, new anchor.BN(1));

  console.log("Derived addresses:");
  console.log("  MXE:", mxePda.toBase58());
  console.log("  Mempool:", mempoolPda.toBase58());
  console.log("  Execpool:", execpoolPda.toBase58());
  console.log("  Cluster:", clusterPda.toBase58());
  console.log("  CompDef(mxe_keygen):", compDefPda.toBase58());
  console.log("  Computation(mxe_keygen):", computationPda.toBase58());

  // Manually build instruction (compute Anchor discriminator and serialize args)
  const crypto = require("crypto");
  const disc = crypto.createHash("sha256").update("global:init_mxe").digest().subarray(0, 8);
  const clusterBuf = Buffer.alloc(4);
  clusterBuf.writeUInt32LE(clusterOffset, 0);
  // MempoolSize enum: Tiny=0, Small=1, Medium=2, Large=3
  const mempoolSize = 0; // Tiny
  const mempoolBuf = Buffer.from([mempoolSize]);
  const data = Buffer.concat([disc, clusterBuf, mempoolBuf]);
  const keys = [
    { pubkey: owner.publicKey, isSigner: true, isWritable: true }, // signer
    { pubkey: mxePda, isSigner: false, isWritable: true }, // mxe
    { pubkey: mempoolPda, isSigner: false, isWritable: true }, // mempool
    { pubkey: execpoolPda, isSigner: false, isWritable: true }, // execpool
    { pubkey: clusterPda, isSigner: false, isWritable: true }, // cluster
    { pubkey: compDefPda, isSigner: false, isWritable: true }, // mxe_keygen_computation_definition
    { pubkey: computationPda, isSigner: false, isWritable: true }, // mxe_keygen_computation
    owner.publicKey ? { pubkey: owner.publicKey, isSigner: false, isWritable: false } : undefined, // mxe_authority (optional)
    { pubkey: program.programId, isSigner: false, isWritable: false }, // mxe_program
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system_program
  ];
  const filteredKeys = keys.filter(Boolean);
  const ix = new anchor.web3.TransactionInstruction({ programId: arciumProgram.programId, keys: filteredKeys, data });
  const tx = new anchor.web3.Transaction().add(ix);
  const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();
  tx.recentBlockhash = blockhash;
  tx.lastValidBlockHeight = lastValidBlockHeight;
  tx.feePayer = owner.publicKey;
  tx.sign(owner);
  const sig = await provider.sendAndConfirm(tx, [owner], { commitment: "confirmed" });
  console.log("Init MXE TX Signature:", sig);
}

function readKpJson(path) {
  const file = fs.readFileSync(path);
  return Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});


