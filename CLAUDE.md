# CLAUDE.md

This repository is now treated as the early Anvil codebase.

## Product intent

Anvil is a terminal-first coding-agent platform with Claude-like capabilities, but the runtime should remain provider-agnostic.

The engine is ours.
The model provider is pluggable.

## Detected stack

- Languages: Rust and Python
- Active product core: Rust
- Compatibility and parity surface: Python

## Verification

- Run Rust verification from `rust/`: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
- If behavior changes in compatibility surfaces, review `src/` and `tests/` too.

## Repository shape

- `rust/` contains the active CLI/runtime implementation and should be the default place for product work.
- `src/` contains compatibility and parity-oriented surfaces.
- `tests/` contains validation surfaces for compatibility logic.
- `docs/` contains the Anvil architecture, roadmap, and Claude-derived priority map.

## Working agreement

- Prefer small, reviewable changes.
- Keep Anvil branding and terminology consistent in new docs.
- Do not assume Anvil is tied permanently to Anthropic-only flows.
- When adding model-facing behavior, design the runtime around provider adapters rather than vendor-specific branching.
- Keep shared defaults in `.claude.json`; reserve `.claude/settings.local.json` for machine-local overrides.
