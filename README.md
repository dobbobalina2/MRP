# Bonsai Pay Demo

This demo uses Google Sign-In to generate a client authentication token. The token includes a nonce that contains the user's connected wallet address, aligning with similar principles described in the [OpenPubkey: Augmenting OpenID Connect with User held Signing Keys](https://eprint.iacr.org/2023/296) paper. The JWT's integrity is verified within the guest [zkVM Program] using Google's public RS256 signing [certificates](https://www.googleapis.com/oauth2/v3/certs). The guest uses [Bonsai] to run the [RISC Zero zkVM], generating a cryptographic proof of the JWT's integrity, issuing a receipt that comprises the SNARK, an obfuscated identifier, and the user's address. The finalized proof is posted onchain and verified with the [RISC Zero Verifier] and used for arbitrary transactions, if valid.

> **Note: This software is not production ready. Do not use in production.**

This is based on the [Bonsai Foundry Template] for writing an application using [RISC Zero] and Ethereum.

This repository implements the application on Ethereum utilizing RISC Zero as a [coprocessor] to the smart contract application. Prove computation with the [RISC Zero zkVM] and verify the results in your Ethereum contract.

Check out the [developer FAQ] for more information on zkVM application design.

## Dependencies

First, [install Rust] and [Foundry], and then restart your terminal.

```sh
# Install Rust
curl https://sh.rustup.rs -sSf | sh
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
```

Next, you will need to install the `cargo risczero` tool.
We'll use [`cargo binstall`][cargo-binstall] to get `cargo-risczero` installed, and then install the `risc0` toolchain.
See [RISC Zero installation] for more details.

```sh
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

### Google Cloud Platform

This demo requires a Google Cloud Platform account. You will also need an account to generate a client ID to enable Sign-In-With-Google with OIDC via Google Cloud Identity Platform. You can find more information on how to set up Google Sign-In [here](https://developers.google.com/identity/sign-in/web/sign-in) and [here](https://developers.google.com/identity/protocols/oauth2/openid-connect).

### Etherscan API Key

You will need an Etherscan API key to verify the contract's source code. You can get one [here](https://etherscan.io/apis). This is not required, but is helpful for verifying the contract source code and generating the ABI bindings with [`wagmi`](https://wagmi.sh), which is used in the Bonsai Pay UI.

Now you have all the tools you need to develop and deploy an application with [RISC Zero].

## Quick Start

- Builds for zkVM program, the publisher app, and any other Rust code.

  ```sh
  cargo build
  ```

- Build your Solidity smart contracts

  > NOTE: `cargo build` needs to run first to generate the `ImageID.sol` contract.

  ```sh
  forge build
  ```

- Create a `.env` and update the necessary environment variables as shown in the [`.env.example`] file, for the UI.

  ```sh
  cp ui/.env.example ui/.env
  ```

### Run the Tests

- Tests your zkVM program.

  ```sh
  cargo test
  ```

- Test your Solidity contracts, integrated with your zkVM program.

  ```sh
  RISC0_DEV_MODE=true forge test -vvv
  ```

### Configuring Bonsai

**_Note:_** _To request an API key [complete the form here](https://bonsai.xyz/apply)._

With the Bonsai proving service, you can produce a [Groth16 SNARK proof] that is verifiable on-chain.
You can get started by setting the following environment variables with your API key and associated URL.

```bash
export BONSAI_API_KEY="" # see form linked above
export BONSAI_API_URL="" # provided with your api key
```

The first issue is that prove does not work  consistently as seen in AADemo. Sometimes it will work in dev_mode off but I can't get it to reproduce consistently.

The primary error I have is that for some reason my seal is returning false when I verify them in tests and in production on-chain.
The inputs in AaFork.sol are the inputs I'm using in production and the seal is a direct output from the bonsai proving service.

If you wish to verify the nonce in the jwt token copy and paste it into here https://jwt.io/


```sh
RISC0_DEV_MODE=true forge test -vvv --match-path tests/AADemo.t.sol
RISC0_DEV_MODE=false forge test -vvv --match-path tests/AADemo.t.sol


RISC0_DEV_MODE=false forge test -vvv --match-path tests/AaFork.t.sol --fork-url https://ethereum-holesky-rpc.publicnode.com
RISC0_DEV_MODE=true forge test -vvv --match-path tests/AaFork.t.sol --fork-url https://ethereum-holesky-rpc.publicnode.com



```
