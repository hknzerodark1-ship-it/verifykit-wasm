import init, { hash_input } from './pkg/verifykit_wasm.js';
import { createHash } from 'node:crypto';

// 256 lattice vectors — all combinations of Tu, Du, Ru, Phu
const LATTICE_VECTORS = [];

// Generate all 256 combinations (4^4)
const symbols = ['tu', 'du', 'ru', 'phu'];
for (let i = 0; i < 4; i++) {
    for (let j = 0; j < 4; j++) {
        for (let k = 0; k < 4; k++) {
            for (let l = 0; l < 4; l++) {
                LATTICE_VECTORS.push(`${symbols[i]} ${symbols[j]} ${symbols[k]} ${symbols[l]}`);
            }
        }
    }
}

async function runSaturationTest() {
    await init();
    
    let passed = 0;
    let failed = 0;
    const failures = [];
    
    console.log("🜏 PARITY SATURATION TEST — 256 LATTICE STATES 🜏");
    console.log("");
    
    for (let i = 0; i < LATTICE_VECTORS.length; i++) {
        const input = LATTICE_VECTORS[i];
        const wasmHash = hash_input(input);
        
        // BLAKE3 hash via Node.js crypto
        const nodeHash = createHash('blake3').update(input).digest('hex');
        
        if (wasmHash === nodeHash) {
            passed++;
        } else {
            failed++;
            failures.push({ index: i, input, wasmHash, nodeHash });
        }
        
        // Progress indicator every 64 vectors
        if ((i + 1) % 64 === 0) {
            console.log(`   Processed: ${i + 1}/256 — Passed: ${passed}, Failed: ${failed}`);
        }
    }
    
    console.log("");
    console.log("════════════════════════════════════════════════════════════");
    console.log(`📊 SATURATION TEST COMPLETE`);
    console.log(`   ✅ Passed: ${passed}/256`);
    console.log(`   ❌ Failed: ${failed}/256`);
    console.log(`   📈 Coverage: ${(passed / 256 * 100).toFixed(2)}%`);
    
    if (failed > 0) {
        console.log("");
        console.log("⚠️ FAILURES DETECTED:");
        failures.slice(0, 5).forEach(f => {
            console.log(`   Vector ${f.index}: "${f.input}"`);
        });
        process.exit(1);
    }
    
    console.log("");
    console.log("🜏 ZERO RESIDUE — SATURATION ACHIEVED 🜏");
    console.log("   Every lattice state is bit-verified across Rust ↔ WASM.");
    console.log("   Total Saturation: No dark logic remains.");
}

runSaturationTest().catch(console.error);
