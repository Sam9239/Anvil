# Anvil Roadmap

## Release Objective

Ship Anvil as a deployment-ready, terminal-first coding agent CLI that can operate as a local coding assistant comparable to Claude Code. The agent must support interactive REPL and headless modes, multi-provider LLM backends, a complete tool system with permission-aware execution, extensibility through skills and plugins, and installable packaging for Windows, macOS, and Linux.

## Current Phase Snapshot

As of 2026-04-02:

- Phase 0: complete
- Phase 1: complete
- Phase 2A: complete (provider abstraction, 4 adapters, model registry)
- Phase 2B: complete (live CLI/runtime provider routing, env-based provider auth)
- Phase 2C-2D: next priority (streaming engine, context assembly)
- Phase 3: required before ship
- Phase 4: required before ship
- Phase 5: required before ship
- Phase 6: required before ship
- Phase 7: follow-on growth after first ship

## Gap Summary From Reference Analysis

Anvil was compared against a production coding agent reference (Claude Code src, ~1,900 TypeScript files, ~205K lines). The following critical gaps block deployment readiness:

| Area | Anvil Status | Reference Status | Gap Severity |
|------|-------------|------------------|--------------|
| Provider abstraction | Phase 2A complete (4 adapters, 13 models) | Single-provider but decoupled | **Closed** |
| Streaming query loop | Basic conversation loop | Async generator with auto-compact, recovery, thinking preservation | **Critical** |
| Tool breadth | ~18 tools, many stubbed | ~60 tools, all production-grade | **High** |
| Permission depth | 3 basic modes | Multi-level: rules, hooks, classifiers, auto-mode | **High** |
| Agent orchestration | Stub only | Full subagent spawning, worktrees, coordinator mode | **High** |
| Skill system | Local file loading | Bundled registry, project/user skills, MCP skills, frontmatter | **High** |
| Plugin system | Missing entirely | Full loader, lifecycle, marketplace | **High** |
| Command breadth | 15 commands | ~100 commands | **High** |
| MCP integration | Basic stdio | Full client with OAuth, resources, channel notifications | **Medium** |
| Security/sandboxing | Basic permission modes | AST-based bash analysis, destructive command detection, path validation | **Medium** |
| TUI/rendering | Basic terminal output | Full React/Ink TUI with streaming, syntax highlighting | **Medium** |
| Error recovery | None | Max-tokens recovery, reactive compact, prompt-too-long handling | **Medium** |
| Context assembly | Basic prompt building | Git status, CLAUDE.md discovery, memory files, date injection | **Medium** |
| File state caching | None | LRU cache with clone-for-subagents | **Low** |
| Packaging/CI | Local cargo build only | Not applicable (reference is internal) | **Critical** |

---

## Phase 0: Product Identity And Repo Foundation

Status: complete

Delivered:

- standalone Anvil repository
- Anvil branding at the product level
- private GitHub origin (now public)
- clear separation between shipping implementation and coverage surfaces
- initial architecture and roadmap docs

## Phase 1: Runtime Hardening And Test Stability

Status: complete

Delivered:

- Rust CLI rebrand to `anvil`
- green Rust workspace tests
- Windows-focused shell, PowerShell, OAuth, MCP stdio, prompt, and tool test stabilization
- improved local verification confidence for the current engine

## Phase 2: Provider Abstraction And Streaming Engine

Status: Phase 2A complete, Phase 2B-2D next

This phase transforms Anvil from an Anthropic-only client into a provider-agnostic engine. It also upgrades the conversation loop from basic request-response to a production-grade streaming engine.

### 2A: Provider Interface And Adapters

Status: **complete** (2026-04-02)

Delivered:

- `Provider` trait in `rust/crates/providers/` crate:
  ```rust
  pub trait Provider {
      fn stream_message(&mut self, request: &ProviderRequest)
          -> Result<Vec<ProviderEvent>, ProviderError>;
      fn capabilities(&self) -> ProviderCapabilities;
      fn model_id(&self) -> &str;
      fn provider_kind(&self) -> ProviderKind;
  }
  ```
