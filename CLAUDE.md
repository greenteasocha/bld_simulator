# Claude Code Instructions for cross-solver

## Project Overview
Rubik's Cube state editor and solver with WebAssembly export capability.

## Environment Setup Commands
Run these commands at the start of each session to ensure proper Rust/WASM environment:

```bash
# Set up Rust environment
source ~/.cargo/env
rustup default stable
rustup target add wasm32-unknown-unknown
```

## Development Commands

### WebAssembly Build
```bash
# Build Rust to WebAssembly
wasm-pack build --target web --out-dir pkg

# Output will be in pkg/ directory:
# - bld_simulator.js (JavaScript wrapper)
# - bld_simulator_bg.wasm (WebAssembly binary)  
# - bld_simulator.d.ts (TypeScript definitions)
```

### Standard Rust Build
```bash
# Build TUI application
cargo build --release

# Run TUI application  
cargo run
```

### Testing
```bash
# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run lints
cargo clippy
```

## Project Structure

- `src/` - Rust source code
  - `main.rs` - TUI entry point
  - `lib.rs` - Library exports 
  - `wasm.rs` - WebAssembly bindings
  - `cube/` - Core cube logic
  - `parser/` - Move notation parsing
  - `display/` - TUI display logic
- `pkg/` - Generated WebAssembly output (gitignored)
- `web/` - Frontend project (to be created)

## Configuration Files

- `Cargo.toml` - Configured for both native and WASM builds
- WebAssembly dependencies: wasm-bindgen, serde-wasm-bindgen
- TUI dependencies: ratatui (native only)

## Notes
- Always run environment setup commands after shell restart
- WASM build generates TypeScript-compatible bindings
- TUI and WASM builds use same core logic modules