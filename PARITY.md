# PARITY GAP ANALYSIS

Scope: repository-level comparison between Anvil's active Rust implementation under `rust/crates/` and a production coding agent reference (Claude Code source, ~1,900 TypeScript files, ~205K lines).

Method: compare feature surfaces, registries, runtime plumbing, tool implementations, and behavioral claims. No external source code is copied into Anvil through this process.

## Executive Summary

Anvil has a working Rust foundation covering:

- API client with SSE streaming and OAuth
- conversation loop with session persistence
- working local tool execution (file ops, shell, search, web)
- MCP stdio bootstrap and routing
- project guidance discovery
- hooks runtime behavior (pre/post tool use)
- permission modes (ReadOnly/WorkspaceWrite/DangerFullAccess)
- Windows-verified test coverage
- 18 built-in tools (core tools production quality, advanced tools stubbed)
- 15 slash commands
- Python compatibility and analysis workspace

Anvil is **not yet at deployment-ready breadth**. The reference codebase provides ~60 production tools, ~100 commands, a streaming async query engine with auto-compaction and error recovery, multi-level permissions with classifiers, full agent orchestration, a plugin system, a bundled skills registry, and a React/Ink terminal UI.

### Critical Gaps (Blocking Deployment)

| Gap | Anvil | Reference | Impact |
|-----|-------|-----------|--------|
| Provider abstraction | Anthropic-only, hardcoded | Single-provider but decoupled design | Cannot support multiple LLMs |
| Streaming query engine | Basic request-response loop | Async generator with auto-compact, recovery, thinking preservation | Unreliable in long sessions |
| Packaging and CI | Local cargo build only | N/A (internal) | Cannot distribute to users |

### High Gaps (Required Before Ship)

| Gap | Anvil | Reference | Impact |
|-----|-------|-----------|--------|
| Tool completeness | 18 tools, ~8 stubbed | ~60 tools, all complete | Missing key workflows |
| Permission depth | 3 basic modes | Multi-level: rules, hooks, classifiers, auto-mode | Cannot safely run in auto mode |
| Command breadth | 15 commands | ~100 commands | Missing planning, review, task commands |
| Agent orchestration | Stub only | Full subagent spawning, worktrees, coordinator | Cannot delegate complex tasks |
| Skill system | Local file loading | Bundled registry, project/user/MCP skills | No reusable workflows |
| Plugin system | Missing entirely | Full loader, lifecycle, tool/command/hook extension | No extensibility |

### Medium Gaps (Important For Quality)

| Gap | Anvil | Reference | Impact |
|-----|-------|-----------|--------|
| MCP integration | Basic stdio only | Full client with OAuth, resources, notifications | Limited external tool support |
| Security analysis | Basic permission check | AST-based bash analysis, destructive command detection | Risk of accidental damage |
| Error recovery | None | Max-tokens recovery, reactive compact, prompt-too-long | Fails on edge cases |
| Context assembly | Basic prompt | Git status, CLAUDE.md, memory, date, tools list | Poor project awareness |
| TUI/rendering | Basic terminal output | React/Ink with streaming, syntax highlighting, panels | Rough user experience |
| File state cache | None | LRU cache with subagent cloning | Redundant file reads |

## Detailed Surface Review

### tools/

Current Rust coverage:

- tool registry exists in `rust/crates/tools/src/lib.rs`
- 18 tools defined: bash, read_file, write_file, edit_file, glob_search, grep_search, web_fetch, web_search, todo_write, skill, agent, tool_search, notebook_edit, sleep, send_user_message, config, structured_output, repl, powershell
- core file/shell/search tools are 85-95% complete
- advanced tools (agent, skill, repl, notebook, todo, config, tool_search, send_message) are 20-40% complete (stubs)
- tool execution wired through runtime conversation loop

Reference coverage:

- ~60 built-in tools all production-complete
- tool framework includes: concurrent execution, progress reporting, result size limiting (50K chars with temp file overflow), deferred tool schema loading, tool aliases
- advanced tools fully implemented: AgentTool (6 subagent types, worktree isolation), SkillTool (frontmatter parsing, conditional activation), REPLTool (persistent Node.js/Python sessions), NotebookEditTool (full Jupyter format), AskUserQuestion (multi-choice interactive prompting), EnterPlanMode/ExitPlanMode, EnterWorktree/ExitWorktree, MCPTool (server tool proxying)

Remaining gaps:

- complete all 8 stubbed tool implementations
- add result size limiting and temp file overflow
- add concurrent tool execution (tools marked concurrency-safe run in parallel)
- add tool progress reporting
- add 6+ new tools: AskUserQuestion, PlanMode tools, Worktree tools, MCPTool, ListResources, ReadResource
- add tool input validation beyond basic schema checks

Status: **partial** - core tools work, advanced tools stubbed, framework features missing

### permissions/

Current Rust coverage:

