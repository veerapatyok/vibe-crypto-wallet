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
- **Error Handling**: All ports must return `anyhow::Result` for consistent, rich error reporting.
- **Rule**: No dependencies on infrastructure-specific error types here.

### 2. Application Layer (`wallet-core/src/application`)
- **Services**: `WalletService` coordinates the workflow using `?` for clean error propagation.
- **Dependency Injection**: Services receive trait objects (`Arc<dyn Trait>`) from the composition root.

### 3. Infrastructure Layer (`wallet-core/src/infrastructure`)
- **Adapters**: Concrete implementations of domain ports (e.g., `Bip39Adapter`, `CryptoAdapter`, `UrAdapter`).
- **Error Mapping**: Map external crate errors into `anyhow::Error` using `context()` or `anyhow!`.

### 4. Primary Adapters (Drivers)
- **`wallet-tui`**: Terminal UI for direct interaction. Uses `anyhow` for app-wide error management.
- **`wallet-ffi`**: UniFFI bridge for Android/iOS. **CRITICAL**: Must map `anyhow::Error` to `String` or a specific UniFFI error enum at the boundary using `.map_err(|e| e.to_string())`.

## 🚀 Key Patterns

### Error Handling
1. **Core Logic**: Always use `anyhow::Result<T>`.
2. **Infrastructure**: Convert specialized error types (e.g., `bip39::Error`) to `anyhow`.
3. **Boundaries**: Convert `anyhow::Error` to the target platform's error format (e.g., `String` for FFI).

### Adding a New Chain
1. Update `Chain` enum in `domain/crypto.rs`.
2. Update `CryptoProvider` trait and its implementation in `infrastructure/crypto_adapter.rs`.
3. Update FFI mappings in `wallet-ffi/src/lib.rs`.

### Adding a New Capability
1. Define a Port (trait) in `domain/crypto.rs` or a new domain file.
2. Add the capability to `WalletService` in `application/wallet_service.rs`.
3. Implement an Adapter in `infrastructure/`.
4. Inject the adapter in `wallet_tui/src/main.rs` and `wallet_ffi/src/lib.rs`.

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
- **Panic-Free**: All functions must return `Result` and avoid `unwrap()` to allow safe error handling in UI/FFI.
- **Airgap First**: All communication must go through `AirgapProvider` (UR fragments).
- **Secure Derivation**: Always use optional PIN/Passphrase support in seed generation where applicable.
- **Robust Error Contexts**: Use `anyhow` to ensure sufficient debug information is available if a cryptographic operation fails.
