# Anvil Shipping Plan

## Goal Definition

The goal is not simply to compile Anvil. The goal is to ship a working Anvil CLI that is strong enough to serve as a real daily coding agent and broad enough to cover most of the expected product surface represented in the repository's coverage targets.

We can say the goal is achieved only when Anvil is both:

- technically stable enough to install and run confidently
- functionally broad enough to cover the required core workflows

## Current Assessed State

What is already true:

- Anvil has its own repository and product identity
- the Rust workspace is the active implementation core
- the Rust workspace test suite is green
- REPL, prompt mode, sessions, permissions, MCP stdio, hooks, auth, and a substantial built-in tool set already exist
- compatibility and coverage surfaces still exist in `src/` and `tests/`

What is not yet true:

- provider abstraction is not complete
- provider breadth is not complete
- command breadth is not complete
- plugin system is not implemented
- skills registry is not implemented
- packaging and release automation are not complete
- first-install user experience is not yet release-ready

## Required Before We Can Call The Goal Achieved

### 1. Engine And Provider Layer

Completed:

- runtime core exists
- Anthropic-facing API path exists
- session, prompt, permission, and hook infrastructure exists

Remaining:

- create a provider interface owned by Anvil
- move provider-specific assumptions behind that interface
- keep Anthropic support working through the new provider interface
- add an OpenAI-compatible adapter
- define and implement the Gemini adapter path
- normalize provider usage reporting, finish reasons, and tool-call behavior
- allow provider and model selection from config and CLI

Shipping priority: must ship

### 2. Command Surface

Completed:

- help, status, compact, clear, model, permissions, cost, resume, config, memory, init, diff, version, export, and session commands exist

Remaining:

- add planning commands
- add review commands
- add task-management commands
- add MCP-management commands
- add skills-management commands
- add plugin-management commands
- expand diagnostics and environment health flows where needed

Shipping priority: must ship

### 3. Tool Coverage

Completed:

- shell and PowerShell execution
- file read, write, and edit
- glob and grep search
- web fetch and web search
- notebook editing
- todo tracking
- local skill loading
- sub-agent execution primitive
- REPL execution

Remaining:

- broaden MCP-facing tools and resource flows
- add missing workflow/system tools required for richer coding-agent behavior
- evaluate whether LSP-style tooling is needed for first ship or immediate follow-up
- improve structured output consistency for automation-heavy flows
- continue replacing MVP-only assumptions in the tool registry

Shipping priority: must ship

### 4. Skills System

Completed:

- local `SKILL.md` loading works

Remaining:

- build a bundled skills registry
- support explicit project and user skill directories
- add discovery and management flows
- define skill metadata and selection rules
- integrate skills cleanly with command and tool workflows

Shipping priority: must ship

### 5. Plugin System

Completed:

- no complete plugin subsystem yet

Remaining:

- define plugin manifest format
- implement plugin discovery and loading
- support enable, disable, and reload flows
- allow plugin-provided tools, commands, or hooks within clear safety rules
- document extension boundaries and lifecycle

Shipping priority: must ship

### 6. Sessions, Memory, And Orchestration

Completed:

- session persistence, resume, export, and compaction exist
- hooks runtime behavior exists
- sub-agent primitive exists

Remaining:

- expand orchestration beyond current primitives
- add stronger task lifecycle flows
- define plan, execute, and verify control loops
- clarify what first-ship background task support must include
- improve memory and long-running workflow ergonomics where needed

Shipping priority: required for a strong ship, though some deeper orchestration can follow in a later milestone

### 7. Config, Auth, And Security

Completed:

- API-key and OAuth support exist
- permission modes exist
- config loading exists
- MCP config support exists

Remaining:

- make provider configuration first-class
- add better diagnostics for misconfiguration
- review default permissions and release-safe defaults
- define what secrets and tokens are stored, where, and how they are cleared
- document trusted and untrusted extension behavior clearly

Shipping priority: must ship

### 8. Packaging And Distribution

Completed:

- local build flow exists

Remaining:

- choose release packaging strategy
- produce release artifacts for supported platforms
- document installation and upgrade steps
- add self-check or doctor flow if it is required for supportability
- decide how updates are delivered

Shipping priority: must ship

### 9. CI, QA, And Verification

Completed:

- local Rust verification commands are known
- Rust workspace tests are green

Remaining:

- add CI workflows for format, lint, and test gates
- add targeted smoke tests for the binary itself
- add provider-adapter regression tests
- add release checklists for packaging and basic install validation
- define the supported-platform verification matrix

Shipping priority: must ship

### 10. Documentation And Product Framing

Completed:

- product docs exist
- architecture and roadmap docs exist

Remaining:

- keep README synchronized after every major milestone
- document install, configuration, and operator workflows clearly
- document provider selection once the abstraction lands
- document plugin and skill lifecycles once implemented
- keep parity and shipping docs synchronized with the code

Shipping priority: must ship

## Release Gates

### Gate A: Foundation

Must be true:

- Rust workspace stable
- tests green
- docs aligned with current state

Current state: complete

### Gate B: Provider Decoupling

Must be true:

- provider abstraction implemented
- Anthropic adapter moved behind the interface
- at least one additional adapter working

Current state: not started

### Gate C: Product Workflow Coverage

Must be true:

- command and tool breadth covers the required daily workflows
- structured output is reliable enough for automation use
- must-have coverage gaps are resolved

Current state: partial

### Gate D: Extension Readiness

Must be true:

- skills registry exists
- plugin system exists
- extension rules are documented

Current state: not started

### Gate E: Release Readiness

Must be true:

- packaging complete
- CI complete
- install path documented and tested
- release checklist complete

Current state: not started

## What Can Wait Until After First Strong Ship

These items matter, but they do not have to block the first strong release unless they become necessary during implementation:

- richer remote transport layers
- broader analytics and service ecosystem
- deep marketplace workflows
- broader UI polish beyond CLI quality requirements
- advanced domain-specific workflow packs

## Working Rule

This file is the source of truth for remaining work. If a major milestone changes what is complete, what is blocked, or what is required, update this document in the same change set.

