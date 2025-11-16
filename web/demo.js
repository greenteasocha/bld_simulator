#!/usr/bin/env node
/**
 * Cross Solver WebAssembly Demo
 * 
 * Usage:
 *   node demo.js                        # Run with default scramble
 *   node demo.js "R U R' D R U' R' D'"  # Run with custom scramble
 */

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Import WASM functions
import initWasm, { 
  parse_scramble, 
  apply_scramble_to_state, 
  solve_bld_with_default_moveset,
  greet
} from '../pkg/bld_simulator.js';

async function loadWasm() {
  const wasmPath = join(__dirname, '../pkg/bld_simulator_bg.wasm');
  const wasmBuffer = readFileSync(wasmPath);
  await initWasm(wasmBuffer);
}

function printHeader() {
  console.log('🎲 Cross Solver - BLD WebAssembly Demo');
  console.log('=====================================\n');
}

function printSection(title) {
  console.log(`\n📋 ${title}`);
  console.log('─'.repeat(50));
}

async function demonstrateScramble(scramble) {
  printSection(`Parsing Scramble: "${scramble}"`);
  
  // 1. Parse the scramble
  const parsed = parse_scramble(scramble);
  
  if (!parsed.success) {
    console.log(`❌ Failed to parse scramble: ${parsed.error}`);
    return;
  }
  
  console.log(`✅ Parsed moves: ${parsed.moves.join(' ')}`);
  
  // 2. Apply scramble to solved state
  printSection('Applying Scramble to Solved State');
  const scrambledState = apply_scramble_to_state(scramble);
  
  if (!scrambledState.success) {
    console.log(`❌ Failed to apply scramble: ${scrambledState.error}`);
    return;
  }
  
  console.log('✅ Scrambled state generated');
  console.log(`   Corner Positions: [${scrambledState.state.cp.join(', ')}]`);
  console.log(`   Corner Orientations: [${scrambledState.state.co.join(', ')}]`);
  console.log(`   Edge Positions: [${scrambledState.state.ep.join(', ')}]`);
  console.log(`   Edge Orientations: [${scrambledState.state.eo.join(', ')}]`);
  
  // 3. Solve using BLD method
  printSection('BLD Solver Analysis');
  
  const { cp, co, ep, eo } = scrambledState.state;
  const cpArray = new Uint8Array(cp);
  const coArray = new Uint8Array(co);
  const epArray = new Uint8Array(ep);
  const eoArray = new Uint8Array(eo);
  
  const solution = solve_bld_with_default_moveset(cpArray, coArray, epArray, eoArray);
  
  if (!solution.success) {
    console.log(`❌ BLD solver failed: ${solution.error}`);
    return;
  }
  
  const { solution: bldSolution } = solution;
  
  console.log('✅ BLD Solution Generated\n');
  
  // Display corner operations
  if (bldSolution.corner_operations.length > 0) {
    console.log('🔶 Corner Operations:');
    bldSolution.corner_operations.forEach((op, i) => {
      console.log(`   ${i + 1}. ${op}`);
    });
  } else {
    console.log('🔶 Corner Operations: None (corners already solved)');
  }
  
  // Display edge operations  
  if (bldSolution.edge_operations.length > 0) {
    console.log('\n🔷 Edge Operations:');
    bldSolution.edge_operations.forEach((op, i) => {
      console.log(`   ${i + 1}. ${op}`);
    });
  } else {
    console.log('\n🔷 Edge Operations: None (edges already solved)');
  }
  
  // Display move sequences
  if (bldSolution.move_sequences.length > 0) {
    console.log('\n🎯 Execution Sequences:');
    bldSolution.move_sequences.forEach((seq, i) => {
      console.log(`   ${i + 1}. ${seq.description}`);
      console.log(`      → ${seq.sequence}`);
    });
  } else {
    console.log('\n🎯 Execution Sequences: None (already solved)');
  }
  
  console.log(`\n📊 Summary: ${bldSolution.all_operations.length} operations, ${bldSolution.move_sequences.length} move sequences`);
}

async function main() {
  try {
    // Load WASM
    await loadWasm();
    
    printHeader();
    
    // Get scramble from command line or use default
    const scramble = process.argv[2] || "R U R' U'";
    
    await demonstrateScramble(scramble);
    
    console.log('\n🎉 Demo completed!');
    console.log('\n💡 Try with your own scramble:');
    console.log('   node demo.js "R U2 R\' D R U\' R\' D\'"');
    
  } catch (error) {
    console.error('❌ Demo failed:', error.message);
    process.exit(1);
  }
}

main();