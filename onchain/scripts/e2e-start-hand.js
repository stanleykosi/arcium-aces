// Filepath: onchain/scripts/e2e-start-hand.js
const anchor = require("@coral-xyz/anchor");
const { PublicKey, Keypair, SystemProgram } = require("@solana/web3.js");
const { getMXEPublicKey, awaitComputationFinalization, getComputationAccAddress, getCompDefAccOffset, getMXEAccAddress, getMempoolAccAddress, getExecutingPoolAccAddress, getCompDefAccAddress, getClusterAccAddress, getClockAccAddress, getStakingPoolAccAddress, getArciumProgAddress } = require("@arcium-hq/client");
const { getOrCreateAssociatedTokenAccount, createMint, mintTo } = require("@solana/spl-token");
const os = require("os");
const fs = require("fs");

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AcesUnknown;
  const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

  console.log("Program:", program.programId.toBase58());
  console.log("Payer:", owner.publicKey.toBase58());

  // 0) Ensure Platform Config exists (skip if already created)
  const platformConfigPda = PublicKey.findProgramAddressSync([Buffer.from("platform_config")], program.programId)[0];
  const pcInfo = await getAccountInfoWithRetry(provider.connection, platformConfigPda, 5, 500);
  if (!pcInfo) {
    console.log("Initializing platform config...");
    await program.methods
      .initializePlatformConfig()
      .accounts({
        platformConfig: platformConfigPda,
        admin: owner.publicKey,
        treasuryVault: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc({ commitment: "confirmed" });
  } else {
    console.log("Platform config already exists. Skipping initialization.");
  }

  // 1) Setup SPL mint and token accounts for two players
  console.log("Creating SPL mint...");
  const mint = await createMint(provider.connection, owner, owner.publicKey, null, 6);
  const ownerAta = await getOrCreateAssociatedTokenAccount(provider.connection, owner, mint, owner.publicKey);

  // Second player keypair
  const player2 = Keypair.generate();
  console.log("Player2:", player2.publicKey.toBase58());
  await fundFromOwner(provider, owner, player2.publicKey, 300_000_000n); // 0.3 SOL from owner (avoid faucet limits)
  const player2Ata = await getOrCreateAssociatedTokenAccount(provider.connection, owner, mint, player2.publicKey);

  // Mint tokens to both accounts
  const mintAmount = 1_000_000_000n; // 1000 tokens with 6 decimals
  await mintTo(provider.connection, owner, mint, ownerAta.address, owner, Number(mintAmount));
  await mintTo(provider.connection, owner, mint, player2Ata.address, owner, Number(mintAmount));

  // 2) Create table
  const tableId = BigInt(Math.floor(Math.random() * 1_000_000));
  const tablePda = PublicKey.findProgramAddressSync([Buffer.from("table"), toU64LE(tableId)], program.programId)[0];
  const tableVaultPda = PublicKey.findProgramAddressSync([Buffer.from("vault"), tablePda.toBuffer()], program.programId)[0];
  const smallBlind = new anchor.BN(1000);
  const bigBlind = new anchor.BN(2000);
  const buyIn = new anchor.BN(20 * 2000); // 20 BB per rule

  console.log("Creating table...", tablePda.toBase58());
  await program.methods
    .createTable(new anchor.BN(tableId.toString()), smallBlind, bigBlind, buyIn)
    .accounts({
      table: tablePda,
      creator: owner.publicKey,
      platformConfig: platformConfigPda,
      tokenMint: mint,
      creatorTokenAccount: ownerAta.address,
      tableVault: tableVaultPda,
      systemProgram: SystemProgram.programId,
      tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([owner])
    .rpc({ commitment: "confirmed" });

  // 3) Join table with player2
  console.log("Player2 joining table...");
  await program.methods
    .joinTable(new anchor.BN(tableId.toString()), buyIn)
    .accounts({
      table: tablePda,
      player: player2.publicKey,
      playerTokenAccount: player2Ata.address,
      tableVault: tableVaultPda,
      tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .signers([player2])
    .rpc({ commitment: "confirmed" });

  // 4) Start hand (queue Arcium computation)
  // Hardcode/compute Arcium env (avoid getArciumEnv)
  const clusterOffset = 1116522165;
  const arciumProgramId = getArciumProgAddress();
  const clusterAccount = getClusterAccAddress(clusterOffset);
  const poolAccount = getStakingPoolAccAddress();
  const clockAccount = getClockAccAddress();
  const computationOffset = new anchor.BN(Buffer.from(cryptoRandomBytes(8)));
  const mxePub = await getMXEPublicKey(provider, program.programId);
  const arciumPubkey32 = Uint8Array.from(mxePub); // 32 bytes
  const handDataSeedCounter = (await program.account.table.fetch(tablePda)).handIdCounter; // u64

  console.log("Starting hand...");
  await program.methods
    .startHand(new anchor.BN(tableId.toString()), computationOffset, Array.from(arciumPubkey32))
    .accounts({
      table: tablePda,
      payer: owner.publicKey,
      handData: PublicKey.findProgramAddressSync([Buffer.from("hand"), tablePda.toBuffer(), toU64LE(BigInt(handDataSeedCounter))], program.programId)[0],
      mxeAccount: getMXEAccAddress(program.programId),
      mempoolAccount: getMempoolAccAddress(program.programId),
      executingPool: getExecutingPoolAccAddress(program.programId),
      computationAccount: getComputationAccAddress(program.programId, computationOffset),
      compDefAccount: getCompDefAccAddress(program.programId, Buffer.from(getCompDefAccOffset("shuffle_and_deal")).readUInt32LE()),
      clusterAccount,
      poolAccount,
      clockAccount,
      systemProgram: SystemProgram.programId,
      arciumProgram: arciumProgramId,
    })
    .signers([owner])
    .rpc({ commitment: "confirmed" });

  console.log("Awaiting computation finalization...");
  const finalizeSig = await awaitComputationFinalization(provider, computationOffset, program.programId, "confirmed");
  console.log("Computation finalized:", finalizeSig);

  // Fetch table to verify state transition
  const table = await program.account.table.fetch(tablePda);
  console.log("Game state:", table.gameState);
  console.log("Hand started. Dealer position:", table.dealerPosition);
}

function readKpJson(path) { const file = fs.readFileSync(path); return Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString()))); }
function toU64LE(n) { const b = Buffer.alloc(8); b.writeBigUInt64LE(BigInt(n)); return b; }
function cryptoRandomBytes(len) { return require("crypto").randomBytes(len); }
async function fundFromOwner(provider, owner, recipient, lamportsBig) {
  const lamports = Number(lamportsBig);
  const tx = new (require("@solana/web3.js").Transaction)().add(
    require("@solana/web3.js").SystemProgram.transfer({ fromPubkey: owner.publicKey, toPubkey: recipient, lamports })
  );
  const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();
  tx.recentBlockhash = blockhash;
  tx.lastValidBlockHeight = lastValidBlockHeight;
  tx.feePayer = owner.publicKey;
  tx.sign(owner);
  const sig = await provider.sendAndConfirm(tx, [owner], { commitment: "confirmed" });
  return sig;
}

async function getAccountInfoWithRetry(connection, pubkey, maxRetries = 5, delayMs = 500) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const info = await connection.getAccountInfo(pubkey);
      return info;
    } catch (e) {
      if (attempt === maxRetries) throw e;
      await new Promise((r) => setTimeout(r, delayMs));
    }
  }
}

main().catch((e) => { console.error(e); process.exit(1); });


