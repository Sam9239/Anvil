# Anvil Target Architecture

## Design Goal

Build a terminal-first coding agent with Claude-like capabilities while keeping the core runtime independent from any single model vendor.

## Core Principle

The engine is Anvil.

The model is a provider plugged into Anvil.

That separation lets us preserve a consistent user experience while swapping between providers based on cost, capability, latency, or availability.

## Runtime Layers

### 1. Product Shell

User-facing surfaces:

- interactive REPL
- one-shot prompt mode
- JSON and NDJSON structured output
- session resume/export
- slash commands
- config inspection and doctor flows

Suggested home:

- `rust/crates/anvil-cli`
- later `rust/crates/transports` for SDK/editor/remote integrations

### 2. Engine Runtime

The main orchestration layer:

- system prompt assembly
- conversation state
- model streaming
- tool call loop
- permission checks
- compaction and token budgets
- transcript/session persistence
- hook execution
- task lifecycle

Suggested home:

- `rust/crates/runtime`

### 3. Tool System

A unified tool registry with typed schemas, policies, and execution adapters:

- bash
- PowerShell
- file read/write/edit
- grep/glob/search
- web fetch/search
- notebook support
- MCP tool bridging
- git and repo workflows
- task and sub-agent tools
- planning/review helpers

Suggested home:

- `rust/crates/tools`

### 4. Command Layer

Slash command registry and handlers:

- `/help`
- `/status`
- `/compact`
- `/clear`
- `/model`
- `/permissions`
- `/config`
- `/memory`
- `/review`
- `/plan`
- `/tasks`
- `/mcp`
- `/plugins`
- `/skills`

Suggested home:

- `rust/crates/commands`

### 5. Provider Adapters

Provider-facing model adapters behind one internal interface.

Target interface responsibilities:

- send request
- stream output events
- parse tool-use blocks
- normalize usage/cost data
- expose provider capabilities

Initial provider plan:

1. `AnthropicAdapter`
2. `OpenAICompatibleAdapter`
3. `OpenAIResponsesAdapter`
4. `GeminiAdapter`

Suggested home:

- `rust/crates/providers`

### 6. Skills and Plugins

Extension surfaces, not core runtime hacks.

Skills:

- bundled skills
- local project skills
- user-level skills
- MCP-derived skills
- ranking and discovery

Plugins:

- manifest loading
- install/enable/disable
- command registration
- tool registration
- hook registration
- optional MCP integration

Suggested home:

- `rust/crates/skills`
- `rust/crates/plugins`

### 7. Agent Orchestration

Multi-step and multi-agent execution:

- planner
- executor
- verifier
- background jobs
- local sub-agents
- remote task runners

Suggested home:

- `rust/crates/agents`

### 8. Storage and Config

- session store
- config hierarchy
- local memory
- project memory
- telemetry and traces
- cached tool outputs

Suggested home:

- `rust/crates/storage`

## Provider Model

Anvil should support two operating modes.

### Mode A: Single Provider

One configured provider powers the whole session.

Good for:

- simple setups
- predictable behavior
- lower operational complexity

### Mode B: Routed Provider Selection

The engine chooses a provider by task type or user preference.

Examples:

- Anthropic for long-form coding turns
- OpenAI-compatible endpoint for structured tool calling
- Gemini for large-context analysis

This routing must be explicit and inspectable. Users should always know which provider handled a turn.

## Recommended Internal Contract

Anvil should standardize on one internal event stream:

- `TextDelta`
- `ToolUseStart`
- `ToolUseInputDelta`
- `ToolUseStop`
- `ToolResult`
- `Usage`
- `MessageStop`
- `Error`

Every provider adapter maps its native protocol into this contract.

That gives the CLI, session layer, storage layer, and tool loop one stable runtime shape.

## MVP End-State

To count as Claude-like enough for daily use, Anvil should provide:

- interactive coding REPL
- one-shot prompt mode
- shell and file tools
- permission modes
- session persistence
- compaction
- MCP support
- slash commands for status/config/review/plan
- local skills
- at least two model-provider adapters

## Important Non-Goals

- chasing one-to-one parity with every UI detail from the source snapshot
- tightly coupling the product to a single model vendor
- inheriting risky defaults without review
- preserving old claw-code branding or public fork identity