- `ProviderEvent` enum: `TextDelta`, `ToolUseStart`, `ToolUseInputDelta`, `ToolUseComplete`, `Usage`, `MessageStop`, `Error`
- `ProviderCapabilities`: max context tokens, max output tokens, tool support, vision, streaming
- `ProviderError`: typed errors with `Auth`, `Network`, `RateLimited`, `ContextTooLong`, `Api`, `Parse`, `Other`
- `StopReason` enum: `EndTurn`, `MaxTokens`, `ToolUse`, `StopSequence`
- Provider-agnostic request types: `ProviderRequest`, `ChatMessage`, `ChatContent`, `ChatRole`, `ToolDefinition`
- `resolve_provider_model()`: parses `"openai:gpt-4.1"` or auto-detects from model name prefix
- 4 provider adapters:
  - **AnthropicProvider**: wraps existing `api` crate, full SSE stream parsing, error mapping
  - **OpenAiProvider**: OpenAI Chat Completions API with streaming SSE and JSON modes, tool_call accumulation
  - **OpenAiProvider::new_xai()**: xAI Grok via OpenAI-compatible endpoint
  - **GeminiProvider**: Google Generative Language API with `generateContent`/`streamGenerateContent`
- `ModelRegistry` with 13 models across 4 providers (Anthropic, OpenAI, Google, xAI):
  - Display name, API model ID, provider kind, tier (Fast/Balanced/Powerful)
  - Max context tokens, default max output tokens, tool/vision support
  - Per-model pricing for cost estimation
  - Lookup by ID, alias, or provider; default model per provider
- `pricing_for_model()` and `max_tokens_for_model()` utility functions
- 25 unit tests passing, clippy clean, fmt clean

### 2B: CLI Provider Integration

Status: complete (2026-04-02)

Delivered:

- wired `providers` crate into the live `anvil` CLI runtime
- replaced `AnthropicRuntimeClient` with a provider-based client that dispatches to the correct adapter
- wired provider API keys from environment variables per provider
- enabled `--model provider:model-name` and provider-prefixed `/model` switching

Still open in this area:

- update `/cost` command to use per-model pricing from the provider registry
- add first-run provider setup and config-backed provider defaults

### 2C: Streaming Query Engine

Required deliverables:

- rewrite `conversation.rs` to use async generator pattern (Rust `Stream` trait):
  ```
  fn query(params) -> impl Stream<Item = QueryEvent>
  ```
- implement turn-based loop: API request -> extract tool calls -> execute tools concurrently -> yield results -> continue or stop
- add auto-compact: track token usage per turn, trigger compaction when approaching context limit
- add reactive compact: handle `prompt_too_long` errors by compacting and retrying
- add max-output-tokens recovery: retry up to 3 times when response is truncated
- add thinking block preservation across turns
- implement `StreamingToolExecutor`: run independent tools concurrently, sequential when dependencies exist
- add token budget tracking per session and per turn

### 2D: Context Assembly

Required deliverables:

- system prompt builder that assembles:
  - OS and platform information
  - current working directory and git status (branch, recent commits, status)
  - CLAUDE.md / ANVIL.md project instruction discovery (walk up directory tree)
  - memory file discovery from `~/.anvil/memory/`
  - current date/time
  - available tools list with descriptions
  - available commands list
- cache system context per session (invalidate on directory change)
- support `appendSystemPrompt` and `customSystemPrompt` overrides

Exit criteria:

- runtime no longer assumes one provider
- providers can be added by implementing one trait
- conversation loop streams events and handles errors gracefully
- auto-compaction prevents context overflow
- system prompt includes full project context

---

## Phase 3: Tool System Completion

Status: required before ship

This phase brings the tool system from MVP (~18 partially implemented tools) to production grade (~30+ fully functional tools).

### 3A: Core Tool Framework Hardening

Required deliverables:

- complete the `Tool` trait implementation:
  ```
  Tool {
    fn name() -> &str
    fn description() -> String
    fn input_schema() -> JsonSchema
    fn is_read_only(input) -> bool
    fn is_destructive(input) -> bool
    fn check_permissions(input, context) -> PermissionResult
    fn execute(input, context) -> ToolResult
    fn validate_input(input) -> ValidationResult
  }
  ```
- implement tool result size limiting: max 50K chars per result, overflow to temp file with preview
- implement concurrent tool execution: tools marked `is_concurrency_safe` run in parallel
- implement tool input validation with Zod-equivalent JSON schema validation
- add tool progress reporting for long-running tools

