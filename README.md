# Riverbraid WASM Bridge

This is the cognitive floor of the Riverbraid Protocol. It provides stateless, deterministic verification of constellation maps.

## Verification Status
- **Current State:** STATIONARY
- **Integrity Score:** 29/29
- **Target Architecture:** wasm32-wasip1

## Protocol Usage
The bridge is designed to be used in a pipeline:
`tsh scan --json | wasmtime riverbraid-wasm-bridge.wasm`
