# Feature Coverage Map For Anvil

This document turns the repository's source-analysis inputs into implementation priorities for Anvil.

## Source Basis

This planning view is grounded in the local analysis assets under `Claude/`, especially:

- `Claude/claude_code_src_report.md`
- `Claude/claude_code_file_inventory.md`
- `Claude/claude_code_tree_depth3.txt`
- `Claude/analysis.json`

## Coverage Thesis

The highest-value result does not come from copying every source file or every terminal UI detail. It comes from reproducing the most important behavioral surfaces inside Anvil:

- reliable query and conversation loop
- broad tool orchestration
- strong permissions model
- command-driven workflows
- extensibility through skills and plugins
- stable session and memory handling
- provider flexibility without fragmenting the runtime

## Current Priority Order

### Tier 1: Shipping-critical surfaces

These are the areas that matter most before we can call the main goal achieved:

- runtime and provider abstraction
- tool registry and tool execution breadth
- command surface breadth
- permissions, sessions, hooks, and memory handling
- CLI and structured automation flows

### Tier 2: High-value extension surfaces

These should land before or alongside the first strong ship target:

- skills registry
- plugin system
- MCP resource and workflow breadth
- richer task and sub-agent orchestration

### Tier 3: Later breadth and polish

These can follow the first strong ship target if the core ship criteria are satisfied:

- richer terminal UX parity
- broader remote transport layers
- companion, voice, or analytics-heavy features
- non-essential UI and ecosystem breadth

## Current Surface Status

### Runtime and assistant loop

Status: strong foundation

Implemented:

- local conversation runtime
- session persistence and compaction
- permission handling
- prompt assembly and project guidance discovery
- usage tracking
- hook execution support

Remaining:

- provider decoupling
- broader orchestration product features
- richer long-running workflow control

### Tools

Status: partial but already useful

Implemented:

- shell and PowerShell execution
- file operations
- search tools
- web tools
- notebook editing
- todo tracking
- local skill loading
- REPL execution
- sub-agent execution primitive

Remaining:

- broader MCP-facing tools
- more workflow and system tools
- richer task and resource operations
- removal of remaining MVP-only assumptions

### Commands

Status: partial

Implemented:

- session, status, model, permissions, config, cost, init, export, and related local commands

Remaining:

- plan
- review
- tasks
- MCP management
- skills management
- plugin management

### Skills

Status: basic local loading only

Implemented:

- direct local `SKILL.md` loading

Remaining:

- bundled skill registry
- project and user skill management
- discovery and ranking flows
- tighter command integration

### Plugins

Status: missing

Remaining:

- manifest model
- loader and lifecycle
- command, tool, and hook extension path
- enable, disable, and reload flows

### Provider breadth

Status: missing as a product layer

Implemented:

- current Anthropic-facing API path

Remaining:

- provider abstraction
- OpenAI-compatible adapter
- Gemini adapter path
- normalized provider capability model

## What To Maximize First

If the goal is maximum useful coverage rather than maximum file-count imitation, Anvil should prioritize implementation in this order:

1. provider abstraction and multi-provider support
2. command breadth for planning, review, tasks, MCP, skills, and plugins
3. broader tool coverage and automation quality
4. skills registry
5. plugin system
6. deeper orchestration and transport breadth

## Working Rule

Whenever a surface moves materially from missing to partial, or from partial to strong, update this document and `PARITY.md` in the same milestone.