### 3B: Complete Existing Tool Implementations

Required deliverables (finish all stubbed tools):

- **BashTool** (90% -> 100%): add AST-based command security analysis, destructive command detection (rm -rf, git push --force, etc.), sed injection prevention, background process management, proper timeout handling
- **FileReadTool** (95% -> 100%): add PDF reading support (page ranges), image file detection, Jupyter notebook rendering, binary file detection, encoding detection
- **FileEditTool** (95% -> 100%): add unique match validation (fail if old_string matches multiple locations), file state tracking (detect external modifications), line ending preservation
- **FileWriteTool** (95% -> 100%): add dangerous file protection (.env, credentials, .gitconfig), path traversal prevention
- **GlobTool** (95% -> 100%): add result limiting (max 100 files), modification time sorting
- **GrepTool** (95% -> 100%): add multiline mode, context lines (-A/-B/-C), offset/limit pagination, output modes (content, files_with_matches, count)
- **WebFetchTool** (85% -> 100%): add HTML-to-markdown conversion, content size limiting, redirect following, timeout handling
- **WebSearchTool** (80% -> 100%): integrate with search API provider, add domain allow/block lists
- **AgentTool** (40% -> 100%): implement full subagent spawning with isolated message history, support agent types (general-purpose, explore, plan), implement worktree isolation via `git worktree`, add memory state management for agents
- **SkillTool** (20% -> 100%): implement full skill resolution (project -> user -> bundled), frontmatter parsing, argument substitution, token estimation
- **TodoWrite** (30% -> 100%): implement full task state management (pending/in_progress/completed), task persistence, progress display
- **NotebookEdit** (25% -> 100%): implement cell insert/edit/delete operations, output clearing, Jupyter format compliance
- **REPL** (25% -> 100%): implement Node.js and Python REPL execution with persistent session, output capture, timeout
- **SendMessageTool** (20% -> 100%): implement inter-agent messaging via mailbox pattern
- **ToolSearch** (30% -> 100%): implement deferred tool schema loading, keyword and exact-match search
- **ConfigTool** (30% -> 100%): implement settings.json read/write with validation

### 3C: New Tools Required For Deployment

Required deliverables (new tools to implement):

- **AskUserQuestion**: interactive multi-choice prompting during tool execution, support single-select and multi-select, option previews
- **EnterPlanMode / ExitPlanMode**: plan mode state management for complex task planning before execution
- **EnterWorktree / ExitWorktree**: git worktree creation and cleanup for isolated agent work
- **MCPTool**: execute tools from connected MCP servers, handle MCP tool schemas, proxy tool calls
- **ListResources / ReadResource**: MCP resource browsing and reading
- **PowerShell** (70% -> 100%): complete Windows PowerShell execution with proper encoding and error handling

### 3D: Permission System Upgrade

Required deliverables:

- implement multi-level permission model:
  - **Level 1 - Tool level**: each tool declares `is_read_only()`, `is_destructive()`, minimum permission
  - **Level 2 - Rule-based**: allow/deny/ask rules per tool name/pattern from config
  - **Level 3 - Hook-based**: PreToolUse hooks can deny or modify tool calls
  - **Level 4 - Auto-mode classifier**: pattern matching for safe bash commands (git status, ls, cat, etc.) that can auto-approve in auto mode
- implement permission modes: `default` (ask for dangerous), `auto` (classify and auto-approve safe), `plan` (read-only tools only), `bypass` (approve all)
- implement denial tracking: count consecutive denials per tool, fall back to prompting after threshold
- implement `--dangerously-skip-permissions` flag for CI/automation

Exit criteria:

- all 18 existing tools fully implemented (no stubs)
- at least 6 new tools implemented
- permission system supports multi-level decisions
- concurrent tool execution works
- tool results respect size limits

---

## Phase 4: Command Surface Expansion

Status: required before ship

This phase expands the slash command surface from 15 commands to the full set needed for daily engineering workflows.

### 4A: Planning And Review Commands

Required deliverables:

- `/plan` - enter plan mode, create implementation plans before executing
- `/review` - review recent changes, generate summaries
- `/diff` - show git diff of current changes (improve existing)
- `/commit` - stage and commit changes with generated message
- `/pr` - create pull request with generated title and description
- `/issue` - create or view GitHub issues

