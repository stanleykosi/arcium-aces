# Intro to Arcium

Arcium is a decentralized private computation network that enables secure processing of encrypted data through Multi-Party Computation (MPC). It solves a fundamental problem in Web3: how to process sensitive data while maintaining privacy. Traditionally, computation requires data to be decrypted, making it vulnerable to attacks and exposing private information. Arcium changes this by allowing computations to run on fully encrypted data.

## What Arcium Enables

As a Solana developer, Arcium gives you the ability to:

1. **Build Privacy-Preserving Applications**: Add privacy to your applications without adopting a new blockchain, programming language, or workflow. Arcium maintains full composability within familiar ecosystems.
2. **Use Familiar Tooling**: Leverage the Arcis framework, which extends Solana's Anchor tooling. Built in Rust, it allows you to add privacy simply by marking functions as confidential—no cryptography knowledge required.
3. **Process Sensitive Data**: Run computations on encrypted data without ever decrypting it. This means sensitive information like user balances, trade orders, or personal data can be processed securely.

## How It Works

Your application (MXE) works with encrypted data in three simple steps:

1. Client encrypts data and sends it to your MXE program
2. Your program submits the computation to Arcium's network of MPC nodes
3. Nodes process the data while keeping it encrypted, then return the results

The entire process happens on-chain through Solana, with each step verified and coordinated by Arcium's programs. For larger computations, an optional callback server handles results that don't fit in a single transaction.

## Common Use Cases

1. **Private DeFi**: Build dark pools, aka private order books, where trade sizes and prices remain hidden, enabling truly permissionless confidential trading without front-running or market manipulation.
2. **Secure AI**: Enable AI model inference and training on sensitive data while keeping the data encrypted.
3. **Confidential Gaming**: Build hidden information games where player moves and state remain private until revealed (e.g., card games, strategy games, auctions).

## Getting Started

Arcium provides a familiar development experience for Solana developers:

* Use the `arcium` CLI (a wrapper over `anchor` CLI) to build Solana programs with Arcium
* Write confidential instructions in Rust using the Arcis framework
* Integrate with your Solana programs using the TypeScript client library

Follow these steps to get started:

