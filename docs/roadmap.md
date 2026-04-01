# Anvil Roadmap

## Phase 0: Brand and Repo Foundation

Goal: make Anvil a standalone private-ready product base.

Deliverables:

- standalone local Git repo
- Anvil branding in top-level docs
- private GitHub repo as future `origin`
- old `claw-code` clone retained as upstream sync source
- architecture and roadmap docs committed

Exit criteria:

- repo identity is Anvil, not claw-code
- roadmap and architecture are documented
- upstream sync strategy is clear

## Phase 1: Runtime Core Hardening

Goal: make the Rust runtime stable enough for daily local use.

Deliverables:

- polished conversation loop
- predictable tool execution path
- clean permission model
- better session resume/export
- compaction and usage tracking made trustworthy
- structured output cleaned up for automation

Exit criteria:

- one-shot and REPL modes are reliable
- tool-capable JSON mode is clean enough for scripting
- session restore and usage accounting are dependable

## Phase 2: Claude-Like Core Parity

Goal: close the highest-value capability gaps from the local Claude analysis.

Priority targets:

1. commands
2. tools
3. CLI/headless transports
4. skills
5. plugins

Concrete deliverables:

- `/review`, `/plan`, `/tasks`, `/mcp`, `/plugins`, `/skills`
- richer tool registry and execution orchestration
- hook execution pipeline
- better headless and structured transport behavior
- local and bundled skill loading

Exit criteria:

- Anvil feels like a serious coding-agent CLI, not just a model wrapper
- the missing features in `PARITY.md` are reduced in the highest-impact areas

## Phase 3: Provider Abstraction

Goal: separate the Anvil engine from any one model vendor.

Deliverables:

- internal provider trait/interface
- Anthropic adapter
- OpenAI-compatible adapter
- OpenAI Responses adapter
- Gemini adapter
- config and CLI selection for provider/model

Exit criteria:

- switching providers does not require changing the core runtime
- usage, streaming, and tool calls are normalized across adapters

## Phase 4: Extensions and Workflows

Goal: make Anvil extensible and workflow-aware.

Deliverables:

- skill registry
- plugin registry
- project and user skill directories
- plugin command and tool injection
- MCP-derived skills and tool discovery

Exit criteria:

- new capabilities can be added without patching the engine core
- project-specific workflows become first-class

## Phase 5: Agent Orchestration

Goal: support larger coding jobs and multi-step execution.

Deliverables:

- planner/executor/verifier loops
- sub-agent spawning
- background tasks
- better task views and summaries
- optional remote execution hooks

Exit criteria:

- Anvil can reliably handle long-running coding tasks
- planning and verification become built-in product features

## Phase 6: Specialization

Goal: tailor Anvil to your real workflows.

Potential specialization tracks:

- GitHub-heavy coding automation
- data and Power BI workflows
- review and audit helpers
- notebook and spreadsheet workflows
- Windows and PowerShell-first tool ergonomics

Exit criteria:

- Anvil is not just Claude-like
- it is better aligned with your day-to-day work than generic agent CLIs
