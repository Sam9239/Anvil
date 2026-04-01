# CLAUDE.md

This file provides working guidance for the Rust workspace inside Anvil.

## Detected stack

- Language: Rust
- Workspace role: active product implementation

## Verification

Run Rust verification from the workspace root:

- `cargo fmt`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## Working agreement

- Prefer small, reviewable changes and keep generated bootstrap files aligned with actual repo workflows
- Keep shared defaults in `.claude.json`; reserve `.claude/settings.local.json` for machine-local overrides
- Do not overwrite existing `CLAUDE.md` content automatically; update it intentionally when repo workflows change
- When major product milestones land, also update the root `README.md` and the planning docs under `../docs/`
