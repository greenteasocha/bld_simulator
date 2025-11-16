# Cross Solver WebAssembly Demo

Node.js demo for the Cross Solver WebAssembly package.

## Prerequisites

- Node.js 18+ 
- The WebAssembly package must be built first

## Setup

```bash
cd web
npm install
```

## Usage

### Method 1: Interactive Demo Script

```bash
# Run with default scramble
node demo.js

# Run with custom scramble
node demo.js "R U R' D R U' R' D'"

# Complex scramble example
node demo.js "R U2 R' D2 R U' R' D R U R' U' R' F R F'"
```

### Method 2: Full Development Demo

```bash
# Compile TypeScript and run comprehensive demo
npm run dev

# Just compile TypeScript
npm run build

# Run compiled JavaScript
npm start
```

## Output Example

```
🎲 Cross Solver - BLD WebAssembly Demo
=====================================

📋 Parsing Scramble: "R U R' U'"
──────────────────────────────────────────────────
✅ Parsed moves: R U R' U'

📋 Applying Scramble to Solved State
──────────────────────────────────────────────────
✅ Scrambled state generated
   Corner Positions: [1, 0, 6, 3, 4, 5, 2, 7]
   Corner Orientations: [0, 2, 2, 0, 0, 0, 2, 0]
   Edge Positions: [0, 1, 4, 3, 5, 2, 6, 7, 8, 9, 10, 11]
   Edge Orientations: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

📋 BLD Solver Analysis
──────────────────────────────────────────────────
✅ BLD Solution Generated

🔶 Corner Operations:
   1. Swap: UFR ↔ FDR
   2. Swap: UFR ↔ RUB
   3. Swap: UFR ↔ UBL
   4. Swap: UFR ↔ UBR

🔷 Edge Operations:
   1. Swap: UF ↔ FR
   2. Swap: UF ↔ UB
   3. Swap: UF ↔ UR
   4. Swap: UF ↔ FR

🎯 Execution Sequences:
   1. Parity: FDR
      → R U R' U R U2 R'
   2. Parity: RUB
      → U R U' R' U R U2 R'
   3. Parity: UBL
      → L' U' L U' L' U2 L
   4. Parity: UBR
      → R U R' U R U2 R'

📊 Summary: 8 operations, 4 move sequences

🎉 Demo completed!
```

## Available Functions

The demo showcases these WebAssembly functions:

- `parse_scramble(input)` - Parse scramble notation
- `apply_scramble_to_state(scramble)` - Apply moves to solved cube  
- `solve_bld_with_default_moveset(cp, co, ep, eo)` - BLD solver with embedded algorithms

## Files

- `demo.js` - Simple interactive demo script
- `src/index.ts` - Comprehensive TypeScript demo  
- `package.json` - Node.js configuration
- `tsconfig.json` - TypeScript configuration