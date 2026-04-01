# CLAUDE.md

This repository is the Anvil codebase.

## Product intent

Anvil is a terminal-first coding agent platform. The product engine belongs to Anvil, and model vendors must integrate through adapters instead of being wired directly into the core runtime.

## Detected stack

- Languages: Rust and Python
- Active product core: Rust
- Compatibility and coverage surface: Python and analysis assets

## Verification

- Run Rust verification from `rust/`: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
- If compatibility behavior changes, review `src/` and `tests/` too

## Repository shape

- `rust/` contains the shipping implementation core and should be the default place for product work
- `src/` contains compatibility, analysis, and coverage-oriented surfaces
- `tests/` contains validation for compatibility and porting flows
- `docs/` contains the shipping plan, roadmap, architecture, and coverage docs

## Working agreement

- Prefer small, reviewable changes
- Keep Anvil terminology consistent in product-facing docs
- Design model-facing logic around provider adapters rather than vendor-specific branching inside the runtime
- Keep shared defaults in `.claude.json`; reserve `.claude/settings.local.json` for machine-local overrides
- After every major milestone, update `README.md`, `docs/shipping-plan.md`, `docs/roadmap.md`, `docs/target-architecture.md`, and `PARITY.md` if their claims changed
