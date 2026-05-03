# Riverbraid-Wasm-Bridge
> **Status:** ROOT_AUDIT_VERIFIED (v1.5.0)
> **Claim Boundary:** Declared Conditions Only[cite: 2]

This repository is a verified node within the Riverbraid Constellation.
It adheres to the Stationary State invariant and is audited against 
the Riverbraid-Manifest-Gold authority gate.[cite: 1, 2]

---
# Riverbraid WASM Bridge

This is the cognitive floor of the Riverbraid Protocol. It provides stateless, deterministic verification of constellation maps.

## Verification Status
- **Current State:** STATIONARY
- **Integrity Score:** 29/29
- **Target Architecture:** wasm32-wasip1

## Protocol Usage
The bridge is designed to be used in a pipeline:
`tsh scan --json | wasmtime riverbraid-wasm-bridge.wasm`

