# Anvil

Anvil is a private, customizable coding-agent platform built from a clean-room study of Claude-like agent workflows and an actively evolving Rust CLI/runtime base.

The goal is not to preserve someone else's branding or history. The goal is to build a serious terminal-first agent we control: one that can plan, edit code, run tools, manage sessions, use MCP, and grow into a Claude-like daily driver for coding and adjacent tasks.

## Product Direction

Anvil is being designed around three layers:

- `Claude/` in the parent workspace is the research map. It tells us which subsystems matter and where the original product invested most of its complexity.
- `rust/` is the active product core. This is where the real Anvil runtime should live.
- `src/` and `tests/` are compatibility and parity surfaces that help us track coverage and preserve behavioral intent while the Rust runtime grows.

## Engine Strategy

Anvil's engine is the runtime we own, not any one model vendor.

- The Anvil engine will handle session state, tool orchestration, permissions, compaction, prompts, command routing, skills, plugins, and memory.
- Model providers will plug into that engine through adapters.
- The long-term target is provider-agnostic execution with support for Anthropic, OpenAI-compatible endpoints, and Gemini-class providers.
- The initial implementation should prioritize the providers that best support coding-agent tool use and streaming reliability.

This means Anvil can feel Claude-like without being locked to Claude.

## Current Repository Shape

```text
.
|-- rust/         # Active Rust workspace and CLI/runtime base
|-- src/          # Compatibility and parity surface
|-- tests/        # Verification for compatibility surfaces
`-- docs/         # Anvil architecture, roadmap, and priority notes
```

## Key Docs

- `docs/target-architecture.md`
- `docs/roadmap.md`
- `docs/claude-priority-map.md`

## Near-Term Goal

Build a private repo and product identity around Anvil, then close the highest-value gaps in this order:

1. core runtime and tool loop polish
2. command and transport parity
3. skills and plugins
4. task orchestration and sub-agents
5. provider routing and model choice

## Verification

Run Rust verification from `rust/`:

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Notes

- `claw-code` should remain available as an upstream sync source, not the shipping identity.
- Anvil should live in its own private repository with its own history, docs, and roadmap.