1. [Install Arcium](installation) - Set up the development environment and tools
2. [Hello World](hello-world) - Create your first confidential instruction
3. [Computation Lifecycle](computation-lifecycle) - Understand how confidential computations work
4. [TypeScript SDK Reference](https://ts.arcium.com/api) - Complete API documentation for TypeScript client libraries

The network is currently in Public Testnet. Join our [Discord](https://discord.com/invite/arcium) to join our community and start building.

# Installation

## Quick Install (Recommended)

On Mac and Linux, run this single command to install Arcium:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://arcium-install.arcium.workers.dev/ | bash
```

`arcup` is a tool for managing versioning of the Arcium tooling (including the CLI and Arx Node). More info on it can be found [here](installation/arcup).

This script will:

* Check for all required dependencies
* Install Linux build dependencies automatically (if needed)
* Download and install `arcup` for your platform
* Install the latest Arcium CLI (command-line interface for interacting with the Arcium network and managing computations)
* Install the Arx Node (the core node software that performs encrypted computations in the network)

### Prerequisites

Before running the installation script, make sure you have these dependencies installed:

* **Rust**: Install from [here](https://www.rust-lang.org/tools/install)
* **Solana CLI**: Install from [here](https://docs.solana.com/cli/install-solana-cli-tools), then run `solana-keygen new`
* **Yarn**: Install from [here](https://yarnpkg.com/getting-started/install)
* **Anchor**: Install from [here](https://www.anchor-lang.com/docs/installation)
* **Docker & Docker Compose**: Install Docker from [here](https://docs.docker.com/engine/install/) and Docker Compose from [here](https://docs.docker.com/compose/install/)

The installation script will check for all these dependencies and provide clear instructions if any are missing.

## Manual Installation

If you prefer to install manually, you can still use the traditional method. arcup is a tool for managing versioning of the arcium tooling (including the CLI and Arx Node). More info on it can be found [here](installation/arcup).

Install `arcup`. We currently support 4 pre-built targets, listed below. We do not support Windows at the moment.

* `aarch64_linux`
* `x86_64_linux`
* `aarch64_macos`
* `x86_64_macos`

You can install it by replacing `<YOUR_TARGET>` with the target you want to install, and running the following command:

{% tabs %}
{% tab title="Arch Linux" %}
```bash
TARGET=aarch64_linux && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.2.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
```
{% endtab %}

{% tab title="x86 Linux" %}
```bash
TARGET=x86_64_linux && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.2.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
```
{% endtab %}

{% tab title="Apple Silicon" %}
```bash
TARGET=aarch64_macos && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.2.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
```
{% endtab %}

{% tab title="Intel Mac" %}
```bash
TARGET=x86_64_macos && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.2.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
```
{% endtab %}
{% endtabs %}

Install the latest version of the CLI using `arcup`:

```bash
arcup install
```

Verify the installation:

```bash
arcium --version
```

## Issues

Installation might fail due to a variety of reasons. This section contains a list of the most common issues and their solutions, taken from anchor's installation guide.

### Platform-Specific Issues

**Windows Users:** Arcium is not currently supported on Windows. We recommend using Windows Subsystem for Linux (WSL2) with Ubuntu for the best experience.

**Linux Systems:** You may need additional dependencies. On Ubuntu/Debian:

```bash
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev
```

### Incorrect `$PATH`

Rust binaries, including `arcup` and `arcium`, are installed to the `~/.cargo/bin` directory. Since this directory is required to be in the `PATH` environment variable, Rust installation tries to set it up automatically, but it might fail to do so on some platforms.

To verify that the `PATH` environment variable was set up correctly, run:

```shell
which arcium
```

The output should look like (with your username):

```
/home/user/.cargo/bin/arcium
```

**Shell-Specific PATH Issues:**

If `which arcium` returns nothing, add the cargo bin directory to your PATH:

*   **Bash/Zsh:** Add to `~/.bashrc` or `~/.zshrc`:

    ```bash
    export PATH="$HOME/.cargo/bin:$PATH"
    ```
*   **Fish:** Add to `~/.config/fish/config.fish`:

    ```bash
    set -gx PATH $HOME/.cargo/bin $PATH
    ```

After editing, restart your terminal or run `source ~/.bashrc` (or equivalent for your shell).

# Arcup Version Manager

The `arcup` version manager enables easy installation and management of the Arcium Networks' tooling suite, consisting of the Arcium CLI binary, the Arx Node Docker image, and the Postgres Docker image (needed to run the Callback Server). With a single command you can install all of the necessary tools, as well as update all of them when there are new releases.

The [Quick Start](#quick-start) section below takes you through basic `arcup` onboarding, however you can find more detailed installation instructions [here](). Also, see the [Versioning section below](#inter-component-versioning) for details on how versioning is handled between the different components of the Arcium Network.

## Quick Start

First, delete any local versions of the CLI, or Arx Node (Docker) that you may currently have installed on your machine (if you don't have any currently installed, you can skip this step):

```bash
rm $HOME/.cargo/bin/arcium
docker images | grep "arcium-hq" | awk '{print $1":"$2}' | xargs docker rmi -f
```

Verify that you do not have any versions of the CLI, or Arx Node (Docker) installed on your machine now:

```bash
arcium --version # Should return "No such file or directory"
docker images # Should not show any arcium-related images
```

Next, install `arcup` on your machine by following [these steps](..#installation). Then run the `arcup` install command:

```bash
arcup install # Will install the latest releases of the Arcium components
```

Now verify that everything is installed correctly:

```bash
arcium --version # Should show the latest CLI version
arcup version # Shows the currently installed versions of all of the Arcium components
docker images # Should list the images for the Arx Node, and Postgres
```

You can also install older versions using the `install` command (and specifying a version), as well as deleting installed versions with the `delete` command, and switching between already installed versions using the `use` command. See the [Available Commands](#available-commands) section below for full details.

## Inter-Component Versioning

The `arcup` version manager is based on [semver](https://semver.org/) (`MAJOR.MINOR.PATCH`). With `arcup`, the `PATCH` version number need not be in-sync across the different components, however the `MAJOR.MINOR` version number will always be in-sync across all of the Arcium components. As such, `PATCH` changes are always non-breaking with respect to the other Arcium components.

For example, if the current versions are:

* CLI: `0.2.4`
* Arx Node: `0.2.15`

If a breaking change is made to the CLI (e.g. increment to `0.3.0`), the `MINOR` version number of Nodes is also incremented (so both would become `0.3.0`). However, if only a (non-breaking) `PATCH` upgrade is made to tooling, then tooling would increment to `0.2.5` and node would remain unchanged.

## Available Commands

```bash
install  Install the latest (or a specific) version of Arcium components (Arx Node and CLI)
update   Update all Arcium components (Arx Node and CLI) to the latest version
list     List all installed versions
version  Show currently active version
use      Switch to using a specific installed version
delete   Delete a specific version
help     Print this message or the help of the given subcommand(s)
```
# Hello World with Arcium

## Hello World

The Arcium tooling suite for writing MXEs (MPC eXecution Environments) is built on top of [Anchor](https://www.anchor-lang.com/), so if you're familiar with Anchor, you should find Arcium to be a familiar experience, except that you're using the `arcium` CLI instead of `anchor`.

To initialize a new MXE project, you can therefore simply run:

```bash
arcium init <project-name>
```

This will create a new project with the given name, and initialize it with a basic structure. The structure is the same as in an Anchor project with two differences, so we won't repeat it here (for an explanation of the Anchor project structure, see the [Anchor documentation](https://www.anchor-lang.com/docs/quickstart/local)). The two differences are:

* The `Arcium.toml` file, which contains the configuration for the Arcium tooling suite.
* The `encrypted-ixs` directory. This is where we write all our code that is meant to operate on encrypted data and therefore runs in MPC. This code is written using our own Rust framework called [Arcis](arcis). This will already be populated with a simple example called `add_together.rs`. Let's take a closer look at it.

### Our first encrypted instruction

```rust
use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct InputValues {
        v1: u8,
        v2: u8,
    }

    #[instruction]
    pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> Enc<Shared, u16> {
        let input = input_ctxt.to_arcis();
        let sum = input.v1 as u16 + input.v2 as u16;
        input_ctxt.owner.from_arcis(sum)
    }
}
```

Let's go through it line by line. `use arcis_imports::*;` imports all the necessary types and functions for writing encrypted instructions with Arcis. The `#[encrypted]` attribute marks a module that contains encrypted instructions. Inside this module, we define a struct `InputValues` that contains the two values we want to encrypt and pass to the encrypted instruction.

The `#[instruction]` macro marks the function as an entry point for MPC execution - while you can write helper functions without this attribute, only functions marked with `#[instruction]` will be compiled into individual circuits that can be called onchain.

The function `add_together` takes an encrypted input parameter of type `Enc<Shared, InputValues>`. Let's break this down:

* `Enc<Owner, Data>` is Arcium's encrypted data type
* `Shared` means the data is encrypted with a shared secret between the client and MXE (both can decrypt it)
* `InputValues` is the actual data structure being encrypted (our struct with v1 and v2)
* The alternative to `Shared` is `Mxe`, where only the MXE can decrypt the data

Inside the function:

1. `input_ctxt.to_arcis()` converts the input into a form we can operate on within the MPC environment.
2. We perform the addition operation, casting the u8 values to u16 to prevent overflow.
3. `input_ctxt.owner.from_arcis(sum)` converts the encrypted sum into an encrypted format that can be stored onchain, while maintaining encryption with the shared secret between the client and the MXE.

### Calling it from Solana

Now that we've written our first confidential instruction, let's see how can use it from within a Solana program. Our default project already contains a Solana program in the `programs/` directory. Let's take a closer look at it too:

```rust
use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

// This constant identifies our encrypted instruction for on-chain operations
// comp_def_offset() generates a unique identifier from the function name
const COMP_DEF_OFFSET_ADD_TOGETHER: u32 = comp_def_offset("add_together");

declare_id!("YOUR_PROGRAM_ID_HERE");

#[arcium_program]
pub mod hello_world {
    use super::*;

    pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn add_together(
        ctx: Context<AddTogether>,
        computation_offset: u64,
        ciphertext_0: [u8; 32],
        ciphertext_1: [u8; 32],
        pub_key: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        let args = vec![
            Argument::EncryptedU8(ciphertext_0),
            Argument::EncryptedU8(ciphertext_1),
            Argument::ArcisPubkey(pub_key),
            Argument::PlaintextU128(nonce),
        ];
        queue_computation(ctx.accounts, computation_offset, args, vec![], None)?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "add_together")]
    pub fn add_together_callback(
        ctx: Context<AddTogetherCallback>,
        output: ComputationOutputs<AddTogetherOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(AddTogetherOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(SumEvent {
            sum: o.ciphertexts[0],
            nonce: o.nonce.to_le_bytes(),
        });
        Ok(())
    }
}
```

For the sake of brevity, we don't include the `InitAddTogetherCompDef`, `AddTogether`, and `AddTogetherCallback` account structs here, but they're automatically generated when you run `arcium init`. Here's a simplified version of what `AddTogether` looks like:

```rust
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct AddTogether<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // ... other required Arcium accounts (see program/ section for full details)
}
```

You can read more about them and the invokation of confidential instructions inside solana programs [here](program).

The key things to note here are that every mxe program is identified by the `#[arcium_program]` macro (which replaces anchor's `#[program]` macro) and that for every confidential instruction, we generally have three instructions in our solana program:

* `init_add_together_comp_def`: This is the instruction that initializes the confidential instruction definition. It is used to set up the computation definition and is therefore only called once prior to the first invocation of the confidential instruction. More info on this can be found [here](program/computation-def-accs).
* `add_together`: This is the instruction that invokes the confidential instruction. It takes in the arguments for the confidential instruction and queues it for execution using the Arcium program. More info on this can be found [here](program).
* `add_together_callback`: This is the instruction that is called by the MPC cluster when the confidential instruction has finished executing which returns our result. More info on this can be found [here](program).

This is due to the general flow of computations throughout Arcium, which you can read more about [here](computation-lifecycle).

## Building and testing

Similar to anchor, we can build the confidential instructions and Solana programs using `arcium build`. Testing is done using the `@arcium-hq/client` typescript library (more info on it can be found [here](js-client-library)) by default and can be run using `arcium test` (make sure you have installed the npm dependencies prior by running `yarn` or `npm install` in your project directory).

Let's take a quick look at the default test file. Note that some helper functions and imports are excluded for brevity, but you can find the complete examples in your generated project:

```typescript
describe("Hello World", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;
  const provider = anchor.getProvider();

  const arciumEnv = getArciumEnv();

  it("Is initialized!", async () => {
    const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

    console.log("Initializing add together computation definition");
    const initATSig = await initAddTogetherCompDef(program, owner, false);
    console.log(
      "Add together computation definition initialized with signature",
      initATSig
    );

    const privateKey = x25519.utils.randomPrivateKey();
    const publicKey = x25519.getPublicKey(privateKey);
    const mxePublicKey = await getMXEPublicKeyWithRetry(
      provider as anchor.AnchorProvider,
      program.programId
    );

    console.log("MXE x25519 pubkey is", mxePublicKey);
    const sharedSecret = x25519.getSharedSecret(privateKey, mxePublicKey);
    const cipher = new RescueCipher(sharedSecret);

    const val1 = BigInt(1);
    const val2 = BigInt(2);
    const plaintext = [val1, val2];

    const nonce = randomBytes(16);
    const ciphertext = cipher.encrypt(plaintext, nonce);

    const sumEventPromise = awaitEvent("sumEvent");
    const computationOffset = new anchor.BN(randomBytes(8), "hex");

    const queueSig = await program.methods
      .addTogether(
        computationOffset,
        Array.from(ciphertext[0]),
        Array.from(ciphertext[1]),
        Array.from(publicKey),
        new anchor.BN(deserializeLE(nonce).toString())
      )
      .accountsPartial({
        computationAccount: getComputationAccAddress(
          program.programId,
          computationOffset
        ),
        clusterAccount: arciumEnv.arciumClusterPubkey,
        mxeAccount: getMXEAccAddress(program.programId),
        mempoolAccount: getMempoolAccAddress(program.programId),
        executingPool: getExecutingPoolAccAddress(program.programId),
        compDefAccount: getCompDefAccAddress(
          program.programId,
          Buffer.from(getCompDefAccOffset("add_together")).readUInt32LE()
        ),
      })
      .rpc({ commitment: "confirmed" });
    console.log("Queue sig is ", queueSig);

    const finalizeSig = await awaitComputationFinalization(
      provider as anchor.AnchorProvider,
      computationOffset,
      program.programId,
      "confirmed"
    );
    console.log("Finalize sig is ", finalizeSig);

    const sumEvent = await sumEventPromise;
    const decrypted = cipher.decrypt([sumEvent.sum], sumEvent.nonce)[0];
    expect(decrypted).to.equal(val1 + val2);
  });
});
```

This test demonstrates the complete flow of encrypted computations in Arcium. Here's what each key step does:

* `initAddTogetherCompDef`: Call the `init_add_together_comp_def` instruction to initialize the confidential instruction definition. (only need to be called once after the program is deployed)
* `getMXEPublicKeyWithRetry`: Fetch the MXE's x25519 public key.
* `x25519.utils.randomPrivateKey`: Generate a random private key for the x25519 key exchange.
* `x25519.getPublicKey`: Generate the public key corresponding to the private key we generated above.
* `x25519.getSharedSecret`: Generate the shared secret with the MXE cluster using a x25519 key exchange.
* `cipher = new RescueCipher(sharedSecret)`: Initialize the Rescue cipher (the constructor internally performs a HKDF with HMAC based on the Rescue-Prime hash function, you can learn more [here](encryption))
* `cipher.encrypt`: Encrypt the inputs for the confidential instruction.
* `awaitEvent`: Wait for the `sumEvent` event to be emitted by the program on finalization of the computation (in the callback instruction).
* `addTogether`: Call the `add_together` instruction to invoke the confidential instruction.
* `awaitComputationFinalization`: Since waiting for an Arcium computation isn't the same as waiting for one Solana transaction (since we need to wait for the MPC cluster to finish the computation and invoke the callback), we wait using this function, which is provided by the Arcium typescript library.

## Ready to Deploy?

Now that you've built and tested your MXE locally, you're probably eager to see it running on devnet! Head over to our [deployment guide](deployment) where we'll walk you through getting your MXE live on Solana devnet. We'll cover everything from choosing the right RPC endpoint to initializing your computation definitions.

## What's Next?

Now that you've built your first MXE, you're ready to deploy it to testnet. Follow the [deployment guide](deployment) to get your MXE running on Solana devnet and test with real encrypted computations.

From there, you can build more sophisticated applications by learning about [input/output patterns](arcis/input-output) for working with encrypted data, [callback accounts](program/callback-accs) for persistent state, and [JavaScript client integration](js-client-library/encrypting) for frontend development.

For inspiration, browse our [examples repo](https://github.com/arcium-hq/examples/) to see voting systems, games, and DeFi applications built with Arcium. If you need help, join our [Discord community](https://discord.gg/arcium) where other builders share tips and get support.

# Arcium Computation Lifecycle

Before diving into the details of the tooling, it's useful to understand the general architecture of Arcium. The below diagram gives a high-level overview of the lifecycle of a typical interaction with Arcium (we call these "computations").

{% @mermaid/diagram content="sequenceDiagram
    participant Client
    participant MXE Program
    participant Arcium Program
    participant MPC Cluster
    participant Callback Server
    
    Client->>Client: Encrypt params
    Client->>MXE Program: Invoke computation with encrypted params
    MXE Program->>Arcium Program: Handle & format params and send to Arcium Program
    Arcium Program->>Arcium Program: Queue Computation in MXE's Mempool
    MPC Cluster->>Arcium Program: Fetch new computation from mempool
    MPC Cluster->>MPC Cluster: Compute using MPC
    MPC Cluster->>Arcium Program: Callback with Result
    MPC Cluster->>Callback Server: Send additional data (if any)
    Callback Server->>Callback Server: Handle data update to on-chain accounts
    Callback Server->>MXE Program: Invoke callback instruction (if additional data was sent)
    Arcium Program->>Arcium Program: Verify Result
    Arcium Program->>MXE Program: Invoke callback instruction with result
    MXE Program->>MXE Program: Handle Result
    MXE Program->>Client: Notify of completion" %}

We have 4 key actors here (with one additional participant if needed):

* The client: The party that wants to perform a computation, usually the user of your MXE. This is implemented using the [Arcium TypeScript Client Library](broken-reference).
* The MXE Program: Your app. An MXE (MPC eXecution Environment) consists of everything needed to perform computations and is implemented using the [Arcium program tooling](broken-reference):
  * A smart contract that is deployed on the blockchain and is used to format, submit submit computations to Arcium.
  * A set of confidential instructions (we call these "computation definitions") that are used to define what parameters are needed for the computation and what the computation is. Writing these is done using [Arcis](arcis).
  * Some metadata about the MXE, most importantly the MPC cluster we would like to use to compute our computations.
* The Arcium Program: The program in charge of assigning, scheduling, and verifying computations for the various MPC clusters to perform.
* The MPC Cluster: The parties that are performing the client's computations using MPC.
* The Callback Server: A server that is used to handle additional data from the MPC cluster. This is optional and only needed for cases when the computation result is more than what can fit in a single Solana transaction.

# Encryption

Encrypted data is passed as an `Enc<Owner, T>` generic type, where `Owner` specifies who can decrypt the data (either `Shared` or `Mxe`), and `T` is the underlying data type being encrypted. In the case of `Mxe`, the nodes collectively can decrypt the data under dishonest majority assumptions, whereas if the `Owner` is `Shared`, then the data was encrypted using a shared secret between the user and the MXE. Underneath the hood, this generic wrapper type contains the encrypted data, as well as the public key (only for `Shared` owner) and nonce used to encrypt the data.

Encrypted data can be decrypted globally or selectively to a given user. For global decryption, you can call `reveal` method on any variable of [supported data type](arcis/types). Read more about how we enable this using re-encryption (aka sealing) in Arcium [here](encryption/sealing).

Private inputs are encrypted using the arithmetization-oriented symmetric [Rescue cipher](https://eprint.iacr.org/2019/426). Prior to the encryption, a [x25519](https://www.rfc-editor.org/rfc/rfc7748.html#page-7) elliptic curve Diffie-Hellman key exchange is performed between the client and the cluster to derive a common shared secret. The Rescue key is obtained by applying the [HKDF](https://datatracker.ietf.org/doc/html/rfc5869) key derivation to the shared secret. This increases the min-entropy of the key.\
Note:

1. Since the x25519 key exchange natively returns shared secrets in the finite field with $$p = 2^{255} - 19$$ elements, we implemented Rescue over the field $$\mathbb{F}_{p}$$. States in the context of Rescue are elements of the $$m$$-dimensional vector space $$\mathbb{F}_p^m$$, i.e., the Rescue cipher transforms vectors of size $$m$$ to vectors of the same size.
2. The security level $$s$$ of the cipher is set to 128 bits.
3. We use the Rescue block cipher in [Counter (CTR) mode](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf) (see Section 6.5), with fixed $$m = 5$$. The choice $$m = 5$$ is motivated by the fact that it is the smallest value that attains the minimum of recommended rounds (10), given the fixed finite field and security level. The `counter`s are of the form `[nonce, i, 0, 0, 0]`, where `nonce` are 16 random bytes provided by the user.
4. The hash function used for the key derivation is [Rescue-Prime](https://eprint.iacr.org/2020/1143.pdf) over $$\mathbb{F}_{2^{255}-19}$$, with $$m = 6$$ and `capacity = 1` (yielding `rate = 5`, which matches the size of the states for the Rescue cipher, see 3.). According to [Section 2.2](https://eprint.iacr.org/2020/1143.pdf), this offers 255 / 2 bits of security against collision, preimage and second-preimage attacks.

The decryption of `input_enc: Enc<Owner, T>` can conveniently be obtained by calling `input_enc.to_arcis()` (the nodes do not learn `input`, they simply convert the ciphertext to secret-shares of `input` by running the Rescue decryption circuit in MPC). If the owner is `Shared`, the MXE and the client perform a key exchange first. Similarly, `owner.from_arcis(output)` encrypts the secret-shared `output` by running the Rescue encryption circuit in MPC.\
Note:

1. After decrypting the user-provided inputs, the MXE increments the `nonce` by 1 and uses it for encrypting the outputs. For the forthcoming interaction with the MXE, a new `nonce` must be provided.
2. The performance will benefit from reducing the number of calls to `owner.from_arcis(..)` (per owner). Ideally, put all data encrypted to `owner` in one struct.

# Sealing aka re-encryption

Suppose you're Alice, and you have secret data onchain, and you want to share it with Bob. Or it could be that you want to compute a function on your sensitive data, and share the result with Bob without revealing the data, or the result to anyone else.

Arcium enables you to re-encrypt any data to a given public key. This is known as "sealing" in cryptography, effectively having the ability to restrict data access and information flow.

This is useful for a variety of reasons, such as compliance, end-to-end privacy, and more.

```rust
#[encrypted]
mod circuits {
    use arcis_imports::*;

    #[instruction]
    pub fn verify_loan_eligibility(
        alice_balance: Enc<Shared, u64>,
        min_balance_required: Enc<Mxe, u64>,
        loan_officer: Shared
    ) -> Enc<Shared, bool> {
        let balance = alice_balance.to_arcis();
        let threshold = min_balance_required.to_arcis();

        // Check if Alice meets minimum balance for loan without revealing her exact balance
        let is_eligible = balance >= threshold;

        // Re-encrypt the result for the loan officer
        loan_officer.from_arcis(is_eligible)
    }
}
```

In this example, we have a confidential function `verify_loan_eligibility` that takes Alice's encrypted balance (encrypted with a shared secret between Alice and the MXE), the minimum balance requirement (encrypted only for the MXE), and a `Shared` type parameter representing the loan officer who will receive the result.

The function checks if Alice meets the minimum balance requirement for loan eligibility without revealing her actual balance to anyone. The boolean result is then re-encrypted specifically for the loan officer using their public key. This way, Alice's financial privacy is preserved - the loan officer only learns whether she's eligible, not her actual balance, and the MPC nodes never see the unencrypted values.

# Arcis

Arcis is a Rust-based framework designed for writing secure multi-party computation (MPC) circuits to be executed on the Arcium network. It provides developers with a powerful and intuitive interface to create privacy-preserving applications that can compute over encrypted data.

## Key Features

* **Rust-based**: Leverage the safety and performance of Rust in your MPC development.
* **Circuit-oriented**: Design and implement MPC circuits with ease.
* **Privacy-focused**: Enable computations on encrypted data without revealing the underlying information.

In the following sections, we'll dive deeper into Arcis' syntax, core components, and best practices for building efficient and secure MPC circuits.

# Operations

## Operations

Arcis supports many of Rust's native operations but extends them to work seamlessly with encrypted data, allowing you to write private computations using familiar Rust syntax. See the tables below for a detailed list of supported and unsupported operations.

### Table of contents

* [Expression support](#expression-support)
  * [Binary expressions](#binary-expressions)
  * [Casts](#cast-expressions)
  * [Literals](#literal-expressions)
  * [Methods](#method-calls)
  * [Paths](#paths)
* [Item support](#item-support)
* [Pattern support](#pattern-support)

## Expression support:

| Expression Name   | Example                        | Support         | Comments                                                                 |
| ----------------- | ------------------------------ | --------------- | ------------------------------------------------------------------------ |
| Array literal     | `[a, b]`                       | Supported       |                                                                          |
| Assignment        | `a = b;`                       | Supported       |                                                                          |
| Async block       | `async { ... }`                | Unsupported     |                                                                          |
| Await             | `foo().await`                  | Unsupported     |                                                                          |
| Binary expression | `a + b`                        | Partial Support | [See table below](#binary-expressions) for supported binary expressions. |
| Block expression  | `{ ... }`                      | Supported       |                                                                          |
| Break             | `break;`                       | Unsupported     |                                                                          |
| Function call     | `f(a, b)`                      | Partial Support | [See table below](#function-calls) for supported functions.              |
| Casts             | `a as u16`                     | Partial Support | [See table below](#cast-expressions) for supported conversions.          |
| Closures          | `\|a, b \| a + b`              | Supported       |                                                                          |
| Const block       | `const { ... }`                | Supported       |                                                                          |
| Continue          | `continue;`                    | Unsupported     |                                                                          |
| Field access/set  | `obj.field`                    | Supported       |                                                                          |
| For loop          | `for i in expr { ... }`        | Supported       | Note that `expr` will have its length known at compile-time.             |
| If                | `if cond { ... } else { ... }` | Supported       | Complexity is in O( then\_block + else\_block).                          |
| Indexing          | `a[idx]`                       | Supported       | Complexity will be in O(`a.len()`) if `idx` isn't known at compile-time. |
| If let            | `if let Some(x) = ...`         | Unsupported     |                                                                          |
| Literals          | `1u128`                        | Partial Support | [See table below](#literal-expressions) for supported literals.          |
| Loops             | `loop { ... }`                 | Unsupported     | Cannot be supported as the number of iterations is not known.            |
| Macros            | `println!("{}", q)`            | Partial Support | [See table below](#macros) for supported macros.                         |
| Match             | `match n { ... }`              | Unsupported     |                                                                          |
| Method calls      | `x.foo(a, b)`                  | Partial Support | [See table below](#method-calls) for supported methods.                  |
| Parentheses       | `(a + b)`                      | Supported       |                                                                          |
| Paths             | `Foo::bar`                     | Partial Support | [See table below](#paths) for supported paths.                           |
| Ranges            | `4..5`                         | Partial Support | Not supported in `arr[4..16]`.                                           |
| Raw addresses     | `&raw const foo`               | Unsupported     |                                                                          |
| References        | `&mut foo`                     | Supported       |                                                                          |
| Repeat arrays     | `[4u8; 128]`                   | Supported       |                                                                          |
| Return            | `return false;`                | Unsupported     |                                                                          |
| Struct literals   | `MyStruct { a: 12, b }`        | Supported       |                                                                          |
| Try expression    | `this_call_can_err()?;`        | Unsupported     |                                                                          |
| Tuple literal     | `(a, 4, c)`                    | Supported       |                                                                          |
| Unary expressions | `!x`                           | Partial Support | User-defined unary operations are not supported.                         |
| Unsafe            | `unsafe { ... }`               | Unsupported     |                                                                          |
| While loops       | `while x < 64 { ... }`         | Unsupported     | Cannot be supported as the number of iterations is not known.            |

### Binary expressions

Note: user-defined binary operations are currently unsupported.

| Example    | Supported types                            |
| ---------- | ------------------------------------------ |
| `a + b`    | Integers, floats                           |
| `a - b`    | Integers, floats                           |
| `a * b`    | Integers, floats                           |
| `a / b`    | Integers, floats                           |
| `a % b`    | Integers                                   |
| `a && b`   | Booleans                                   |
| `a \|\| b` | Booleans                                   |
| `a ^ b`    | Booleans                                   |
| `a & b`    | Booleans                                   |
| `a \| b`   | Booleans                                   |
| `a << b`   | None                                       |
| `a >> b`   | Integers, if `b` is known at compile time. |
| `a == b`   | All. Use `derive(PartialEq)` for structs.  |
| `a != b`   | All. Use `derive(PartialEq)` for structs.  |
| `a < b`    | Booleans, integers, floats                 |
| `a <= b`   | Booleans, integers, floats                 |
| `a >= b`   | Booleans, integers, floats                 |
| `a > b`    | Booleans, integers, floats                 |
| `a += b`   | Integers, floats                           |
| `a -= b`   | Integers, floats                           |
| `a *= b`   | Integers, floats                           |
| `a /= b`   | Integers, floats                           |
| `a %= b`   | Integers                                   |
| `a ^= b`   | Booleans                                   |
| `a &= b`   | Booleans                                   |
| `a \|= b`  | Booleans                                   |
| `a <<= b`  | None                                       |
| `a >>= b`  | Integers, if `b` is known at compile time  |

### Cast expressions

`a as MyType` is only supported:

| From Type    | To Type      |
| ------------ | ------------ |
| integer type | integer type |
| `bool`       | integer type |
| integer type | `bool`       |
| `&...&T`     | `&T`         |

### Function calls

The following function calls are supported:

* user-defined function calls (without recursion)
* `ArcisRNG::bool()` to generate a boolean.
* `ArcisRNG::gen_integer_from_width(width: usize) -> u128`. Generates a secret integer between 0 and 2^width - 1 included.
* `ArcisRNG::gen_public_integer_from_width(width: usize) -> u128`. Generates a public integer between 0 and 2^width - 1 included.
* `ArcisRNG::gen_integer_in_range(min: u128, max: u128, n_attempts: usize) -> (result: u128, success: bool)`. See function doc for more information.
* `ArcisRNG::shuffle(slice)` on slices. Complexity is in `O(n*log³(n) + n*log²(n)*sizeof(T))`.
* `Mxe::get()` to be able to create MXE-owned secret data.
* `Shared::new(arcis_public_key)` to share private data with `arcis_public_key`.
* `ArcisPublicKey::from_base58(base58_byte_string)` to create a public key from a base58-encoded address.
* `ArcisPublicKey::from_uint8(u8_byte_slice)` to create a public key from a Uint8 array.

### Literal expressions

| Example     | Support     |
| ----------- | ----------- |
| `"foo"`     | Unsupported |
| `b"foo"`    | Supported   |
| `c"foo"`    | Unsupported |
| `b'f'`      | Supported   |
| `'a'`       | Unsupported |
| `1`         | Supported   |
| `1u16`      | Supported   |
| `1f64`      | Supported   |
| `1.0e10f64` | Supported   |
| `true`      | Supported   |

### Macros

The following macros are supported in order to help you debug your rust code:

* `debug_assert!`, `debug_assert_ne!`, `debug_assert_eq!`. They do not change instruction behavior and are only useful for debugging your rust code.
* `eprint!`, `eprintln!`, `print!`, `println!`. They do not change instruction behavior and are only useful for debugging your rust code.

### Method calls

The following method calls are supported:

* user-defined method calls (without generics and without recursion)
* `.clone()` on all `Clone` objects.
* `.len()`, `.is_empty()`, `.swap(a, b)`, `.fill(value)`, `.reverse()`, `.iter()`, `.iter_mut()`, `.into_iter()`, `.windows(width)` on arrays.
* `.sort()` on arrays of integers. Complexity is in `O(n*log²(n)*bit_size)`.
* `.enumerate()`, `.chain(other)`, `.cloned()`, `.copied()`, `.count()`, `.rev()`, `.zip(other)`, `.map(func)`, `.for_each(func)`, `.fold(init, func)` on iterators.
* `.take(n)`, `.skip(n)`, `.step_by(n)` on iterators when `n` is compile-time known.
* `.reveal()` if not inside a `if` or a `else`
* `.to_arcis()` on `Enc`s
* `.from_arcis(x)` on `Owner`s (objects of types `Mxe` or `Shared`) if not inside a `if` or a `else`
* `.abs()`, `.min(x)`, `.max(x)` on integers and floats
* `.abs_diff(other)`, `.is_positive()`, `.is_negative()` on integers
* `.to_le_bytes()`, `to_be_bytes()` on typed integers (does not work on integers the interpreter does not know the type)
* `.exp()`, `.exp2()`, `.ln()`, `.log2()`, `.sqrt()` on floats.

### Paths

The following paths are supported:

* `IntType::BITS`, `IntType::MIN` and `IntType::MAX` where `IntType` is an integer type.
* Paths to user-defined constants, functions and structs, as long as they don't use the unsupported `crate` or `super`.
* `std::mem::replace` and `std::mem::swap`

## Item support:

| Item Name         | Example                   | Support         | Comments                                                      |
| ----------------- | ------------------------- | --------------- | ------------------------------------------------------------- |
| Constant          | `const MAX: u16 = 65535`  | Supported       |                                                               |
| Enum              | `enum MyEnum { ... }`     | Unsupported     |                                                               |
| Extern            | `extern ...`              | Unsupported     |                                                               |
| Functions         | `fn foo() -> u8 { 0 }`    | Partial Support | Recursive functions are not supported.                        |
| Impls             | `impl MyType { ... }`     | Partial Support | Traits are not supported. `MyType` should not be a reference. |
| Macro Definitions | `macro_rules! ...`        | Unsupported     |                                                               |
| Macro Invocations | `println!(...)`           | Partial Support | [See table above](#macros) for supported macros.              |
| Modules           | `mod my_module { ... }`   | Supported       |                                                               |
| Statics           | `static ...`              | Unsupported     |                                                               |
| Structs           | `struct MyStruct { ... }` | Supported       |                                                               |
| Traits            | `trait MyTrait { ... }`   | Unsupported     |                                                               |
| Type Aliases      | `type MyId = usize;`      | Supported       |                                                               |
| Union             | `union MyUnion { ... }`   | Unsupported     |                                                               |
| Use               | `use arcis_imports::*`    | Partial Support | Only `use arcis_imports::` is supported.                      |

## Pattern support:

The following patterns are supported in function arguments and `let` statements:

* simple idents: `let ident = ...;`
* mutable idents: `let mut ident = ...;`
* ref idents: `let ref ident = ...;`
* mutable ref idents: `let ref mut ident = ...;`
* parentheses around a supported pattern: `let (...) = ...;`
* reference of a supported pattern: `let &... = ...;`
* array of supported patterns: `let [...] = ...;`
* struct of supported patterns: `let MyStruct { ... } = ...;`
* tuple of supported patterns: `let (...) = ...;`
* tuple struct of supported patterns: `let MyStruct(...) = ...;`
* type pattern of a supported pattern: `let ...: ty = ...;`
* wild pattern: `let _ = ...;`

Note: in particular, the `..` pattern is currently unsupported.

#### Performance Considerations

While Arcis provides these operations on encrypted data, it's important to note that operations on encrypted data are more computationally expensive than their plaintext counterparts. Complex calculations can lead to increased computation time and resource usage. It's recommended to optimize your algorithms to minimize the number of operations on encrypted data where possible.

# Types

The following types are supported:

* `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
* `f64`, `f32` (Note: these types are emulated by the fixed-point numbers `k-2^-52`, for `k` between `-2^250` and `2^250`.)
* tuples of supported types, including `()`
* fixed-length arrays of a supported type
* (mutable) references to a supported type
* user-defined structs of supported types
* functions (but not as input or output of an encrypted instruction)
* `ArcisPublicKey`, an Arcis public key wrapper.
* Arcis-defined `Enc`, `Mxe` and `Shared`.

In particular, we do not currently support `HashMap`, `Vec`, `String` (we do not support types with a variable `len`). Constant-size byte strings (like `b"hello_world"`) are supported.

Here, `Enc` type defines the encrypted data input, which is used as `Enc<Owner, T>` where `Owner` can be either `Mxe` or `Shared`, signaling which party the data of type `T` can be decrypted by. You can read more about dealing with encrypted inputs/outputs [here](input-output).

Note: Currently all these types get mapped to secret shares over the curve25519 scalar field under the hood, meaning they all take up the same amount of space. Changing this for better space utilization is on the roadmap and will be implemented soon.

# Input/Output

Inputs and outputs in confidential instructions are handled in the same way. Arcium network does not mutate any state itself. Both inputs and outputs can be encrypted or plaintext data, either being passed by value or by reference. Passing by reference is only possible for account data, where the Arcium nodes will be able fetching data from account. This is beneficial for accounts where data is larger than what can fit in a single Solana transaction, or if you want to avoid storage costs for the data while the computation is in progress (as each input has to be written to a computation object for the duration of the computation).

Encrypted data is passed as an `Enc<Owner, T>` generic type, where `Owner` specifies who can decrypt the data:

* **`Enc<Shared, T>`**: Data encrypted with a shared secret between the client and MXE. Both the client and MXE can decrypt this data. Use this when:
  * Accepting user inputs that the user needs to verify later
  * Returning results the user must be able to decrypt
  * Implementing privacy-preserving user interactions
* **`Enc<Mxe, T>`**: Data encrypted exclusively for the MXE. Only the MXE nodes (acting together) can decrypt this data. Use this when:
  * Storing internal state that users shouldn't access directly
  * Passing data between MXE functions
  * Protecting protocol-level data from individual users

Learn more about [encryption in Arcium Network](../encryption).

```rust
// Define the data structures we'll work with
struct Order {
    size: u64,
    bid: bool,
    owner: u128,
}

// OrderBook must be a fixed-size structure for MPC
const ORDER_BOOK_SIZE: usize = 100; // Maximum orders supported

struct OrderBook {
    orders: [Order; ORDER_BOOK_SIZE],
}

#[instruction]
pub fn add_order(
    order_ctxt: Enc<Shared, Order>,
    ob_ctxt: Enc<Mxe, &OrderBook>,
) -> Enc<Mxe, OrderBook> {
    let order = order_ctxt.to_arcis();
    let mut ob = *(ob_ctxt.to_arcis());
    let mut found = false;
    for i in 0..ORDER_BOOK_SIZE {
        let overwrite = ob.orders[i].size == 0 && !found;
        if overwrite {
            ob.orders[i] = order;
        }
        found = overwrite || found;
    }
    ob_ctxt.owner.from_arcis(ob)
}
```

Let's use this example to understand how to pass inputs into confidential instructions, compute on them and return outputs. Here, we are trying to add an order to an existing order book.

In this example, `order_ctxt: Enc<Shared, Order>` is passed by value, meaning the entire encrypted order data is submitted onchain. In contrast, `ob_ctxt: Enc<Mxe, &OrderBook>` is passed by reference - only the account's public key is submitted onchain, and the MPC nodes will fetch the actual data from that account during computation. This is particularly useful for large data structures like order books that might not fit in a single transaction.

In order to use the parameters `order_ctxt` and `ob_ctxt` for computation, we need to convert them to corresponding secret shares for the nodes to compute in MPC. This can be done by calling `to_arcis` function on any `Enc` generic parameter. This does not reveal the plaintext data underneath to the nodes during the process.

Here, the order parameter disappears after the confidential instruction has been processed (just as you'd expect in regular rust too). To output the new order book, we convert it back using `from_arcis` on `ob_ctxt.owner` field which defines the owner, aka the party which encrypted the data, to get the new `Enc<Origin, T>` type, and return it.

Currently, as many outputs as can fit in a single transaction are sent in the callback transaction, whereas the rest are all sent to the [callback server](../callback-server) for state updates. This means that you might need to make state changes through the callback server, and are responsible for updating the on-chain accounts, if needed.

For more details on how to invoke these encrypted instructions from your Solana program, see [Invoking a Computation](../program).

# Best practices

We provide here a set of miscellaneous tips and tricks and things to keep in mind when writing confidential instructions and programs.

## Execution flow

While we strive to make the Arcis compiler accept inputs that are as close as possible to standard rust code, there are some differences. Fundamentally, this is because code emitted by the Arcis compiler will not depend on the value of private inputs and so must be data independent (Intuitively, this makes sense. If the execution flow of our code depended on the value of a private input, an external observer could use this to learn information about the private values). We highlight here a few examples to keep in mind.

* If/else statements. We would normally not be able to use a masked value in the condition of an if statement. The Arcis compiler will however interpret this correctly and rewrite it into a data independent form. Since this is done by a macro, some syntax valid for the rust compiler will not be accepted (missing else branch, or else if clauses). Additionally, you will not gain any performance from using a masked value in the condition of an if statement: the program will still execute both branches, and just not use the result of the branch that is not taken.
* In general, control flow behavior that depends on a masked value is not supported. This includes early returns, or break statements in for loops, for example. A good rule of thumb is that the execution flow should be the same, no matter what value is masked.
* Currently, variable sized types such as `Vec<T>` are also not supported as length of the data should be known at compile time.

## Operations

Arcium supports multiple MPC backends, but all are based on additive-secret sharing. This has a few implications on what operations are more and less expensive, so we present a few guidelines below. Of course, performance always depends on the exact circuit. These are heuristic and not rules.

* Multiplications between secret values are significantly more expensive than on plaintext, as they involve heavy pre-processing and communication. - Multiplications between a secret and a plaintext value, as well as additions between secret/plaintext values, are basically free and run at pretty much the same speed as on plaintext data.
* Comparisons require conversion from Scalars to arrays of boolean bits which we then compare element-wise. This is a relatively expensive operation. A good rule of thumb is therefore the ordering of performance (where additions is the cheapest operation) is additions -> multiplications -> comparisons.

# Invoking a Computation from your Solana program

Before reading this, we recommend having read the [Computation Lifecycle](computation-lifecycle) section and the [Arcis inputs/outputs](arcis/input-output) section.

## The Basics

Let's say we have the following encrypted instruction and want to invoke it from our MXE.

```rust
#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct InputValues {
        v1: u8,
        v2: u8,
    }

    #[instruction]
    pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> Enc<Shared, u16> {
        let input = input_ctxt.to_arcis();
        let sum = input.v1 as u16 + input.v2 as u16;
        input_ctxt.owner.from_arcis(sum)
    }
}
```

To do this, we first need to receive the encrypted parameter of type `InputValues` which contains two encrypted `u8`s, then encode them into the `Argument` format, and finally queue the computation for execution. Additionally, we need to define a callback instruction that will be invoked when the computation is complete. Callback instructions have a few requirements:

1. They must be defined with the `#[arcium_callback(encrypted_ix = "encrypted_ix_name")]` macro.
2. They must have exactly two arguments: `ctx: Context<...>` and `output: ComputationOutputs<T>` where `T` is named as `{encrypted_ix_name}Output`.

For passing encrypted arguments, if the corresponding argument is `Enc<Shared, T>`, then we need to pass the `Argument::ArcisPubkey(pub_key)` and `Argument::PlaintextU128(nonce)`, before the ciphertext. If the corresponding argument is `Enc<Mxe, T>`, then we only need to pass the nonce as `Argument::PlaintextU128(nonce)` and the ciphertext. Ciphertexts are passed as `Argument::EncryptedXYZ(ciphertext)` where `XYZ` is the type of the ciphertext, with the possibilities being `EncryptedU8`, `EncryptedU16`, `EncryptedU32`, `EncryptedU64`, `EncryptedU128`, `EncryptedBool`.

```rust
pub fn add_together(
    ctx: Context<AddTogether>,
    computation_offset: u64,
    ciphertext_0: [u8; 32],
    ciphertext_1: [u8; 32],
    pub_key: [u8; 32],
    nonce: u128,
) -> Result<()> {
    // Build the args the confidential instruction expects (Ciphertext, Ciphertext, u8)
    let args = vec![
        Argument::ArcisPubkey(pub_key),
        Argument::PlaintextU128(nonce),
        Argument::EncryptedU8(ciphertext_0),
        Argument::EncryptedU8(ciphertext_1),
    ];
    // Build & queue our computation (via CPI to the Arcium program)
    queue_computation(
        ctx.accounts,
        // Random offset for the computation
        computation_offset,
        // The one-time inputs our confidential instruction expects
        args,
        // Accounts needed for the callback instruction
        vec![],
        // Callback server address
        // None here because the output of the confidential instruction can fit into a solana transaction
        // as its just 1 Ciphertext which is 32 bytes
        None
    )?;
    Ok(())
}

// Macro provided by the Arcium SDK to define a callback instruction.
#[arcium_callback(encrypted_ix = "add_together")]
pub fn add_together_callback(
    ctx: Context<AddTogetherCallback>,
    output: ComputationOutputs<AddTogetherOutput>,
) -> Result<()> {
    let o = match output {
        ComputationOutputs::Success(AddTogetherOutput { field_0 }) => field_0,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    emit!(SumEvent {
        sum: o.ciphertexts[0],
        nonce: o.nonce.to_le_bytes(),
    });

    Ok(())
}

```

Let's also have a look at the `Accounts` structs for each of these instructions:

```rust
/// Accounts required to invoke the `add_together` encrypted instruction.
/// `add_together` must be the name of the encrypted instruction we're invoking.

#[queue_computation_accounts("add_together", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct AddTogether<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_ADD_TOGETHER)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}
```

That's a lot of accounts to remember! Here's what each one does:

**Core MXE Accounts:**

* `mxe_account`: Your MXE's metadata and configuration
* `mempool_account`: Queue where computations wait to be processed
* `executing_pool`: Tracks computations currently being executed
* `computation_account`: Stores individual computation data and results
* `comp_def_account`: Definition of your encrypted instruction (circuit)

**Arcium Network Accounts:**

* `cluster_account`: The MPC cluster that will process your computation
* `pool_account`: Arcium's fee collection account
* `clock_account`: Network timing information

**System Accounts:**

* `payer`: Pays transaction fees and rent
* `system_program`: Solana's system program for account creation
* `arcium_program`: Arcium's core program that orchestrates MPC

The good news is these can be copy-pasted for any confidential instruction. You only need to change:

1. `COMP_DEF_OFFSET_ADD_TOGETHER` to match your instruction name
2. The instruction name in the `queue_computation_accounts` macro

How about the accounts for the callback instruction?

```rust
#[callback_accounts("add_together", payer)]
#[derive(Accounts)]
pub struct AddTogetherCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    /// Like above, COMP_DEF_PDA_SEED is a constant defined in the Arcium SDK.
    /// COMP_DEF_OFFSET_ADD_TOGETHER is an encrypted instruction specific u32
    /// offset which can be calculated with `comp_def_offset("add_together")`, where
    /// comp_def_offset is a function provided by the Arcium SDK and `add_together`
    /// is the name of the encrypted instruction we're invoking.
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_ADD_TOGETHER)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
}
```

Here it's a lot fewer accounts to fortunately! Like with the `AddTogether` struct, we need to change the parameter for the `derive_comp_def_pda` macro and in the `callback_accounts` macro depending on the encrypted instruction we're invoking. But what if we don't just want to return a raw value and need some additional accounts? Check out [input/outputs](arcis/input-output) for how to handle encrypted data and [callback accounts](program/callback-accs) for returning additional accounts in the callback.

# Computation Definition Accounts

## Why Computation Definitions Exist

When you write an encrypted instruction using Arcis, it gets compiled into an MPC circuit - essentially a program that the MPC nodes can execute securely on encrypted data. But here's the challenge: how do the MPC nodes know what circuit to run when your Solana program calls for a computation?

That's where Computation Definition Accounts come in. They serve as the bridge between your Solana program and the MPC network, storing both the circuit itself and metadata about how to execute it. Think of it as uploading your encrypted instruction to the blockchain so the MPC nodes can access it when needed.

## Computation Definition Accounts

When we define an encrypted instruction using [Arcis](../arcis), we need the MPC cluster that will execute this confidential instruction to have access to the confidential instruction itself, its interface, and some more metadata. This is done by defining a `ComputationDefinitionAccount` struct, which consists of two parts:

1. The confidential instruction metadata and interface.
2. The raw MPC bytecode.

The interface provides data around what input and output types are expected, what accounts are required, and a few other pieces of metadata. It's data is stored in an account with the seeds`b"ComputationDefinitionAccount", mxe_program_id, comp_def_offset`. The first is exported as a constant by the Arcium Anchor SDK, the second is just the program id of our MXE program, and the third is a confidential-instruction-specific offset. It is computed with `comp_def_offset = sha256(<confidential_instruction_name>).slice(0,4)` and then interpreted as a little-endian u32. Theoretically, you shouldn't need to know this, but it's good to know what's going on under the hood. We abstract this with `derive_comp_def_pda` macro which takes in the `comp_def_offset` as a parameter, and computes the `ComputationDefinitionAccount` address for you.

The MPC bytecode is stored inside account(s) with the seeds `b"ComputationDefinitionRaw", comp_def_acc, i`. Like above, the first is exported as a constant by the Arcium Anchor SDK, the second is the computation definition account we defined above, and the third is an index starting from 0 up to however many accounts we need to store the full MPC bytecode.

## Usage

When working locally, you theoretically don't need to care about the MPC bytecode accounts, as the Arcium CLI will handle the creation and management of these accounts for you. You do however need to create the interface ComputationDefinitionAccount, which can easily be done with the Arcium Anchor tooling. Let's say we want to deploy a confidential instruction called `add_together`:

```rust
pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
    init_comp_def(ctx.accounts, true, 0, None, None)?;
    Ok(())
}

#[init_computation_definition_accounts("add_together", payer)]
#[derive(Accounts)]
pub struct InitAddTogetherCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    // The computation definition account that will be created. We can't
    // specify the seeds and account type directly here, as it gets
    // initialized via CPI so these constraints would fail in our non-CPI
    // instruction. This is ok, as the Arcium program will create the
    // account with the correct seeds and account type for us.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}
```

And that's all, we just have to make sure to call this instruction once at the beginning before we can use the confidential instruction.

# Callback Accounts

Callback accounts provide a way to define additional accounts to be used in the callback instruction for a computation. This is helpful when you want to use the output of a computation to modify an onchain account.

**When to use callback accounts:**

* Storing computation results in persistent accounts
* Updating game state, user balances, or protocol data
* Writing results that exceed transaction size limits

Expanding on our [example from before](), let's say we want to save the result of our addition in an account for later use. Let's define an account first to save our data and an instruction to initialize it, as callback accounts must already exist and cannot change in size when being used as part of a computation:

```rust
#[account]
#[derive(InitSpace)]
pub struct SecretAdditionResult{
    pub sum: u8,
}

pub fn init(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"AdditionResult"],
        space = 8 + SecretAdditionResult::INIT_SPACE,
        // Note: In a real implementation you should usually save the bump too,
        // but for the sake of simplicity in this example we skip that
        bump
    )]
    pub add_result_account: Account<'info, SecretAdditionResult>,
    pub system_program: Program<'info, System>,
}
```

Now that we have defined and initialized our account, let's use it in our existing example from before. Let's start with the queue step:

```rust
pub fn add_together(
    ctx: Context<AddTogether>,
    computation_offset: u64,
    ciphertext_0: [u8; 32],
    ciphertext_1: [u8; 32],
    pub_key: [u8; 32],
    nonce: u128,
) -> Result<()> {
    // Note: Using `create_program_address` with the bump would be more efficient than `find_program_address`.
    // Since this PDA is constant, you could also derive it at compile time and save it as a constant.
    // We use find_program_address here for simplicity.
    let addition_result_pda = Pubkey::find_program_address(&[b"AdditionResult"], ctx.program_id).0;

    // Build the args the confidential instruction expects (Ciphertext, Ciphertext, u8)
    let args = vec![
        Argument::ArcisPubkey(pub_key),
        Argument::PlaintextU128(nonce),
        Argument::EncryptedU8(ciphertext_0),
        Argument::EncryptedU8(ciphertext_1),
    ];

    // Build & queue our computation (via CPI to the Arcium program)
    queue_computation(
        ctx.accounts,
        // Random offset for the computation
        computation_offset,
        // The one-time inputs our confidential instruction expects
        args,
        // Additional callback accounts we want to receive when the computation is complete,
        // in this case our account from before. We specify it's pubkey and that we want it to be
        // passed as writable in the callback since we plan to edit it.
        vec![CallbackAccount{
            pubkey: addition_result_pda,
            is_writable: true,
        }],
        // Callback server address
        // None here because the output of the confidential instruction can fit into a solana transaction
        // as its just 1 Ciphertext which is 32 bytes
        None
    )?;
    Ok(())
}

/* The AddTogether accounts struct stays exactly the same */
```

Note here how we added the account we need in the callback Vec inside `queue_computation`, but since we didn't actually read or write to the account itself we don't need to pass it as part of the accounts struct. Let's take a look at how the callback instruction changes next:

```rust
// Macro provided by the Arcium Macros SDK to define a callback instruction.
#[arcium_callback(encrypted_ix = "add_together")]
pub fn add_together_callback(
    ctx: Context<AddTogetherCallback>,
    output: ComputationOutputs<AddTogetherOutput>,
) -> Result<()> {
    let o = match output {
        ComputationOutputs::Success(AddTogetherOutput { field_0 }) => field_0,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    emit!(SumEvent {
        sum: o.ciphertexts[0],
        nonce: o.nonce.to_le_bytes(),
    });

    // Save the result in our callback account too
    ctx.accounts.add_result_account.sum = o.ciphertexts[0];

    Ok(())
}


#[callback_accounts("add_together", payer)]
#[derive(Accounts)]
pub struct AddTogetherCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_ADD_TOGETHER)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    /// CHECK: instructions_sysvar, checked by the account constraint
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
    // Append the callback account(s) in the same order we provided them in the queue function
    // call
    #[account(
        mut,
        seeds = [b"AdditionResult"],
        // Note: In a real implementation you should usually save the bump too,
        // but for the sake of simplicity in this example we skip that
        bump
    )]
    pub add_result_account: Account<'info, SecretAdditionResult>,
}
```

What did we change? We appended the callback account we plan to receive to the end of the accounts struct and that's it, Arcium takes care of the rest.

# Callback Type Generation

## Quick Reference

| Return Type      | Generated Struct           | Access Pattern                          |
| ---------------- | -------------------------- | --------------------------------------- |
| `Enc<Shared, T>` | `SharedEncryptedStruct<1>` | `result.ciphertexts[0]`, `result.nonce` |
| `Enc<Mxe, T>`    | `MXEEncryptedStruct<1>`    | `result.ciphertexts[0]`, `result.nonce` |
| `(T, U, V)`      | `{Circuit}TupleStruct0`    | `result.field_0`, `result.field_1`      |
| Custom struct    | `{Circuit}OutputStruct0`   | `result.field_0`, `result.field_1`      |

**Callback pattern:**

```rust
#[arcium_callback(encrypted_ix = "your_function")]
pub fn callback(output: ComputationOutputs<YourFunctionOutput>) -> Result<()> {
    let result = match output {
        ComputationOutputs::Success(data) => data,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };
    // Access result.field_0, result.ciphertexts[0], etc.
}
```

## Overview

One of the most frustrating parts of working with encrypted computations used to be manually parsing raw bytes in your callback functions. You'd have to remember exactly where each piece of data was located, what size it was, and how to convert it back to the right type. Not fun, and definitely error-prone.

The good news? Arcium now handles all of this for you automatically. When you write an encrypted instruction, the macro system analyzes what your function returns and generates perfectly typed Rust structs that you can use directly in your callbacks.

## The Magic Behind the Scenes

Here's what happens when you define an encrypted instruction:

1. Arcium reads your circuit's output types
2. It generates corresponding Rust structs with predictable names
3. It automatically detects encryption patterns and creates specialized types
4. Everything gets integrated into your `#[arcium_callback]` functions

The best part? You never have to think about byte parsing again. Let's see how this works in practice.

## Basic Example: Simple Addition

Consider this encrypted instruction that adds two numbers:

```rust
#[encrypted]
mod circuits {
    use arcis_imports::*;

    #[instruction]
    pub fn add_together(input: Enc<Shared, (u8, u8)>) -> Enc<Shared, u16> {
        let (a, b) = input.to_arcis();
        let sum = a as u16 + b as u16;
        input.owner.from_arcis(sum)
    }
}
```

Behind the scenes, Arcium sees that your function returns `Enc<Shared, u16>` and automatically generates this output struct for you:

```rust
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddTogetherOutput {
    pub field_0: SharedEncryptedStruct<1>,
}
```

Notice how it detected that you're returning shared-encrypted data and created a `SharedEncryptedStruct<1>` (the `1` means there's one encrypted value). Now you can use this directly in your callback:

```rust
#[arcium_callback(encrypted_ix = "add_together")]
pub fn add_together_callback(
    ctx: Context<AddTogetherCallback>,
    output: ComputationOutputs<AddTogetherOutput>,
) -> Result<()> {
    let result = match output {
        ComputationOutputs::Success(AddTogetherOutput { field_0 }) => field_0,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    emit!(SumEvent {
        sum: result.ciphertexts[0],
        nonce: result.nonce.to_le_bytes(),
    });
    Ok(())
}
```

## How the Naming Works

You might be wondering: "How do I know what the generated struct will be called?" Great question! The naming follows predictable patterns that make sense once you see them:

### Your Circuit Gets an Output Struct

If your encrypted instruction is called `add_together`, you get a struct called `AddTogetherOutput`. Simple! Arcium just takes your circuit name and converts it to PascalCase, then adds "Output" at the end.

### Fields Are Numbered

Since Anchor doesn't support tuple structs (yet), Arcium uses numbered fields instead. So if your function returns multiple values, you'll get `field_0`, `field_1`, `field_2`, and so on. Not the prettiest names, but they're consistent and predictable.

### Complex Types Get Their Own Structs

When your function returns complex nested data (like tuples or custom structs), Arcium generates additional helper structs:

* Regular structs become `{CircuitName}OutputStruct{number}`
* Tuples become `{CircuitName}TupleStruct{number}`
* Nested stuff gets even longer names to avoid conflicts

## The Really Cool Part: Automatic Encryption Detection

Here's where things get really neat. Arcium doesn't just generate basic structs - it's smart enough to recognize when you're working with encrypted data and creates specialized types that make your life easier.

### SharedEncryptedStruct\<N>

When your circuit returns `Enc<Shared, T>`, Arcium knows this is data that both the client and the MXE can decrypt. It generates a struct that includes everything needed for decryption:

```rust
pub struct SharedEncryptedStruct<const LEN: usize> {
    pub encryption_key: [u8; 32],    // The shared public key
    pub nonce: u128,                 // Random nonce for security
    pub ciphertexts: [[u8; 32]; LEN], // Your actual encrypted data
}
```

The `<N>` part tells you how many encrypted values are packed inside. So `SharedEncryptedStruct<1>` has one encrypted value, `SharedEncryptedStruct<3>` has three, and so on.

In your callback, you can access everything you need:

```rust
let shared_key = result.encryption_key;  // For key exchange
let nonce = result.nonce;               // For decryption
let encrypted_value = result.ciphertexts[0]; // Your data
```

### MXEEncryptedStruct\<N>

For `Enc<Mxe, T>` data, only the MXE cluster can decrypt it - clients can't. Since there's no shared secret needed, the struct is simpler:

```rust
pub struct MXEEncryptedStruct<const LEN: usize> {
    pub nonce: u128,                 // Still need the nonce
    pub ciphertexts: [[u8; 32]; LEN], // Your encrypted data
}
```

Notice there's no `encryption_key` field here - that's because clients don't get to decrypt MXE data.

```rust
// Working with MXE-encrypted data
let nonce = result.nonce;
let encrypted_value = result.ciphertexts[0];
// Note: You can't decrypt this on the client side!
```

### EncDataStruct\<N>

For simple encrypted data without key exchange metadata:

```rust
// Pattern: Only N Ciphertexts
pub struct EncDataStruct<const LEN: usize> {
    pub ciphertexts: [[u8; 32]; LEN], // Raw encrypted values
}
```

## Let's See This in Action

Nothing beats real examples! Let's look at how this type generation works in actual Arcium applications that people are building:

### Voting Application

The [confidential voting example](https://github.com/arcium-hq/examples/tree/main/voting) shows a perfect use case. You have poll data that only the MXE should see, and a user's vote that should be shared between the user and the MXE:

```rust
#[instruction]
pub fn vote(
    poll_data: Enc<Mxe, &PollData>,     // Poll results stay private
    vote_choice: Enc<Shared, u8>        // User can verify their vote
) -> (Enc<Mxe, PollData>, Enc<Shared, bool>) {
    // ... voting logic that maintains privacy
}
```

Since this function returns a tuple `(Enc<Mxe, PollData>, Enc<Shared, bool>)`, Arcium generates:

```rust
pub struct VoteOutput {
    pub field_0: VoteTupleStruct0,  // The whole tuple wraps into one field
}

pub struct VoteTupleStruct0 {
    pub field_0: MXEEncryptedStruct<N>,    // The updated poll data
    pub field_1: SharedEncryptedStruct<1>, // The vote confirmation
}
```

Now in your callback, you can work with properly typed data instead of raw bytes:

```rust
#[arcium_callback(encrypted_ix = "vote")]
pub fn vote_callback(
    ctx: Context<VoteCallback>,
    output: ComputationOutputs<VoteOutput>,
) -> Result<()> {
    let VoteOutput { field_0 } = match output {
        ComputationOutputs::Success(result) => result,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    let poll_data = field_0.field_0;         // The updated poll (MXE only)
    let vote_confirmation = field_0.field_1; // User's confirmation (shared)

    // Emit an event with the user's confirmation
    emit!(VoteEvent {
        confirmation: vote_confirmation.ciphertexts[0],
        nonce: vote_confirmation.nonce.to_le_bytes(),
    });
    Ok(())
}
```

### Coinflip Application

The [coinflip example](https://github.com/arcium-hq/examples/tree/main/coinflip) is beautifully simple - just a function that returns a random boolean:

```rust
#[instruction]
pub fn flip() -> Enc<Shared, bool> {
    // Generate secure randomness in MPC
    // Return encrypted result that client can decrypt
}
```

Arcium sees this returns `Enc<Shared, bool>` and creates:

```rust
pub struct FlipOutput {
    pub field_0: SharedEncryptedStruct<1>, // Just one boolean
}
```

Your callback becomes super clean:

```rust
#[arcium_callback(encrypted_ix = "flip")]
pub fn flip_callback(
    ctx: Context<FlipCallback>,
    output: ComputationOutputs<FlipOutput>,
) -> Result<()> {
    let result = match output {
        ComputationOutputs::Success(FlipOutput { field_0 }) => field_0,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    // Emit the encrypted result - client will decrypt to see heads/tails
    emit!(FlipEvent {
        result: result.ciphertexts[0],
        nonce: result.nonce.to_le_bytes(),
    });
    Ok(())
}
```

### Blackjack Application

From the [blackjack example](https://github.com/arcium-hq/examples/tree/main/blackjack) with complex game state:

```rust
#[instruction]
pub fn player_hit(
    game_state: Enc<Mxe, &GameState>,
    player_hand: Enc<Shared, PlayerHand>
) -> (Enc<Mxe, GameState>, Enc<Shared, PlayerHand>, Enc<Shared, bool>) {
    // ... game logic
}
```

**Generated types**:

```rust
pub struct PlayerHitOutput {
    pub field_0: PlayerHitTupleStruct0,
}

pub struct PlayerHitTupleStruct0 {
    pub field_0: MXEEncryptedStruct<N>,    // Updated game state
    pub field_1: SharedEncryptedStruct<M>, // Player's new hand
    pub field_2: SharedEncryptedStruct<1>, // Is game over?
}
```

## Complex Nested Structures

For more complex outputs with nested data structures:

```rust
pub struct UserData {
    id: u32,
    active: bool,
}

#[instruction]
pub fn complex_example() -> (
    UserData,
    Enc<Shared, u32>,
    (u64, f32),
    Enc<Mxe, bool>
) {
    // ... complex logic
}
```

**Generated types**:

```rust
pub struct ComplexExampleOutput {
    pub field_0: ComplexExampleTupleStruct0, // Entire tuple as single field
}

pub struct ComplexExampleTupleStruct0 {
    pub field_0: ComplexExampleTupleStruct0OutputStruct0,  // UserData
    pub field_1: SharedEncryptedStruct<1>,                 // Enc<Shared, u32>
    pub field_2: ComplexExampleTupleStruct0TupleStruct02,  // (u64, f32) tuple
    pub field_3: MXEEncryptedStruct<1>,                    // Enc<Mxe, bool>
}

pub struct ComplexExampleTupleStruct0OutputStruct0 {
    pub field_0: u32,   // UserData.id
    pub field_1: bool,  // UserData.active
}

pub struct ComplexExampleTupleStruct0TupleStruct02 {
    pub field_0: u64,   // First tuple element
    pub field_1: f32,   // Second tuple element
}
```

## Working with Generated Types

### Pattern Matching

Use destructuring to access nested data:

```rust
let ComplexExampleOutput {
    field_0: ComplexExampleTupleStruct0 {
        field_0: user_data,
        field_1: shared_encrypted,
        field_2: tuple_data,
        field_3: mxe_encrypted,
    }
} = match output {
    ComputationOutputs::Success(result) => result,
    _ => return Err(ErrorCode::AbortedComputation.into()),
};

// Access specific fields
let user_id = user_data.field_0;
let is_active = user_data.field_1;
let shared_value = shared_encrypted.ciphertexts[0];
let timestamp = tuple_data.field_0;
```

### Error Handling

Always handle computation failures:

```rust
let result = match output {
    ComputationOutputs::Success(data) => data,
    ComputationOutputs::Failure(error) => {
        msg!("Computation failed: {:?}", error);
        return Err(ErrorCode::AbortedComputation.into());
    }
    ComputationOutputs::Timeout => {
        msg!("Computation timed out");
        return Err(ErrorCode::ComputationTimeout.into());
    }
};
```

## Best Practices

### 1. Use Descriptive Variable Names

```rust
// Good
let FlipOutput { field_0: coin_result } = result;
let is_heads = coin_result.ciphertexts[0];

// Less clear
let FlipOutput { field_0 } = result;
let result = field_0.ciphertexts[0];
```

### 2. Document Your Circuit Interfaces

```rust
/// Returns (updated_game_state, player_hand, is_game_over)
#[instruction]
pub fn player_hit(/* ... */) -> (Enc<Mxe, GameState>, Enc<Shared, PlayerHand>, Enc<Shared, bool>) {
    // ...
}
```

### 3. Handle All Computation States

```rust
let result = match output {
    ComputationOutputs::Success(data) => data,
    ComputationOutputs::Failure(_) => return Err(ErrorCode::AbortedComputation.into()),
    ComputationOutputs::Timeout => return Err(ErrorCode::ComputationTimeout.into()),
};
```

### 4. Emit Events for Client Tracking

```rust
emit!(ComputationCompleteEvent {
    computation_id: ctx.accounts.computation_account.key(),
    success: true,
    result_hash: hash(&result.ciphertexts[0]),
});
```

## When Things Go Wrong

Don't worry, we've all been there! Here are the most common issues you'll run into and how to fix them:

### "Type not found" Errors

```rust
// Error: cannot find type `MyCircuitOutput` in this scope
output: ComputationOutputs<MyCircuitOutput>
```

This usually means one of two things:

1. **Typo in the circuit name** - Check that `MyCircuit` exactly matches your `#[instruction]` function name (case matters!)
2. **You forgot to rebuild** - Run `arcium build` again after making changes to your encrypted instructions

### "No field found" Errors

```rust
// Error: no field `result` on type `AddTogetherOutput`
let value = output.result;
```

Remember, the generated structs use numbered fields like `field_0`, `field_1`, etc. There's no field called `result` unless you specifically named your function that way.

Try this instead:

```rust
let value = output.field_0;  // First (and often only) field
```

### Encryption Type Mismatches

```rust
// Error: expected `SharedEncryptedStruct<1>`, found `MXEEncryptedStruct<1>`
```

This happens when your circuit returns `Enc<Mxe, T>` but your callback expects `Enc<Shared, T>` (or vice versa). Double-check your encrypted instruction's return type - it needs to match what you're expecting in the callback.

### Debugging Generated Types

To see what types are generated for your circuit, check the build output or use [`cargo expand`](https://github.com/dtolnay/cargo-expand) in your program directory:

```bash
# First install cargo-expand if you haven't already
cargo install cargo-expand

# Then use it to see the generated code
cd programs/your-program
cargo expand > expanded.rs
# Search for your circuit name in expanded.rs
```

## Migration from v0.1.x

If you're upgrading from an older version, the new type generation system replaces manual byte parsing:

**Old way (v0.1.x)**:

```rust
pub fn callback(output: ComputationOutputs) -> Result<()> {
    let bytes = if let ComputationOutputs::Bytes(bytes) = output {
        bytes
    } else {
        return Err(ErrorCode::AbortedComputation.into());
    };

    let sum = bytes[48..80].try_into().unwrap();
    let nonce = bytes[32..48].try_into().unwrap();
    // ...
}
```

**New way (v0.2.0+)**:

```rust
pub fn callback(output: ComputationOutputs<AddTogetherOutput>) -> Result<()> {
    let AddTogetherOutput { field_0 } = match output {
        ComputationOutputs::Success(result) => result,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    let sum = field_0.ciphertexts[0];
    let nonce = field_0.nonce;
    // ...
}
```

For detailed migration steps, see the [Migration Guide](../../migration#6-update-your-callback-functions).

***

And that's it! The callback type generation system takes all the tedious work out of handling encrypted computation results. No more manual byte parsing, no more wondering if you got the offsets right, and no more runtime surprises when your data doesn't match what you expected.

With these automatically generated types, you can focus on building amazing privacy-preserving applications instead of wrestling with low-level data handling. Pretty neat, right?

# JavaScript Client

## Overview

Arcium offers two TS libraries, which provide tools and utilities for interacting with Arcium and the MXEs (MPC eXecution Environments) deployed on it.

Client library `@arcium-hq/client`:

* Handle secret sharing and encryption of inputs
* Submit confidential transactions
* Manage callbacks for computation results

Reader library `@arcium-hq/reader`:

* Read MXE data
* View computations for a given MXE

Generally speaking, the client library is used to build & invoke computations on MXEs and then track their outputs, while the reader library is more so to track the overall network. To get a better idea of its place in the general architecture, we highly recommend taking a look at the [computation lifecycle](computation-lifecycle).

## Installation

Client library:



{% tabs %}
{% tab title="npm" %}
```bash
npm install @arcium-hq/client
```
{% endtab %}

{% tab title="yarn" %}
```bash
yarn add @arcium-hq/client
```
{% endtab %}

{% tab title="pnpm" %}
```bash
pnpm add @arcium-hq/client
```
{% endtab %}
{% endtabs %}

Reader library:

{% tabs %}
{% tab title="npm" %}
```bash
npm install @arcium-hq/reader
```
{% endtab %}

{% tab title="yarn" %}
```bash
yarn add @arcium-hq/reader
```
{% endtab %}

{% tab title="pnpm" %}
```bash
pnpm add @arcium-hq/reader
```
{% endtab %}
{% endtabs %}

## API Reference

For complete TypeScript SDK documentation and API reference for the client and reader libraries, visit: [ts.arcium.com/api](https://ts.arcium.com/api)

## Using the client

Prefer a more step-by-step approach? Get started with learning [how to encrypt inputs for confidential transactions](js-client-library/encrypting).

# Encrypting inputs

Let's say we have the following confidential instruction that adds 2 encrypted `u8`s and returns the result as in plaintext:

```rust
use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct InputValues {
        v1: u8,
        v2: u8,
    }

    #[instruction]
    pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> u16 {
        let input = input_ctxt.to_arcis();
        let sum = input.v1 as u16 + input.v2 as u16;
        sum.reveal()
    }
}
```

We want to input the values `x = 42` and `y = 101` into this instruction. To do this, we first have to build the parameters for the confidential instruction correctly:

```ts
import { RescueCipher, getArciumEnv, x25519 } from "@arcium-hq/client";
import { randomBytes } from "crypto";

// Our confidential instruction takes two encrypted `u8` values as input, so we need to provide two ciphertext values which are represented as `[u8; 32]` in our Solana program.
const val1 = BigInt(1);
const val2 = BigInt(2);
const plaintext = [val1, val2];
```

Now that we have the inputs, we need to encrypt them. This is done using the `RescueCipher` class with some info about the MPC cluster we want to use:

```ts
// Fetch the MXE x25519 public key
const mxePublicKey = await getMXEPublicKeyWithRetry(
  provider as anchor.AnchorProvider,
  program.programId
);
// Generate a random private key for x25519 elliptic curve Diffie-Hellman key exchange.
const privateKey = x25519.utils.randomPrivateKey();
// Derive the public key from the private key.
const publicKey = x25519.getPublicKey(privateKey);
// Generate a random nonce for the encryption.
const nonce = randomBytes(16);
// Get the shared secret with the cluster.
const sharedSecret = x25519.getSharedSecret(privateKey, mxePublicKey);
// Initialize the cipher with the shared secret.
const cipher = new RescueCipher(sharedSecret);
// Encrypt the plaintext, and serialize it to a `[u8; 32]` array.
const ciphertext = cipher.encrypt(plaintext, nonce);
```

To decrypt the data, again it follows a similar pattern:

```ts
// Initialize the cipher with the shared secret.
const cipher = new RescueCipher(sharedSecret);
const plaintext = cipher.decrypt(ciphertext, nonce);
```

# Tracking callbacks

Unlike regular transactions, confidential computations involve additional steps after your Solana transaction completes:

1. **Your transaction completes** - Encrypted data is submitted and queued in the MXE's mempool
2. **Computation waits in queue** - MPC nodes process computations from the mempool in order
3. **MPC execution** - When your computation's turn comes, MPC nodes execute it off-chain
4. **Callback invocation** - Results are returned via your callback instruction

This means you can't simply await a transaction completion like normal Solana programs. Instead, you need to wait for the entire computation lifecycle to finish. The Arcium client library provides utilities to handle this:

## Await computation completion with `awaitComputationFinalization`

```ts
// Generate a random 8-byte computation offset
const computationOffset = new anchor.BN(randomBytes(8), "hex");

// `program` is the anchor program client of the MXE we're invoking
// the instruction `ourIx` on (which then invokes a computation under the hood by CPIing into the Arcium program).
// `queueSig` is the signature of said transaction.
const queueSig = await program.methods
  .ourIx(
    // Computation offset that you provide when invoking the instruction
    computationOffset
    /* other inputs */
  )
  .accounts(/* some accounts */)
  .rpc();

// Since this is a Arcium computation, we need to wait for it to be finalized
// a little bit differently
const finalizeSig = await awaitComputationFinalization(
  // Connection to the chain
  provider.connection,
  // Computation offset that you provide when invoking the instruction
  computationOffset,
  // Program ID of the MXE
  program.programId,
  // Solana commitment level, "confirmed" by default
  "confirmed"
);

console.log("Computation was finalized with sig: ", finalizeSig);
```

# Deployment

## Getting Started with Deployment

So you've built and tested your MXE locally, and now you're ready to deploy it to Solana devnet. This guide will walk you through the deployment process and share some tips to make it go smoothly.

## What You'll Need

Before we dive into deployment, let's make sure you have everything ready:

* Your MXE built successfully with `arcium build`
* Tests passing locally with `arcium test`
* A Solana keypair with around 2-5 SOL for deployment costs (program deployment and account initialization)
* Access to a reliable RPC endpoint - we strongly recommend getting a free API key from [Helius](https://helius.dev), or [QuickNode](https://quicknode.com)

## Preparing Your Program

Before you deploy, there are a couple of important things to consider about how your program handles computation definitions.

### Handling Large Circuits with Offchain Storage

Here's something important to know: right now, Arcis compiled circuits aren't super efficient with their encoding, which means your circuit files can easily be several MBs in size. That makes initializing computation definitions on-chain pretty expensive - and will require a lot of transactions to fully upload.

The good news is you can store your circuits offchain instead. Just upload them to IPFS, a public S3 bucket, or even Supabase object storage - wherever works for you. Here's how to update your program to use offchain storage:

**Standard approach (works for small circuits):**

```rust
pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
    // This initializes the computation definition account
    init_comp_def(ctx.accounts, true, 0, None, None)?;
    Ok(())
}
```

**Offchain approach (recommended for larger circuits):**

```rust
// First, import the types you'll need
use arcium_client::idl::arcium::types::{CircuitSource, OffChainCircuitSource};

pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
    // Point to your uploaded circuit file
    init_comp_def(
        ctx.accounts,
        true,
        0,
        Some(CircuitSource::OffChain(OffChainCircuitSource {
            source: "https://your-storage.com/path/to/add_together_testnet.arcis".to_string(),
            hash: [0; 32], // Just use zeros for now - hash verification isn't enforced yet
        })),
        None,
    )?;
    Ok(())
}
```

With the offchain approach, you'll:

1. Build your project with `arcium build` to generate the circuit files
2. Upload the files from `build/` folder to your preferred storage service (files include network suffix, e.g., `add_together_testnet.arcis` for testnet)
3. Update your init functions in the Solana program with the public URLs
4. Run `arcium build` again to rebuild the Solana program with your changes

Note: Your circuit files must be publicly accessible without authentication. Make sure your storage service allows public read access.

This saves a ton on transaction costs and lets you work with much larger circuits!

### Note on Cluster Configuration

When testing locally, you've been using `arciumEnv.arciumClusterPubkey` in your test code. After deployment to devnet, you'll need to update this to use the actual cluster pubkey - we'll show you exactly how in the post-deployment section.

## Basic Deployment

The `arcium deploy` command handles both deploying your program and initializing the MXE account. Here's the basic command structure:

```bash
arcium deploy --cluster-offset <cluster-offset> --keypair-path <path-to-your-keypair> --rpc-url <your-rpc-url>
```

Let's break down what each parameter does:

### Understanding Cluster Offsets

The `--cluster-offset` tells your MXE which Arcium cluster it should connect to. Think of clusters as groups of nodes that will perform your encrypted computations. For devnet, you can choose from these offsets:

* `1116522165`
* `3458519414`
* `768109697`

Each represents a different cluster on devnet. They all work the same way, so just pick one for your deployment.

### Choosing Your RPC Provider

The `--rpc-url` parameter is particularly important. While you could use Solana's default RPC endpoints with the shorthand notation (`-u d` for devnet), the default RPC can be unreliable and cause deployment failures due to dropped transactions.

**Recommended approach with a reliable RPC:**

```bash
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://devnet.helius-rpc.com/?api-key=<your-api-key>
```

**If you must use the default RPC:**

```bash
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  -u d  # 'd' for devnet, 't' for testnet, 'l' for localnet
```

Just be prepared for potential transaction failures with the default RPC.

## Advanced Deployment Options

Once you're comfortable with basic deployment, you might want to customize things further.

### Adjusting Mempool Size

The mempool determines how many computations your MXE can queue up. The default "Tiny" size works fine for testing, but you might want more capacity for production:

```bash
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url <your-rpc-url> \
  --mempool-size Medium
```

Available sizes are: `Tiny`, `Small`, `Medium`, `Large`. Start small and increase if you need more capacity.

### Using a Custom Program Address

If you need your program at a specific address (maybe for consistency across deployments), you can provide a program keypair:

```bash
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url <your-rpc-url> \
  --program-keypair ./program-keypair.json
```

### Partial Deployments

Sometimes you might need to run just part of the deployment process. For instance, if you've already deployed the program but need to reinitialize the MXE account:

```bash
# Skip program deployment, only initialize MXE account
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url <your-rpc-url> \
  --skip-deploy
```

Or if you only want to deploy the program without initialization:

```bash
# Deploy program only, skip MXE initialization
arcium deploy --cluster-offset 1116522165 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url <your-rpc-url> \
  --skip-init
```

## After Deployment

### Initialize Your Computation Definitions

Your MXE is deployed, but you still need to initialize the computation definitions. This tells the Arcium network what encrypted operations your MXE can perform. Computation definitions only need to be initialized once - they persist on-chain and don't need to be re-initialized unless you're deploying to a new program address. You can initialize them anytime after deployment completes successfully.

Remember how we mentioned you'd need to update your cluster configuration? Now's the time! You'll need to update your test or client code to use the actual cluster pubkey instead of the local testing environment.

**Replace this (local testing):**

```typescript
const arciumEnv = getArciumEnv();

// Later in your transaction...
.accountsPartial({
    clusterAccount: arciumEnv.arciumClusterPubkey,
    // ... other accounts
})
```

**With this (for devnet):**

```typescript
// Use the cluster offset from your deployment
const clusterAccount = getClusterAcc(cluster_offset); // e.g., getClusterAcc(1116522165)

// In your transaction...
.accountsPartial({
    clusterAccount: clusterAccount,
    // ... other accounts
})
```

Make sure to use the same `cluster_offset` value that you used during deployment! This ensures your program talks to the right cluster.

Once you've updated the cluster configuration, you can run the initialization:

```typescript
// Now with the correct cluster configured
await initAddTogetherCompDef(program, owner, false);
```

### Verify Everything's Working

Let's make sure your deployment succeeded:

```bash
solana program show <your-program-id> --url <your-rpc-url>
```

Then run your tests against the deployed program:

```bash
arcium test --skip-local-validator --provider.cluster devnet
```

Note: This will use the RPC endpoint configured for devnet in your Anchor.toml file or environment settings.

## Common Issues and Solutions

### Dealing with Dropped Transactions

If your deployment fails with transaction errors, it's almost always the RPC. Switch to a dedicated provider:

```bash
# Instead of this (unreliable):
arcium deploy ... -u d

# Use this (reliable):
arcium deploy ... --rpc-url https://devnet.helius-rpc.com/?api-key=<your-key>
```

### Running Out of SOL

Check your balance before deploying:

```bash
solana balance <your-keypair-pubkey> --url devnet
```

Need more devnet SOL? Request an airdrop:

```bash
solana airdrop 2 <your-keypair-pubkey> --url devnet
```

### Deployment Partially Failed?

No worries, you can complete the missing steps. If the program deployed but initialization failed, just run with `--skip-deploy`. If initialization succeeded but deployment failed, use `--skip-init`.

## What's Next?

With your MXE deployed, you're ready to:

1. Update your client code to connect to the deployed program
2. Initialize all your computation definitions
3. Run end-to-end tests with real encrypted computations
4. Monitor performance and adjust mempool size if needed

If you run into any issues or have questions, don't hesitate to reach out on [Discord](https://discord.gg/arcium)!

# Callback Server

When an encrypted instruction produces output that's too large to fit in a single Solana transaction (which has a size limit), you'll need to implement a callback server. This is a simple HTTP server that you develop and host yourself, which acts as an intermediary to receive large computation results from the MPC nodes and process them according to your application's needs.

For example, if your encrypted instruction produces a large output (say 10KB), the MPC nodes will first pack as much data as possible into the normal callback transaction (\~1KB), and send the remaining data (in this case \~9KB) to your callback server. This allows you to handle arbitrarily large outputs while still maintaining the efficiency of direct onchain callbacks when possible.

The callback server provides a simple HTTP endpoint that receives the computation output, verifies its authenticity using signatures from the MPC nodes, and processes the data according to your needs. This allows you to handle arbitrarily large computation results while maintaining the security guarantees of the Arcium protocol. Onchain, the callback server must also call the `finalize` transaction for the computation, where the Arcium program verifies that the data submitted by the callback server matches the data computed by the MPC nodes by comparing their hashes.

## API Interface

### POST /callback

Receives a raw byte object with the following structure:

`mempool_id|comp_def_offset|tx_sig|data_sig|pub_key|data`

* `mempool_id`: u16 - Mempool identifier
* `comp_def_offset`: u32 - Identifier for the given computation definition in the MXE program
* `tx_sig`: \[u8; 64] - The transaction signature of the callback transaction
* `data_sig`: \[u8; 64] - The signature of the data, signed by one of the node's private keys
* `pub_key`: \[u8; 32] - The public key of the node that signed the data
* `data`: Vec - The actual computation output to be processed

The server will then verify the signatures, and if they are valid, it will process the data.

The most common use case is to perform any necessary processing and submit the data back to the chain.

The server will then return a 200 OK response.

# Current Limitations

## Output sizes

Outputs of encrypted instructions should be able to fit in a single solana transaction (the callback transaction), otherwise you need to setup additional infrastructure to handle transaction data handling via [callback server](callback-server).

# Migrate from v0.1.x to v0.2.x

## 1. Update Arcium Rust dependencies

```bash
# Update program dependencies
cd programs/*
cargo update --package arcium-client --precise 0.2.0
cargo update --package arcium-macros --precise 0.2.0
cargo update --package arcium-anchor --precise 0.2.0

# Update encrypted-ixs dependencies
cd ../../encrypted-ixs
cargo update --package arcis-imports --precise 0.2.0
```

## 2. Update Arcium TS dependencies

{% tabs %}
{% tab title="npm" %}
```bash
npm install @arcium-hq/client@0.2.0
```
{% endtab %}

{% tab title="yarn" %}
```bash
yarn add @arcium-hq/client@0.2.0
```
{% endtab %}

{% tab title="pnpm" %}
```bash
pnpm add @arcium-hq/client@0.2.0
```
{% endtab %}
{% endtabs %}

## 3. (Optional) Update your `Arcium.toml`

We no longer need the `clusters` field under `localnet` in `Arcium.toml`. New file `Arcium.toml` should look like this:

```toml
[localnet]
# number of nodes in the single cluster of the localnet
nodes = 2
# number of seconds to wait for the localnet to come online
localnet_timeout_secs = 60
```

This change is optional, and leaving `clusters` field as is will still work.

## 4. Simplify your Arcium crate imports

No more 10 line import statements from `arcium_anchor`, `arcium_client`, `arcium_macros`, etc. All of them can be compressed such that the following:

```rust
use arcium_anchor::{
    comp_def_offset, derive_cluster_pda, derive_comp_def_pda, derive_comp_pda, derive_execpool_pda,
    derive_mempool_pda, derive_mxe_pda, init_comp_def, queue_computation, ComputationOutputs,
    ARCIUM_CLOCK_ACCOUNT_ADDRESS, ARCIUM_STAKING_POOL_ACCOUNT_ADDRESS, CLUSTER_PDA_SEED,
    COMP_DEF_PDA_SEED, COMP_PDA_SEED, EXECPOOL_PDA_SEED, MEMPOOL_PDA_SEED, MXE_PDA_SEED,
};
use arcium_client::idl::arcium::{
    accounts::{
        ClockAccount, Cluster, ComputationDefinitionAccount, PersistentMXEAccount,
        StakingPoolAccount,
    },
    program::Arcium,
    types::Argument,
    ID_CONST as ARCIUM_PROG_ID,
};
use arcium_macros::{
    arcium_callback, arcium_program, callback_accounts, init_computation_definition_accounts,
    queue_computation_accounts,
};
```

now becomes just

```rust
use arcium_anchor::prelude::*;
```

This includes all the basic imports required to setup and run the basic Arcium example program. For additional types such as `CircuitSource`, `CallbackAccount`, they will still need to be imported separately from `arcium_client::idl::arcium::types` module.

## 5. Update your Arcium CPI calls

In your `init_comp_def` calls, you need to add an additional parameter at the third position. So, it used to look like:

```rust
init_comp_def(ctx.accounts, true, None, None)?;
```

Now it should look like:

```rust
init_comp_def(ctx.accounts, true, 0, None, None)?;
```

You don't need to care about the `0` parameter, it's just a placeholder for the new parameter.

## 6. Update your callback functions

No more manually handling all output bytes from an Arcium computation. The new `#[arcium_callback]` macro will handle all the deserialization of bytes for you based on your circuit's generated interface file.

```rust
#[arcium_callback(encrypted_ix = "add_together")]
pub fn add_together_callback(
    ctx: Context<AddTogetherCallback>,
    output: ComputationOutputs,
) -> Result<()> {
    let bytes = if let ComputationOutputs::Bytes(bytes) = output {
        bytes
    } else {
        return Err(ErrorCode::AbortedComputation.into());
    };

    emit!(SumEvent {
        sum: bytes[48..80].try_into().unwrap(),
        nonce: bytes[32..48].try_into().unwrap(),
    });
    Ok(())
}
```

becomes

```rust
#[arcium_callback(encrypted_ix = "add_together")]
pub fn add_together_callback(
    ctx: Context<AddTogetherCallback>,
    output: ComputationOutputs<AddTogetherOutput>,
) -> Result<()> {
    let o = match output {
        ComputationOutputs::Success(AddTogetherOutput { field_0 }) => field_0,
        _ => return Err(ErrorCode::AbortedComputation.into()),
    };

    emit!(SumEvent {
        sum: o.ciphertexts[0],
        nonce: o.nonce.to_le_bytes(),
    });
    Ok(())
}
```

For a comprehensive guide on how the callback type generation system works, see [Callback Type Generation](program/callback-type-generation).

## 7. Update your Context structs

First, all references to `PersistentMXEAccount` becomes just `MXEAccount`. This has to be done in both `queue_computation_accounts` and `init_computation_definition_accounts` context structs.

Second, we need to update the `StakingPoolAccount` to `FeePool` in your `queue_computation_accounts` Context structs.

```rust
#[account(
    mut,
    address = ARCIUM_STAKING_POOL_ACCOUNT_ADDRESS,
)]
pub pool_account: Account<'info, StakingPoolAccount>,
```

now becomes

```rust
#[account(
    mut,
    address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
)]
pub pool_account: Account<'info, FeePool>,
```

### 8. Add a new error code

In your program `ErrorCode` enum, add a new error code:

```rust
#[error_code]
pub enum ErrorCode {
    ...
    #[msg("The cluster is not set")]
    ClusterNotSet,
}
```

### 9. Replace static `mxePublicKey`

No more constant MXE public key definition in your tests or client. Instead you can import `getMXEPublicKey` function from `@arcium-hq/client` and use it to get the MXE public key.

```ts
const mxePublicKey = await getMXEPublicKey(
  provider as anchor.AnchorProvider,
  program.programId
);
```

This might cause some issues in your tests as the function might be called before the MXE keys are fully set. In which case, we recommend using the following helper function as a wrapper around `getMXEPublicKey` to fetch MXE public key with retries:

```ts
async function getMXEPublicKeyWithRetry(
  provider: anchor.AnchorProvider,
  programId: PublicKey,
  maxRetries: number = 10,
  retryDelayMs: number = 500
): Promise<Uint8Array> {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const mxePublicKey = await getMXEPublicKey(provider, programId);
      if (mxePublicKey) {
        return mxePublicKey;
      }
    } catch (error) {
      console.log(`Attempt ${attempt} failed to fetch MXE public key:`, error);
    }

    if (attempt < maxRetries) {
      console.log(
        `Retrying in ${retryDelayMs}ms... (attempt ${attempt}/${maxRetries})`
      );
      await new Promise((resolve) => setTimeout(resolve, retryDelayMs));
    }
  }

  throw new Error(
    `Failed to fetch MXE public key after ${maxRetries} attempts`
  );
}
```

And voila, you should have a working program that is compatible with Arcium tooling v0.2.0!