### 4B: Task And Agent Commands

Required deliverables:

- `/tasks` - list and manage background tasks
- `/agents` - list and manage running subagents
- `/tree` - show project directory tree
- `/debug` - debug tool call inspection mode

### 4C: Extension Management Commands

Required deliverables:

- `/mcp` - manage MCP server connections (list, add, remove, reconnect)
- `/skills` - list, search, and manage loaded skills
- `/plugins` - list, enable, disable, reload plugins
- `/hooks` - list and manage hook configurations

### 4D: Utility Commands

Required deliverables:

- `/doctor` - system diagnostic check (verify API keys, check tools, test connectivity)
- `/login` / `/logout` - authentication management
- `/theme` - terminal color theme selection
- `/bug` - file a bug report with system context
- `/copy` - copy last response to clipboard
- `/add-dir` - add additional directories to context

### 4E: Command Infrastructure

Required deliverables:

- implement command type system: `LocalCommand` (runs handler), `PromptCommand` (injects prompt to model), `LocalJSXCommand` equivalent (runs and renders)
- implement command argument parsing with help text
- implement command autocompletion in REPL
- implement dynamic command registration from skills and plugins

Exit criteria:

- at least 30 slash commands available
- planning, review, task, MCP, and extension commands all functional
- command autocompletion works in REPL
- dynamic command registration from extensions works

---

## Phase 5: Skills, Plugins, And Extension Surfaces

Status: required before ship

### 5A: Skills Registry

Required deliverables:

- define skill format: markdown files with YAML frontmatter
  ```yaml
  ---
  name: commit
  description: Stage and commit changes
  allowedTools: [Bash, Read, Grep, Glob]
  ---
  [skill prompt content]
  ```
- implement skill resolution order: project `.anvil/skills/` -> user `~/.anvil/skills/` -> bundled
- implement bundled skills: commit, review-pr, init, simplify, and other common workflows
- implement skill discovery: list available skills, search by keyword, show skill details
- implement MCP skill builders: wrap MCP tools as invocable skills
- implement `/skill-name` shorthand invocation (e.g., `/commit` invokes commit skill)
- implement conditional skills: activate only when specific file patterns match

### 5B: Plugin System

Required deliverables:

- define plugin manifest format:
  ```json
  {
    "name": "my-plugin",
    "version": "1.0.0",
    "tools": [...],
    "commands": [...],
    "hooks": [...]
  }
  ```
- implement plugin discovery: scan `~/.anvil/plugins/` and project `.anvil/plugins/`
- implement plugin lifecycle: load, enable, disable, reload, uninstall
- implement plugin-provided tools: plugins can register new tools with schemas
- implement plugin-provided commands: plugins can register slash commands
- implement plugin-provided hooks: plugins can register PreToolUse/PostToolUse hooks
- implement plugin safety: sandboxed execution, permission boundaries, no access to other plugin state

### 5C: Hook System Productization

Required deliverables:

- implement hook types: `PreToolUse`, `PostToolUse`, `SessionStart`, `SessionEnd`, `Notification`
- implement hook configuration in settings.json:
  ```json
  {
    "hooks": {
      "PreToolUse": [
        { "matcher": "Bash", "command": "echo $TOOL_INPUT | security-check" }
      ]
    }
  }
  ```
- implement hook execution: run shell commands with tool context as environment variables
- implement hook result handling: hooks can approve, deny, or modify tool calls
- implement `/hooks` command for listing and managing hooks

Exit criteria:

- skills can be loaded from project, user, and bundled directories
- plugins can provide tools, commands, and hooks
- hook system supports custom security policies
- extensions are first-class product concepts

---

## Phase 6: Packaging, CI, QA, And Release Readiness

Status: required before ship

### 6A: Build And Packaging

Required deliverables:

- release builds for: Windows x64, macOS ARM64, macOS x64, Linux x64, Linux ARM64
- choose distribution strategy: GitHub Releases with prebuilt binaries
- implement install script: `curl -fsSL https://anvil.dev/install.sh | sh` (or similar)
- implement Windows installer: MSI or standalone exe with PATH setup
- implement `cargo install anvil` support
- implement self-update: `anvil update` checks for and installs latest version
- implement `anvil doctor` diagnostic that verifies installation health

### 6B: CI Pipeline

Required deliverables:

