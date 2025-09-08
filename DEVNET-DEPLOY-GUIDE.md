# Arcium Devnet Deploy + MXE Init + Comp-Defs + E2E (Step-by-step)

This guide shows exactly what we did to:
- Deploy your Solana program to devnet
- Initialize the Arcium MXE account (manually)
- Initialize computation definitions
- Run a simple end-to-end test

It also explains why the automatic MXE initialization failed and how the manual approach fixed it.

---

## Prerequisites
- Solana CLI configured, wallet at `~/.config/solana/id.json` with devnet SOL
- Node 18+, pnpm installed
- A reliable RPC URL for devnet (we used Helius)

Set these two env vars when needed:
```bash
export ANCHOR_PROVIDER_URL="https://devnet.helius-rpc.com/?api-key=<your-helius-api-key>"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"
```

We use a fixed Arcium cluster offset in this project:
```bash
# Cluster offset used across the repo/scripts
1116522165
```

---

## 1) Deploy the Solana program to devnet

We used the Arcium CLI to deploy the program itself (this does NOT need to initialize MXE every time):
```bash
cd onchain
yes | arcium deploy \
  --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=<your-helius-api-key>" 
```
If you’ve already deployed once and only want to redeploy the program (not MXE), use:
```bash
yes | arcium deploy \
  --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=<your-helius-api-key>" \
  --skip-init
```
This prevents the CLI from re-initializing MXE (which caused errors in our case).

---

## 2) Why `arcium deploy` failed to initialize MXE automatically

During the “InitMxe” step, the CLI hit an Anchor `ConstraintExecutable (2007)` error on `mxe_program`. That means a non-executable account was passed where the Arcium program account should be. In short, the automatic account mapping did not line up for our environment at that time.

We solved this by manually constructing and sending the `init_mxe` instruction with the exact accounts and argument encoding required by Arcium, ensuring:
- The correct `mxe_program` (Arcium program) was provided
- The correct MXE/Mempool/Execpool/Cluster PDAs were derived
- The `mxe_keygen` computation definition and computation PDAs were included
- The enum argument (mempool size) was serialized correctly

After that, MXE was initialized and healthy.

---

## 3) Manual MXE initialization (the working approach)

We added a script `onchain/scripts/manual-init-mxe.js` that:
- Derives all required PDAs
- Builds the raw Anchor discriminator + data for `init_mxe`
- Supplies the correct ordered accounts (including optional authority)
- Submits the transaction

Run it:
```bash
cd onchain
pnpm node scripts/manual-init-mxe.js | cat
```
Expected output ends with a transaction signature and, after verification, an MXE PDA owned by the Arcium program (Executable: false is correct for PDAs).

Verify the MXE account:
```bash
pnpm node scripts/verify-mxe-address.js | cat
```
You should see something like:
- Owner: `BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6` (Arcium program)
- Executable: false (expected for data PDAs)

### How this manual script gets its variables (so you can reuse it anywhere)

Adapting the script for a new program/codebase requires just a few inputs; everything else is derived:

- Program ID (your MXE-enabled program):
  - The script loads `anchor.workspace.YourProgramName` which is set by Anchor using your IDL and `Anchor.toml`.
  - If you’re not using Anchor workspaces, pass an explicit `programId` PublicKey and construct a Program with your IDL.

- Wallet (payer):
  - We read `~/.config/solana/id.json` and set `ANCHOR_WALLET` so Anchor can sign.

- Cluster offset (which Arcium cluster to use):
  - We used `1116522165` in this repo. For your project, use the offset you deployed your cluster with.

- Arcium program ID and all Arcium PDAs (derived, no magic constants):
  - `getArciumProgAddress()` ⇒ the Arcium program Pubkey (used as `mxe_program`).
  - `getMXEAccAddress(program.programId)` ⇒ MXE PDA tied to your program id.
  - `getMempoolAccAddress(program.programId)` ⇒ Mempool PDA.
  - `getExecutingPoolAccAddress(program.programId)` ⇒ Execpool PDA.
  - `getClusterAccAddress(clusterOffset)` ⇒ Cluster PDA from your cluster offset.
  - `getCompDefAccAddress(program.programId, 1)` and `getComputationAccAddress(program.programId, new BN(1))` ⇒ mxe_keygen definition/computation at offset 1 (as specified by Arcium IDL).

