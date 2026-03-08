---
name: Vibe Wallet Development
description: Guidelines and patterns for developing the Vibe airgapped hardware wallet using Hexagonal Architecture.
---

# Vibe Wallet Development Skill

This skill provides instructions and patterns for maintaining and extending the Vibe Hardware Wallet, a Rust-based airgapped solution designed for Android.

## 🏗 Core Architecture: Hexagonal (Ports & Adapters)

The project strictly follows Hexagonal Architecture to separate domain logic from infrastructure.

### 1. Domain Layer (`wallet-core/src/domain`)
- **Entities**: Business objects like `Wallet` and `Chain`.
- **Ports (Driven)**: Traits defining what the core needs (e.g., `MnemonicProvider`, `CryptoProvider`, `AirgapProvider`).
- **Rule**: No dependencies on external crates or infrastructure details here.

### 2. Application Layer (`wallet-core/src/application`)
- **Services**: `WalletService` coordinates the workflow.
- **Dependency Injection**: Services receive trait objects (`Arc<dyn Trait>`) from the composition root.

### 3. Infrastructure Layer (`wallet-core/src/infrastructure`)
- **Adapters**: Concrete implementations of domain ports (e.g., `Bip39Adapter`, `CryptoAdapter`, `UrAdapter`).
- **External Crates**: This is where logic involving `tiny-bip39`, `k256`, `ed25519-dalek`, and `ur` resides.

### 4. Primary Adapters (Drivers)
- **`wallet-tui`**: Terminal UI for direct interaction.
- **`wallet-ffi`**: UniFFI bridge for Android integration.

## 🚀 Key Patterns

### Adding a New Chain
1. Update `Chain` enum in `domain/chain.rs`.
2. Update `CryptoProvider` trait and its implementation in `infrastructure/crypto_adapter.rs`.
3. Update FFI mappings in `wallet-ffi/src/lib.rs`.

### Adding a New Capability
1. Define a Port (trait) in `domain/crypto.rs` or a new domain file.
2. Add the capability to `WalletService` in `application/wallet_service.rs`.
3. Implement an Adapter in `infrastructure/`.
4. Inject the adapter in `wallet-tui/src/main.rs` and `wallet-ffi/src/lib.rs`.

## 🛠 Commands

### Development & Test
- **Test Core**: `cargo test -p wallet-core`
- **Run TUI**: `cargo run -p wallet-tui`
- **Build Mobile**: `cargo ndk -t aarch64-linux-android build --release`

### Mobile Bindings (UniFFI)
```bash
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/lib.rs --language kotlin --out-dir ../../out/
```

## 🔒 Security Requirements
- **Panic-Free**: All functions must return `Result<T, String>` to allow safe error handling in UI/FFI.
- **Airgap First**: All communication must go through `AirgapProvider` (UR fragments).
- **Secure Derivation**: Always use optional PIN/Passphrase support in seed generation where applicable.