- GitHub Actions workflow for:
  - `cargo fmt --check` on every PR
  - `cargo clippy --workspace --all-targets -- -D warnings` on every PR
  - `cargo test --workspace` on every PR
  - cross-platform matrix: Windows, macOS, Linux
  - release build and artifact upload on tag push
  - binary smoke test: run `anvil --version`, `anvil --help`, basic prompt test
- implement PR checks: all CI must pass before merge
- implement release workflow: tag -> build -> test -> publish -> release notes

### 6C: Smoke Tests And Integration Tests

Required deliverables:

- binary smoke tests: version, help, config, init, doctor
- REPL smoke tests: start session, send prompt, receive response, slash commands
- tool smoke tests: bash execution, file read/write/edit, glob, grep
- session smoke tests: create, persist, resume, export, compact
- provider smoke tests: connect to each provider adapter, send basic prompt
- permission smoke tests: verify tool blocking, auto-mode classification
- MCP smoke tests: connect to stdio server, list tools, execute tool

### 6D: First-Run Experience

Required deliverables:

- `anvil init` creates project configuration (`.anvil/` directory, initial settings)
- first-run API key setup: prompt for provider and API key, save to config
- first-run diagnostics: check system requirements, verify connectivity
- welcome message with quick-start guidance
- `anvil --help` with clear usage examples

### 6E: Documentation

Required deliverables:

- installation guide for each platform
- quick-start tutorial (5-minute getting started)
- configuration reference (all settings, environment variables)
- tool reference (each tool with examples)
- command reference (each command with examples)
- provider setup guide (each supported provider)
- skill authoring guide
- plugin authoring guide
- hook configuration guide
- troubleshooting guide

Exit criteria:

- new user can install Anvil in under 2 minutes on any supported platform
- CI enforces code quality on every PR
- smoke tests cover all critical user paths
- documentation covers installation through advanced configuration

---

## Phase 7: Expanded Product Breadth

Status: follow-on after first ship

Potential deliverables:

- richer task orchestration: background tasks, task views, task notifications
- coordinator mode: multi-agent orchestration with master-worker pattern
- remote sessions: structured transport for headless/SDK operation
- voice input integration
- IDE extensions: VS Code and JetBrains plugins
- team features: shared sessions, team memory
- analytics and telemetry (opt-in)
- domain-specific workflow packs
- enhanced TUI: full terminal UI with panels, progress bars, syntax-highlighted output
- MCP server mode: Anvil acts as an MCP server for other tools

---

## Implementation Priority Order

For maximum deployment velocity, work in this order within each phase:

1. ~~**Phase 2A** (Provider interface + adapters): complete~~
2. **Phase 2B** (CLI provider integration): wire providers into CLI
3. **Phase 2C** (Streaming engine): required for reliable operation
3. **Phase 3B** (Complete existing tools): highest user-facing impact
4. **Phase 3D** (Permission upgrade): required for safe operation
5. **Phase 6B** (CI pipeline): should be set up early to catch regressions
6. **Phase 4A** (Planning/review commands): most-requested workflows
7. **Phase 3C** (New tools): fills functional gaps
8. **Phase 5A** (Skills): enables community contribution
9. **Phase 4B-4E** (Remaining commands): rounds out experience
10. **Phase 5B** (Plugins): enables extensibility
11. **Phase 6A** (Packaging): required for distribution
12. **Phase 6C-6E** (Tests, first-run, docs): required for release

## Estimated Effort Per Phase

| Phase | Effort | Crates Affected |
|-------|--------|-----------------|
| Phase 2: Provider + Streaming | Large | New `providers` crate, major `runtime` rewrite, `api` refactor |
| Phase 3: Tool Completion | Large | `tools` crate (complete all stubs), `runtime` (permissions) |
| Phase 4: Command Expansion | Medium | `commands` crate, `rusty-claude-cli` |
| Phase 5: Extensions | Medium | New `skills`, `plugins` crates |
| Phase 6: Release | Medium | CI config, build scripts, docs |

## Roadmap Rules

These rules apply every time the roadmap changes:

- a phase is not complete until verification is green
- a phase is not complete until the relevant docs are updated
- README status must reflect the latest major milestone
- `PARITY.md` must be updated when coverage claims change materially
- shipping-plan.md must stay synchronized with this roadmap
