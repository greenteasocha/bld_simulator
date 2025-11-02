# Rubik's Cube Simulator

An interactive TUI (Text User Interface) application for Rubik's Cube state editing and alternative solution search.

![Demo Screenshot](docs/screenshot.png)

## Features

- 🎨 **Interactive State Editor**: Edit Rubik's Cube states with a user-friendly TUI
- 🔄 **Real-time Visualization**: See your cube state update live as you edit
- 🔍 **Combined Nearby Search**: Find alternative solutions at both operation and move levels
- 📊 **Detailed Results**: View comprehensive analysis of all alternative paths

## Installation

### From Source

#### Prerequisites
- Rust 1.70 or later
- Cargo

#### Steps

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rubiks_cube_simulator.git
cd rubiks_cube_simulator
```

2. Build and run:
```bash
cargo build --release
cargo run --release
```

### Using Cargo Install

```bash
cargo install --git https://github.com/yourusername/rubiks_cube_simulator
```

After installation, run:
```bash
rubiks_cube_simulator
```

## Usage

### Interactive Mode

1. Run the application:
```bash
cargo run --release
# or if installed:
rubiks_cube_simulator
```

2. **Input Fields**:
   - **Scramble**: Enter move sequence (e.g., `R U R' D R U' R' D'`)
   - **Target State**: Edit CP, CO, EP, EO arrays using ↑↓ keys
   - **Real-time Preview**: Watch the cube visualization update on the right

3. **Navigation**:
   - `Tab` / `Enter`: Move to next field
   - `←` `→`: Move cursor within field
   - `↑` `↓`: Change values (for state fields)
   - `Esc`: Cancel and exit
   - `Enter` on EO field: Submit and run search

4. **Results**:
   - Original solution with operations and move sequences
   - Operation-level alternatives
   - Move-level alternatives
   - Summary statistics

### Example Workflow

```
1. Enter scramble: R U R' D R U' R' D'
2. Set target state (use default solved corners/edges with custom permutation)
3. Press Enter on EO field
4. View results showing alternative solutions
```