- Instruction data (args) encoding:
  - The script computes the Anchor discriminator for `init_mxe` (`sha256('global:init_mxe')[0..8]`).
  - Appends `cluster_offset` as u32 LE.
  - Appends `mempool_size` as an enum byte (0=Tiny, 1=Small, 2=Medium, 3=Large). We used `Tiny`.

- Account ordering (exact order per Arcium IDL):
  1) signer (payer, writable, signer)
  2) mxe (writable)
  3) mempool (writable)
  4) execpool (writable)
  5) cluster (writable)
  6) mxe_keygen_computation_definition (writable)
  7) mxe_keygen_computation (writable)
  8) mxe_authority (optional; we pass owner pubkey; readonly)
  9) mxe_program (readonly; Arcium program id)
  10) system_program (readonly)

With these pieces, the script is portable: swap your program name/ID, cluster offset, and it will derive the rest.

---

## 4) Initialize computation definitions (comp-defs)

Use either script below; both align with the docs. We used `init-computation-defs.js` successfully on devnet:
```bash
export ANCHOR_PROVIDER_URL="https://devnet.helius-rpc.com/?api-key=<your-helius-api-key>"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"
cd onchain
pnpm node scripts/init-computation-defs.js | cat
```

If it complains about IDL, run `anchor build` and try again:
```bash
anchor build && pnpm node scripts/init-computation-defs.js | cat
```

This initializes comp-defs for:
- `shuffle_and_deal`
- `reveal_community_cards`
- `evaluate_hands_and_payout`

You should see a success message and signatures for each.

---

## 5) End-to-end test (create table, join, start hand)

We added `onchain/scripts/e2e-start-hand.js` that:
1. Skips platform-config init if already present
2. Creates an SPL token mint
3. Funds a second player from your main wallet (avoid faucet rate-limits)
4. Creates a `Table` PDA and seats creator with buy-in
5. Has player2 join the table with buy-in
6. Starts a hand by queueing the `shuffle_and_deal` computation and waits for finalization

Run it:
```bash
cd onchain
ANCHOR_PROVIDER_URL="https://devnet.helius-rpc.com/?api-key=<your-helius-api-key>" \
ANCHOR_WALLET="$HOME/.config/solana/id.json" \
pnpm node scripts/e2e-start-hand.js | cat
```
Expected output shows PDAs, successful join, then “Starting hand…”, followed by a finalization signature or a clear on-chain program log if something fails. If you see an on-chain program error (e.g., stack-frame violation), that’s a runtime issue in the program’s instruction (not MXE/comp-defs) and you can add `msg!` breadcrumbs to isolate.

Notes:
- If you hit “devnet faucet rate limit,” the script already funds the second player from your wallet using a SystemProgram transfer (no faucet).
- We removed dependency on `getArciumEnv` by directly computing Arcium accounts from helpers and the known cluster offset.

---

## TL;DR: Newbie checklist (like explaining to a 5-year-old)

1) Put on your gloves (set the wallet and RPC):
```bash
export ANCHOR_PROVIDER_URL="https://devnet.helius-rpc.com/?api-key=<your-key>"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"
```

2) Build your sandcastle (deploy the program):
```bash
cd onchain
yes | arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=<your-key>"
```
If you already built it once and just want to rebuild: add `--skip-init`.

3) Ask grown-ups to bring toys (initialize MXE) — do it manually once:
```bash
pnpm node scripts/manual-init-mxe.js | cat
pnpm node scripts/verify-mxe-address.js | cat  # should exist, owned by Arcium
```

4) Learn the game rules (initialize comp-defs) — also once:
```bash
pnpm node scripts/init-computation-defs.js | cat
```

5) Play a round (end-to-end test):
```bash
pnpm node scripts/e2e-start-hand.js | cat
```
If it says “program error,” that’s the game’s rules code (your on-chain program) needing a tweak — MXE and comp-defs are already fine.

---

## Troubleshooting
- ConstraintExecutable (2007) during InitMxe: automatic MXE init passed a non-executable account. Use the manual script which sets correct accounts/args.
- Faucet rate limit: the e2e script funds the second player from your own wallet (no faucet).
- `getArciumEnv` env-var parse errors: we removed the dependency and directly derived all required Arcium accounts.
- On-chain “Access violation” when starting a hand: add `msg!` breadcrumbs in `start_hand` around blinds/queue to pinpoint and fix the instruction logic or stack usage.

---

## That’s it
- Program deployed ✔
- MXE initialized ✔
- Comp-defs initialized ✔
- E2E test in place ✔

If redeploying later, remember to use `--skip-init` to avoid re-initializing MXE.
