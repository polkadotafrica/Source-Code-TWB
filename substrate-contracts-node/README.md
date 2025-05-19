# Running Ink! Smart Contract on Your Local System

This guide will walk you through setting up the required environment and running an Ink! smart contract on a local Substrate node. By the end, you'll have a running Ink! contract deployed on your own local blockchain.

## Prerequisites

Ensure you have the following installed on your system:
- **Basic knowledge of Rust programming**
- **Command-line experience**
- **Rust toolchain (Nightly channel preferred)**
- **Wasm target installed**
- **Node.js and npm** (for running Contracts UI locally)
- **Unix-based OS (Linux or macOS)** or **Windows with WSL**
- **Git** (for cloning repositories)

## Step-by-Step Guide

### Step 1: Install Rust Toolchain

1. Install Rust using the following command:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Set the Rust toolchain to stable, then update to the nightly version:

    ```bash
    rustup default stable
    rustup update nightly
    ```

3. Install the Wasm target for compiling to WebAssembly:

    ```bash
    rustup target add wasm32-unknown-unknown --toolchain nightly
    ```

### Step 2: Install Substrate & Cargo Contract

1. Install the `cargo-contract` CLI tool, which is used to compile, deploy, and interact with Ink! contracts:

    ```bash
    cargo install --force --locked cargo-contract --version 2.0.0
    ```

### Step 3: Run a Local Substrate Node

1. Clone the Substrate Contracts Node repository:

    ```bash
    git clone https://github.com/paritytech/substrate-contracts-node
    cd substrate-contracts-node
    ```

2. Build the node:

    ```bash
    cargo build --release
    ```

3. Run the node in development mode:

    ```bash
    ./target/release/substrate-contracts-node --dev
    ```

   This will give you a sandboxed blockchain running on `ws://127.0.0.1:9944`.

### Step 4: Scaffold Your First Ink! Smart Contract

1. Create a new Ink! contract project:

    ```bash
    cargo contract new hello_counter
    cd hello_counter
    ```

2. Replace the contents of `lib.rs` with the following code to define the smart contract:

    ```rust
    #![cfg_attr(not(feature = "std"), no_std, no_main)]

    #[ink::contract]
    mod hello_counter {
        #[ink(storage)]
        pub struct HelloCounter {
            value: u32,
        }

        impl HelloCounter {
            #[ink(constructor)]
            pub fn new(init_value: u32) -> Self {
                Self { value: init_value }
            }

            #[ink(message)]
            pub fn get(&self) -> u32 {
                self.value
            }

            #[ink(message)]
            pub fn increment(&mut self) {
                self.value += 1;
            }
        }
    }
    ```

### Step 5: Build and Test the Contract

1. Compile the contract to WebAssembly (Wasm):

    ```bash
    cargo +nightly contract build --release --target wasm
    ```

2. Run unit tests for the contract:

    ```bash
    cargo +nightly test
    ```

   You should see the output for the tests you wrote in `lib.rs`.

### Step 6: Deploy the Contract to Your Local Substrate Node

1. Open the Contracts UI at [https://contracts-ui.substrate.io](https://contracts-ui.substrate.io).

2. Connect to your local node using the Polkadot.js extension with the custom endpoint `ws://127.0.0.1:9944`.

3. Upload and instantiate the contract:
    - Click on **Upload a new Contract**.
    - Select the `.contract` file from the `./target/ink/` folder.
    - Provide an initial value (e.g., `10`).

4. Deploy the contract and interact with it:
    - Once deployed, you can interact with the contract using the **increment()** and **get()** functions via the Contracts UI.

### Step 7: Interact with the Contract

- You can call the contract methods directly from the UI:
    - **increment()**: Increments the counter value.
    - **get()**: Retrieves the current counter value.

### Troubleshooting

If you encounter any issues, here are some common solutions:

1. **Error: toolchain ‘nightly’ is not installed**
    - Solution: Install the nightly toolchain:

      ```bash
      rustup install nightly
      rustup default nightly
      rustup update nightly
      rustup target add wasm32-unknown-unknown --toolchain nightly
      ```

2. **cargo-contract: command not found**
    - Solution: Install the `cargo-contract` tool:

      ```bash
      cargo install --force --locked cargo-contract --version 2.0.0
      ```

3. **The Substrate node doesn’t start or hangs on startup**
    - Solution: Ensure you're using the correct build command:

      ```bash
      git clone https://github.com/paritytech/substrate-contracts-node
      cd substrate-contracts-node
      cargo build --release
      ./target/release/substrate-contracts-node --dev
      ```

4. **Contract upload fails on Contracts UI**
    - Solution: Ensure you're uploading the `.contract` file, not the `.wasm` or `.json` files.

5. **Tests don’t run/don't detect #[ink::test]**
    - Solution: Ensure you're running tests with the nightly toolchain:

      ```bash
      cargo +nightly test
      ```

    Also, ensure your test module is annotated with:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn example() { ... }
    }
    ```

## Conclusion

Ink! offers a powerful and efficient way to write smart contracts for Substrate-based blockchains using Rust. In this guide, you've set up a development environment, written your first contract, tested it, and deployed it to a local Substrate node. To continue learning, explore advanced use cases like token standards, DAOs, and cross-chain interactions.

Happy coding!
