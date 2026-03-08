# Vibe Hardware Wallet (Rust Core)

A secure, airgapped hardware wallet implementation in Rust, designed to run on old Android smartphones. This core supports multiple chains (EVM and Solana) through a modular, hexagonal architecture.

## 🚀 Features

- **Airgapped Design**: No network permissions required. All communication via QR codes (UR protocol).
- **Strict Hexagonal Architecture**: Complete decoupling of core logic from external dependencies using the **Ports and Adapters** pattern.
- **Flexible Mnemonics**: Supports both **12-word** and **24-word** BIP-39 mnemonics.
- **PIN/Passphrase Support**: Optional BIP-39 passphrase (PIN) support for enhanced security during seed derivation.
- **Responsive Terminal UI (TUI)**: 
  - A premium interface with **adaptive layout** for tiny screens.
  - Multi-chain QR code support for Seed, Ethereum, and Solana addresses.
  - Interactive generation, import, and address viewing.
- **Multi-Chain Support**:
  - **EVM**: Ethereum, Polygon, BSC, etc. (BIP-44 path `m/44'/60'/0'/0/0`).
  - **Solana**: Ed25519-based derivation (`m/44'/501'/0'/0/0`).
- **UniFFI Bridge**: Automatically generates Kotlin/Swift bindings for mobile integration.

---

## 🏗 Architecture

The project follows a strict **Hexagonal Architecture** (Ports and Adapters), ensuring that the core business logic remains agnostic of infrastructure details.

### 1. Domain Layer (`wallet-core/src/domain`)
Contains the core business entities and the **Ports** (traits) that define required functionality:
- **`Wallet`**: Core state containing the mnemonic.
- **`Chain`**: Supported blockchains (EVM, Solana).
- **`MnemonicProvider`**: Port for phrase generation and seed derivation.
- **`CryptoProvider`**: Port for address derivation and transaction signing.
- **`AirgapProvider`**: Port for high-density UR encoding (QR data).

### 2. Application Layer (`wallet-core/src/application`)
Coordinates the workflow between the domain and infrastructure:
- **`WalletService`**: Orchestrates mnemonic generation, account import, and signing. It interacts with adapters solely through their trait interfaces.

### 3. Infrastructure Layer (`wallet-core/src/infrastructure`)
Contains concrete **Adapters** for the defined Ports:
- **`Bip39Adapter`**: Uses `tiny-bip39` for mnemonics.
- **`CryptoAdapter`**: Specialized logic for Secp256k1 (EVM) and Ed25519 (Solana).
- **`UrAdapter`**: Implements UR (Uniform Resources) protocol for secure airgap communication.

### 4. Primary Adapters
- **`wallet-tui`**: A terminal-based user interface for interacting with the wallet core.
- **`wallet-ffi`**: A UniFFI-based bridge for Android/iOS mobile integration.

---

## 📂 Project Structure

```text
.
├── wallet-core         # Core Logic (The "Hexagon")
│   └── src
│       ├── domain      # Entities & Ports
│       ├── application # Services
│       └── infrastructure # Adapters
├── wallet-ffi          # UniFFI Bridge (FFI Adapter)
└── wallet-tui          # Terminal Interface (UI Adapter)
```

---

## 🛠 Prerequisites

### Rust Toolchain
Ensure you have the latest stable Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Android NDK (For Mobile Build)
To compile for Android, you'll need the NDK and `cargo-ndk`:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk
```

---

## 🏗 Building

### 1. Build the Rust Library
To build the library for your local machine (testing/development):
```bash
cargo build
```

### 2. Build for Android (EVM & Solana Core)
Use `cargo-ndk` to build the shared libraries (`.so`) for different Android architectures:
```bash
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi build --release
```
The output will be in `target/`.

### 3. Run the Terminal UI (Rust Only)
You can interact with the wallet core directly from your terminal:
```bash
cargo run -p wallet-tui
```
**TUI Controls:**
- **`Q`**: Quit application.
- **`UP/DOWN`**: Navigate menus.
- **`ENTER`**: Select/Confirm.
- **`ESC`**: Go back to previous screen.
- **`V`**: View **Seed Phrase** as QR code (in Wallet View).
- **`E`**: View **Ethereum Address** as QR code (in Wallet View).
- **`S`**: View **Solana Address** as QR code (in Wallet View).

### 4. Generate Mobile Bindings (UniFFI)
To generate the Kotlin code that connects your Android app to the Rust core:
```bash
cd wallet-ffi
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/lib.rs --language kotlin --out-dir ../../out/
```

---

## 🧪 Testing

Run the automated test suite to verify mnemonic generation and address derivation for both EVM and Solana:
```bash
cargo test -p wallet-core
```

---

## 📱 Mobile Integration Example

Once the `.so` files and Kotlin bindings are generated, you can use them in your Android project:

```kotlin
// Initialize a new 12-word wallet (or 24)
val wallet = Wallet.newRandom(12) 
println("Your Mnemonic: ${wallet.getMnemonic()}")

// Project different chain addresses (optional PIN)
val ethAddress = wallet.deriveAddress(Chain.Evm, "1234") // or null if no PIN
println("Ethereum: $ethAddress")

val solAddress = wallet.deriveAddress(Chain.Solana, null)
println("Solana: $solAddress")

// Sign an EVM Transaction Hash (optional PIN)
val signature = wallet.signEvmHash(txHashBytes, "1234")
```

---

## 🔒 Security Notes
- **Airplane Mode**: This software is intended to be used on a device with all radios (Wi-Fi, Bluetooth, Cellular) hardware-disabled or permanently turned off.
- **Panic Free**: The Rust core is designed to avoid panics, returning errors gracefully to the UI layer to prevent crashes and undefined behavior.
- **QR Density**: The TUI uses high-density Unicode blocks for QR codes to ensure readability on small screens while maintaining airgap security.
