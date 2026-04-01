# Anvil Rust Workspace

This directory contains the active shipping implementation of Anvil.

## Purpose

The Rust workspace is the core product engine for Anvil. It is where the CLI, runtime, built-in tools, command surface, auth flows, session handling, MCP support, and most shipping work should happen.

## Current Workspace Status

Implemented today:

- `anvil` CLI binary
- interactive REPL and one-shot prompt mode
- Anthropic API client and OAuth flow
- session persistence, resume, export, and compaction
- permission system and tool allowlists
- MCP stdio bootstrap and tool routing
- hooks runtime support
- tool implementations for shell, PowerShell, file operations, search, web, todo, notebook, skill loading, REPL execution, and sub-agent primitives
- passing Rust workspace tests

Not finished yet:

- provider abstraction layer
- OpenAI-compatible and Gemini adapters
- full command breadth for planning, review, tasks, skills, plugins, and MCP management
- plugin system
- full skills registry
- packaging and distribution workflow

## Quick Start

```bash
cd rust
cargo build --release
./target/release/anvil
```

One-shot prompt:

```bash
./target/release/anvil prompt "explain this codebase"
```

## Verification

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Workspace Layout

```text
rust/
|-- Cargo.toml
|-- Cargo.lock
`-- crates/
    |-- api/
    |-- commands/
    |-- compat-harness/
    |-- runtime/
    |-- rusty-claude-cli/
    `-- tools/
```

## Crate Responsibilities

- `api`: HTTP client, auth, request types, retry logic, and streaming parsing
- `commands`: shared slash-command registry and command metadata
- `compat-harness`: coverage extraction and source-shape comparison helpers
- `runtime`: sessions, prompts, permissions, MCP plumbing, hooks, usage, and orchestration
- `rusty-claude-cli`: REPL, prompt execution, rendering, and CLI argument handling
- `tools`: built-in tool registry and tool execution logic

## Working Rule

When a major implementation milestone lands in this workspace, also update the root [README.md](../README.md) and the relevant documents under [docs](../docs).
