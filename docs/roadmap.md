# Anvil Roadmap

## Release Objective

Ship Anvil as a stable, terminal-first coding agent CLI with a strong local runtime, multi-provider capability, broad workflow tooling, and extension surfaces that support real daily engineering work.

## Current Phase Snapshot

As of 2026-04-01:

- Phase 0: complete
- Phase 1: complete
- Phase 2: next priority
- Phase 3: required before ship
- Phase 4: required before ship
- Phase 5: required before ship
- Phase 6: follow-on growth after the first strong ship target

## Phase 0: Product Identity And Repo Foundation

Status: complete

Delivered:

- standalone Anvil repository
- Anvil branding at the product level
- private GitHub origin
- clear separation between shipping implementation and coverage surfaces
- initial architecture and roadmap docs

Exit criteria met:

- Anvil has its own repo identity
- product docs are under Anvil branding
- the repo can move independently

## Phase 1: Runtime Hardening And Test Stability

Status: complete

Delivered:

- Rust CLI rebrand to `anvil`
- green Rust workspace tests
- Windows-focused shell, PowerShell, OAuth, MCP stdio, prompt, and tool test stabilization
- improved local verification confidence for the current engine

Exit criteria met:

- the Rust workspace is stable enough to keep building on
- local core behavior is test-backed rather than speculative

## Phase 2: Provider Abstraction

Status: next priority

Required deliverables:

- introduce a provider interface owned by Anvil
- move provider-specific behavior out of runtime glue where necessary
- keep current Anthropic support working behind the provider interface
- add an OpenAI-compatible adapter
- define the path for Gemini support and additional adapters
- add provider and model selection to configuration and CLI flows

Exit criteria:

- the runtime no longer assumes one provider model
- providers can be added without rewriting the engine core
- usage and tool-call behavior are normalized across adapters

## Phase 3: Command And Tool Coverage

Status: required before ship

Required deliverables:

- add plan, review, tasks, MCP-management, skills, and plugin commands
- expand the tool registry beyond the current local MVP surface
- improve structured output and automation-oriented CLI behavior
- strengthen workflow tools for project operations and agent control
- close the highest-value coverage gaps tracked in `PARITY.md`

Exit criteria:

- Anvil supports the expected day-to-day coding workflows from the CLI
- the command surface feels complete rather than foundational
- must-have tool families are present and stable

## Phase 4: Skills, Plugins, And Extension Surfaces

Status: required before ship

Required deliverables:

- build a bundled skills registry
- support project and user skills cleanly
- implement a plugin loader and lifecycle model
- support plugin-provided tools, commands, or hooks where appropriate
- define extension boundaries and safety rules

Exit criteria:

- Anvil can grow without patching the core crates for every new workflow
- skills and plugins are first-class product concepts

## Phase 5: Packaging, QA, And Release Readiness

Status: required before ship

Required deliverables:

- release builds for supported platforms
- install and upgrade workflow
- CI for format, lint, and test gates
- smoke tests for prompt, REPL, session, tool, and provider flows
- release checklist and operator docs
- packaging and distribution decisions finalized

Exit criteria:

- a new user can install Anvil and run it successfully
- release quality is enforced by automation, not memory

## Phase 6: Expanded Product Breadth

Status: follow-on after the first strong ship target

Potential deliverables:

- richer task orchestration and background execution
- more advanced remote or structured transport layers
- broader service ecosystem
- domain-specific workflow packs
- enhanced UX polish beyond the first shipping requirement

Exit criteria:

- Anvil moves from strong CLI foundation to broader product breadth

## Roadmap Rules

These rules apply every time the roadmap changes:

- a phase is not complete until verification is green
- a phase is not complete until the relevant docs are updated
- README status must reflect the latest major milestone
- `PARITY.md` must be updated when coverage claims change materially
