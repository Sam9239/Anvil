# Anvil Target Architecture

## Product Goal

Ship Anvil as a terminal-first coding agent whose core runtime, tool loop, permissions, memory, and workflow behavior belong to Anvil rather than to any single model vendor.

## Architecture Principles

- The engine is Anvil.
- Providers are adapters.
- Product logic stays in the runtime, not in vendor-specific branches.
- Tool and command contracts must be stable across providers.
- Extension surfaces must be first-class, not runtime hacks.
- The shipping implementation lives primarily in Rust.

## Current Implemented Architecture

The current product core already exists and is centered in the Rust workspace.

### Implemented crates

- `api`: provider-facing Anthropic client, auth, request types, retries, and SSE parsing
- `commands`: shared slash-command registry and help metadata
- `compat-harness`: source-shape extraction and coverage helpers
- `runtime`: sessions, prompts, permissions, hooks, usage, config, MCP, and orchestration
- `rusty-claude-cli`: REPL, prompt execution, rendering, argument parsing, and CLI glue
- `tools`: built-in tool registry and tool execution implementations

### Implemented behavior

- REPL and one-shot prompt execution
- session persistence, resume, export, and compaction
- permission modes and tool allowlists
- local shell and PowerShell execution
- file, search, web, notebook, todo, skill, REPL, and sub-agent tools
- OAuth and API-key auth flows
- MCP stdio bootstrap and routing
- hooks runtime support
- usage and cost tracking

## Target Shipping Architecture

Anvil should ship with the following architectural layers.

### 1. Product shell

Responsibilities:

- interactive REPL
- one-shot prompt mode
- structured output modes
- session resume and export
- command execution and help flows
- rendering and operator feedback

Current state:

- implemented in `rust/crates/rusty-claude-cli`
- stable foundation exists
- broader command surface and cleaner structured transport are still needed

### 2. Engine runtime

Responsibilities:

- conversation state
- prompt assembly
- permissions
- tool orchestration
- hooks
- compaction
- usage tracking
- session persistence
- MCP and local workflow plumbing

Current state:

- implemented in `rust/crates/runtime`
- core engine exists and is tested
- still needs provider decoupling and broader product orchestration

### 3. Tool system

Responsibilities:

- tool registry
- schemas and validation
- execution adapters
- permission-aware access
- built-in tool families
- future plugin and MCP tool injection

Current state:

- implemented in `rust/crates/tools`
- good local core exists
- tool breadth still lags the full target feature set

### 4. Provider layer

Responsibilities:

- normalize provider request and response flows
- stream text and tool events into one internal contract
- normalize usage and finish reasons
- advertise provider capabilities
- support configuration-based provider selection

Current state:

- provider abstraction exists as its own `providers` crate
- CLI/runtime dispatch now flows through provider adapters instead of an Anthropic-only runtime client
- remaining work is around config-driven provider defaults, richer retries, and shipping polish

Target home:

- `rust/crates/providers`

### 5. Command layer

Responsibilities:

- slash-command registry
- validation and parsing
- shared help metadata
- runtime command handlers
- future plan, review, tasks, skills, plugin, and MCP workflows

Current state:

- implemented in `rust/crates/commands`
- current command core is solid
- major command families are still missing

### 6. Extension surfaces

Responsibilities:

- bundled skill registry
- project and user skill loading
- plugin loading and lifecycle
- tool and command injection
- hook extensions
- future marketplace support

Current state:

- local skill-file loading exists
- plugin system is not implemented
- full skills registry is not implemented

Target homes:

- `rust/crates/skills`
- `rust/crates/plugins`

### 7. Agent orchestration

Responsibilities:

- plan, execute, verify loops
- background work
- task views and summaries
- sub-agent control
- future remote workers

Current state:

- sub-agent primitives exist
- full orchestration product surface does not yet exist

Target home:

- `rust/crates/agents`

### 8. Storage and transport

Responsibilities:

- local session store
- memory stores
- future structured and remote transports
- cached outputs and traces
- import/export boundaries

Current state:

- session persistence exists in runtime
- broader storage and transport separation is still pending

Target homes:

- `rust/crates/storage`
- `rust/crates/transports`

## Provider Contract Requirement

Before shipping, Anvil should standardize on one internal provider event model.

Recommended event families:

- `TextDelta`
- `ToolUseStart`
- `ToolUseInputDelta`
- `ToolUseStop`
- `ToolResult`
- `Usage`
- `MessageStop`
- `Error`

Every provider adapter should map its native protocol into this contract so the CLI, runtime, tools, and session store do not need vendor-specific logic.

## Shipping Architecture Decisions

These decisions are locked in unless we intentionally revisit them:

- Rust remains the main shipping implementation language
- Python compatibility and analysis surfaces remain supporting assets, not the product core
- provider-specific protocol handling must not leak across the runtime
- plugin and skill systems should extend the engine without modifying core crates every time
- README and planning docs must be updated when architecture meaningfully changes

## Architecture Exit Criteria

We can call the architecture ready for first shipping only when all of the following are true:

- provider abstraction exists and is wired into the runtime
- at least two provider adapters are available
- command and tool surfaces cover the required shipping workflows
- extension surfaces exist for skills and plugins
- release packaging and CI are in place
- documentation matches the actual architecture in the codebase
