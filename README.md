# Vibe Hardware Wallet (Rust Core)

A secure, airgapped hardware wallet implementation in Rust, designed to run on old Android smartphones. This core supports multiple chains (EVM and Solana) through a modular, hexagonal architecture.

## 🚀 Features

- **Airgapped Design**: No network permissions required. All communication via QR codes (UR protocol).
- **Hexagonal Architecture**: Strict separation of domain logic, application services, and infrastructure adapters.
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
cd rust
cargo build
```

### 2. Build for Android (EVM & Solana Core)
Use `cargo-ndk` to build the shared libraries (`.so`) for different Android architectures:
```bash
cd rust
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi build --release
```
The output will be in `rust/target/`.

### 3. Run the Terminal UI (Rust Only)
You can interact with the wallet core directly from your terminal:
```bash
cd rust
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
cd rust/wallet-ffi
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/lib.rs --language kotlin --out-dir ../../out/
```

---

## 🧪 Testing

Run the automated test suite to verify mnemonic generation and address derivation for both EVM and Solana:
```bash
cd rust
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
