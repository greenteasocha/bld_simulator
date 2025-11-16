import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
// Get current directory for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
// Load WASM file
const wasmPath = join(__dirname, '../../pkg/bld_simulator_bg.wasm');
const wasmBuffer = readFileSync(wasmPath);
// Import WASM functions
import initWasm, { parse_scramble, apply_scramble_to_state, solve_bld_with_default_moveset, greet } from '../../pkg/bld_simulator.js';
async function main() {
    // Initialize WASM (use buffer directly for Node.js)
    await initWasm(wasmBuffer);
    console.log('🎲 Cross Solver WebAssembly Demo\n');
    // Test 1: Greet function
    console.log('1. Testing greet function:');
    console.log(greet('WebAssembly'));
    console.log('');
    // Test 2: Parse scramble
    console.log('2. Testing parse_scramble:');
    const scramble = "D2 F' U B D' F D L F' D2 F R2 D2 B2 L2 F U2 L2 F' L2";
    console.log(`Input: "${scramble}"`);
    const parsed = parse_scramble(scramble);
    console.log('Parsed result:', JSON.stringify(parsed, null, 2));
    console.log('');
    // Test 3: Apply scramble to solved state
    console.log('3. Testing apply_scramble_to_state:');
    console.log(`Applying scramble: "${scramble}"`);
    const scrambledState = apply_scramble_to_state(scramble);
    console.log('Scrambled state:', JSON.stringify(scrambledState, null, 2));
    console.log('');
    // // Test 4: Solve from state
    // if (scrambledState.success && scrambledState.state) {
    //   console.log('4. Testing solve_from_state:');
    //   const { cp, co, ep, eo } = scrambledState.state;
    //   // Convert arrays to Uint8Array for WASM
    //   const cpArray = new Uint8Array(cp);
    //   const coArray = new Uint8Array(co);
    //   const epArray = new Uint8Array(ep);
    //   const eoArray = new Uint8Array(eo);
    //   console.log('Solving state...');
    //   const solutions = solve_from_state(cpArray, coArray, epArray, eoArray);
    //   console.log('Solutions:', JSON.stringify(solutions, null, 2));
    //   if (solutions.success && solutions.solutions) {
    //     console.log('\n✅ All solutions found:');
    //     solutions.solutions.forEach((solution: string, index: number) => {
    //       console.log(`  Solution ${index + 1}: ${solution}`);
    //     });
    //   }
    // } else {
    //   console.log('4. Cannot test solver: scrambled state failed');
    // }
    // Test 5: BLD Solver with Default Moveset
    if (scrambledState.success && scrambledState.state) {
        console.log('5. Testing BLD Solver with Default Moveset (solve_bld_with_default_moveset):');
        const { cp, co, ep, eo } = scrambledState.state;
        // Convert arrays to Uint8Array for WASM
        const cpArray = new Uint8Array(cp);
        const coArray = new Uint8Array(co);
        const epArray = new Uint8Array(ep);
        const eoArray = new Uint8Array(eo);
        console.log('Running BLD solver with embedded moveset...');
        const bldSolution = solve_bld_with_default_moveset(cpArray, coArray, epArray, eoArray);
        console.log('BLD Solution:', JSON.stringify(bldSolution, null, 2));
        if (bldSolution.success && bldSolution.solution) {
            console.log('\n✅ BLD Solution Details:');
            console.log('Corner Operations:', bldSolution.solution.corner_operations);
            console.log('Edge Operations:', bldSolution.solution.edge_operations);
            console.log('All Operations Count:', bldSolution.solution.all_operations.length);
            console.log('Move Sequences Count:', bldSolution.solution.move_sequences.length);
            if (bldSolution.solution.move_sequences.length > 0) {
                console.log('\nMove Sequences:');
                bldSolution.solution.move_sequences.forEach((seq, i) => {
                    console.log(`  ${i + 1}. ${seq.description}: ${seq.sequence}`);
                });
            }
            console.log('\nFormatted Solution:');
            console.log(bldSolution.solution.formatted_solution);
        }
    }
    else {
        console.log('5. Cannot test BLD solver: scrambled state failed');
    }
    console.log('\n🎉 Demo completed!');
}
// Error handling
main().catch(error => {
    console.error('❌ Demo failed:', error);
    process.exit(1);
});