- 3 permission modes: ReadOnly, WorkspaceWrite, DangerFullAccess
- per-tool minimum permission level
- interactive prompting when permission insufficient
- basic allow/deny from config

Reference coverage:

- multi-level system: tool-level -> rule-based (allow/deny/ask per tool name/pattern) -> hook-based (PreToolUse can deny/modify) -> auto-mode classifier (AST-based safe command detection)
- permission modes: default, auto, plan, bypass
- denial tracking: counts consecutive denials, falls back to prompting after threshold
- auto-mode bash classifier: AST analysis identifies safe commands (git status, ls, cat) vs dangerous (rm, sudo, force push)
- transcript classifier for broader pattern analysis

Remaining gaps:

- implement rule-based allow/deny/ask per tool name/pattern
- implement auto-mode with bash command classifier
- implement denial tracking and fallback-to-prompting
- implement plan mode (read-only tools only)
- implement bypass mode for CI/automation

Status: **basic foundation** - needs significant expansion for safe auto-mode operation

### hooks/

Current Rust coverage:

- hook config parsed and merged in `rust/crates/runtime/src/config.rs`
- pre-tool denial and post-tool feedback paths exist
- hook behavior covered by runtime tests

Reference coverage:

- hook types: PreToolUse, PostToolUse, SessionStart, SessionEnd, Notification
- hooks execute shell commands with tool context as environment variables ($TOOL_NAME, $TOOL_INPUT)
- hooks can approve, deny, or modify tool calls
- hook management commands
- hooks captured at session start to prevent mid-session modification
- 70+ React hooks for UI state management (not applicable to Anvil's terminal approach)

Remaining gaps:

- implement full hook type coverage (SessionStart, SessionEnd, Notification)
- implement environment variable injection ($TOOL_NAME, $TOOL_INPUT, etc.)
- implement hook result parsing (approve/deny/modify)
- implement hook management command (`/hooks`)
- implement hook snapshot at session start

Status: **partial** - runtime behavior present, needs productization

### plugins/

Current Rust coverage:

- no plugin subsystem

Reference coverage:

- plugin manifest format with name, version, tools, commands, hooks
- plugin discovery from user and project directories
- plugin lifecycle: load, enable, disable, reload
- plugin-provided tools registered in tool pool
- plugin-provided commands registered in command registry
- plugin-provided hooks executed alongside built-in hooks
- plugin safety boundaries

Remaining gaps:

- entire subsystem needs implementation

Status: **missing**

### skills/

Current Rust coverage:

- `Skill` tool resolves and reads local `SKILL.md` files
- project guidance discovery in runtime prompt assembly

Reference coverage:

- skill format: markdown with YAML frontmatter (name, description, allowedTools, effort level)
- resolution order: project -> user -> bundled
- bundled skills shipped with product (commit, review-pr, init, etc.)
- MCP skill builders: wrap MCP tools as invocable skills
- conditional activation: skills enabled only when specific file patterns match
- token estimation from frontmatter (full content loaded on invocation)
- skill discovery and search
- `/skill-name` shorthand invocation

Remaining gaps:

- implement bundled skills
- implement YAML frontmatter parsing
- implement resolution order (project -> user -> bundled)
- implement skill discovery and search
- implement MCP skill builders
- implement conditional activation
- implement skill management command (`/skills`)

Status: **basic local loading only** - needs full registry

### cli/ and commands

Current Rust coverage:

- 15 slash commands: help, status, compact, clear, model, permissions, cost, resume, config, memory, init, diff, version, export, session
- command registry in `rust/crates/commands/src/lib.rs`
- REPL with streaming output, multiline input
- one-shot prompt mode

Reference coverage:

- ~100 slash commands covering: session management, planning, review, git operations, task management, agent management, MCP management, skill/plugin management, diagnostics, auth, theme, accessibility
- command types: LocalCommand (handler), PromptCommand (inject prompt), LocalJSXCommand (render UI)
- dynamic command registration from skills and plugins
- command autocompletion
- lazy loading for heavy commands

Remaining gaps:

- add ~15-20 essential commands: plan, review, commit, pr, issue, tasks, agents, mcp, skills, plugins, hooks, doctor, login/logout, tree, debug, copy, add-dir
- implement command types (currently all local handlers)
- implement dynamic registration from skills/plugins
- implement command autocompletion
- implement command argument parsing with help text

Status: **functional local core** - needs significant expansion

### runtime / conversation engine

Current Rust coverage:

- conversation loop in `rust/crates/runtime/src/conversation.rs`
- session persistence in `rust/crates/runtime/src/session.rs`
- basic compaction
- usage tracking
- hooks execution
- prompt assembly with project guidance

Reference coverage:

- streaming async generator query loop with multiple recovery mechanisms
- auto-compact: triggers when approaching token limit
- reactive compact: handles prompt_too_long errors
- micro-compact: for cache editing
- snip compact: feature-gated history snipping
- max-output-tokens recovery: retries up to 3 times on truncation
- thinking block preservation across turns
- StreamingToolExecutor for concurrent tool execution
- token budget tracking per session and per turn
- file state caching with LRU bounds and clone-for-subagents
- turn count tracking

Remaining gaps:

- rewrite to streaming async generator pattern
- implement auto-compact with token threshold detection
- implement reactive compact for error recovery
- implement max-output-tokens recovery loop
- implement thinking preservation
- implement concurrent tool execution
- implement file state caching
- implement turn-level token budgeting

Status: **basic loop works** - needs major upgrade for production reliability

### api / providers

Current Rust coverage:

- Anthropic API client with SSE streaming
- OAuth authentication with PKCE
- API key authentication
- retry logic with exponential backoff
- message request/response types

Reference coverage:

- single-provider but with clear event contract: TextDelta, ToolUseStart, ToolUseInputDelta, ToolUseStop, ToolResult, Usage, MessageStop, Error
- usage normalization across provider differences
- tool-call extraction from streaming events
- thinking/redacted_thinking block handling

Remaining gaps:

- define provider trait and ProviderEvent contract
- move Anthropic logic behind provider interface
- implement OpenAI-compatible adapter
- define Gemini adapter path
- implement provider/model selection from config and CLI
- normalize usage and finish reasons across providers

Status: **Anthropic works** - no abstraction layer exists

### services/

Current Rust coverage:

- API client (rust/crates/api)
- OAuth (rust/crates/runtime/src/oauth.rs)
- MCP config and stdio (runtime)
- usage accounting (runtime)

Reference coverage:

- extensive service ecosystem: API, MCP (client/server/auth/resources), compact (auto/reactive/micro/snip), analytics, voice, notifications, rate limits, VCR, token estimation, diagnostics, sleep prevention
- tool orchestration service (StreamingToolExecutor)
- skill search service
- session memory service

Remaining gaps:

- tool orchestration service (concurrent execution manager)
- compact service (multiple compaction strategies)
- notification service (terminal notifications on completion)
- diagnostic service (system health checks)
- rate limit handling service

Status: **core foundation exists** - broader service ecosystem missing

### context assembly

Current Rust coverage:

- basic system prompt builder in `rust/crates/runtime/src/prompt.rs`
- project guidance discovery (CLAUDE.md files)
- OS and platform information

Reference coverage:

- full context assembly: git status snapshot (branch, commits, status, user), CLAUDE.md discovery (walk directory tree), memory files from ~/.claude/memory/, current date/time, available tools with descriptions, available commands, cache-breaking injection
- memoized per session with invalidation
- filtered to relevant directories

Remaining gaps:

- add git status snapshot to system prompt
- add memory file discovery
- add current date/time
- add available tools and commands list
- add memoization with invalidation
- improve CLAUDE.md/ANVIL.md directory walk

Status: **basic** - needs expansion for good project awareness

## Quantitative Summary

| Metric | Anvil | Reference | Coverage |
|--------|-------|-----------|----------|
| Tools (implemented) | 10 complete | ~60 | 17% |
| Tools (total defined) | 18 | ~60 | 30% |
| Commands | 15 | ~100 | 15% |
| Permission levels | 3 modes | 4 modes + rules + hooks + classifiers | 25% |
| Compaction strategies | 1 (basic) | 4 (auto, reactive, micro, snip) | 25% |
| Provider adapters | 1 (Anthropic) | 1 (but abstracted) | N/A |
| MCP transports | 1 (stdio) | 3 (stdio, HTTP/SSE, WebSocket) | 33% |
| Rust test coverage | Green | N/A | Good |
| Lines of Rust | ~18K | ~205K TypeScript | N/A |

## Current Assessment

### What can be called strong:

- Rust foundation and type safety
- test stability (workspace green)
- core file/shell/search tools
- session persistence and resume
- permission mode framework
- OAuth and API key auth
- project guidance discovery

### What is blocking deployment:

1. **Provider abstraction** - cannot support multiple LLMs
2. **Streaming engine** - conversation loop needs reliability upgrades
3. **Tool completeness** - 8 stubbed tools, 6+ missing tools
4. **Permission depth** - cannot safely run in auto mode
5. **Command breadth** - missing essential workflows
6. **Plugin system** - no extensibility
7. **Skills registry** - no reusable workflows
8. **Packaging** - no install path for users
9. **CI pipeline** - no automated quality gates

### Priority order for closing gaps:

1. Provider abstraction (unblocks multi-LLM)
2. Streaming engine (unblocks reliability)
3. Complete existing tools (highest user impact)
4. Permission upgrade (unblocks auto mode)
5. CI pipeline (catch regressions early)
6. Command expansion (daily workflow coverage)
7. Skills registry (community contribution)
8. Plugin system (extensibility)
9. Packaging (distribution)

## Working Rule

Update this document whenever Anvil materially closes or opens a feature-coverage gap. When a surface moves from missing to partial or partial to strong, record the change and date.
