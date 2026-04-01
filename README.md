# Anvil

Anvil is a terminal-first coding agent and automation CLI for software engineering work. It is built around a Rust engine that owns conversation state, tool execution, permissions, session memory, configuration, and workflow orchestration. Anvil connects to LLM providers through a provider adapter interface, making it model-agnostic by design.

Anvil is meant to ship as its own product: a local CLI coding agent you install and run in your terminal, similar in spirit to Claude Code but built from the ground up in Rust with multi-provider support.

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Tools](#tools)
- [Configuration](#configuration)
- [Permissions](#permissions)
- [Skills](#skills)
- [MCP Support](#mcp-support)
- [Sessions](#sessions)
- [Development](#development)
- [Project Status](#project-status)
- [Roadmap](#roadmap)
- [Repository Layout](#repository-layout)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

```bash
# Build from source (requires Rust toolchain)
cd rust
cargo build --release

# Run interactive REPL
./target/release/anvil

# Run with a one-shot prompt
./target/release/anvil "explain this codebase"

# Resume a previous session
./target/release/anvil --resume

# Select a model
./target/release/anvil --model anthropic:claude-sonnet-4-20250514
```

## Features

### Working Today

| Feature | Description |
|---------|-------------|
| **Interactive REPL** | Full terminal REPL with slash commands, multiline input, streaming output |
| **One-Shot Prompt Mode** | Pass a prompt as argument for non-interactive use in scripts and CI |
| **Session Persistence** | Sessions auto-save and can be resumed, exported, or compacted |
| **Model Selection** | Switch models mid-session via `/model` or CLI flag |
| **Permission Modes** | ReadOnly, WorkspaceWrite, and DangerFullAccess permission levels |
| **Shell Execution** | Run bash and PowerShell commands with output capture and timeout |
| **File Operations** | Read, write, and edit files with conflict detection |
| **Code Search** | Glob pattern matching and ripgrep-powered content search |
| **Web Tools** | Fetch URLs and search the web from within agent context |
| **Notebook Editing** | Edit Jupyter notebook cells programmatically |
| **Todo Tracking** | Create and manage task lists during sessions |
| **Local Skills** | Load custom skill definitions from SKILL.md files |
| **Sub-Agent Execution** | Spawn child agents for parallel or isolated work |
| **MCP Bootstrap** | Connect to MCP servers via stdio transport |
| **Authentication** | OAuth and API-key based auth for Anthropic |
| **Usage Tracking** | Token counting and cost estimation per session |
| **Project Instructions** | Discover and use CLAUDE.md/ANVIL.md project guidance files |
| **Hook System** | Pre/post tool execution hooks for custom policies |
| **Cost Reporting** | `/cost` command shows session token usage and estimated cost |
| **Session Export** | Export sessions as JSON for analysis or sharing |

### Planned (Pre-Release)

| Feature | Phase | Priority |
|---------|-------|----------|
| Provider abstraction (multi-LLM) | Phase 2 | Critical |
| OpenAI-compatible provider | Phase 2 | Critical |
| Streaming query engine with auto-compact | Phase 2 | Critical |
| Full context assembly (git status, project docs) | Phase 2 | Critical |
| Complete all tool implementations | Phase 3 | High |
| Multi-level permission system | Phase 3 | High |
| Planning and review commands | Phase 4 | High |
| Bundled skills registry | Phase 5 | High |
| Plugin system | Phase 5 | High |
| Cross-platform packaging | Phase 6 | High |
| CI/CD pipeline | Phase 6 | High |

## Architecture

Anvil is organized into layered crates within a Rust workspace:

```
+------------------------------------------------------------------+
|                        Product Shell                              |
|  CLI, REPL, prompt mode, output rendering, command dispatch       |
|  Crate: rusty-claude-cli                                         |
+------------------------------------------------------------------+
|                        Engine Runtime                             |
|  Conversation loop, session state, permissions, hooks,            |
|  prompt assembly, compaction, usage tracking, MCP plumbing        |
|  Crate: runtime                                                  |
+------------------------------------------------------------------+
|                        Tool System                                |
|  Tool registry, schemas, validation, execution, permission-aware  |
|  access, built-in tools, plugin/MCP tool injection                |
|  Crate: tools                                                    |
+------------------------------------------------------------------+
|                     Provider Adapters                             |
|  Vendor-specific streaming, tool-call normalization, usage        |
|  mapping behind one internal ProviderEvent contract               |
|  Crate: providers (planned), api (current Anthropic)             |
+------------------------------------------------------------------+
|                     Extension Surfaces                            |
|  Skills, plugins, task orchestration, hook extensions              |
|  Crates: skills (planned), plugins (planned)                     |
+------------------------------------------------------------------+
|                        Command Layer                              |
|  Slash command registry, validation, help metadata, handlers      |
|  Crate: commands                                                 |
+------------------------------------------------------------------+
```

### Crate Responsibilities

| Crate | Role | Status |
|-------|------|--------|
| `api` | Anthropic API client, SSE streaming, OAuth, retries | Functional |
| `runtime` | Conversation engine, sessions, permissions, hooks, config, MCP, usage | Functional |
| `tools` | Tool registry and 18 built-in tool implementations | Partial (stubs remain) |
| `commands` | Slash command definitions and dispatch | Functional (15 commands) |
| `rusty-claude-cli` | CLI binary, REPL, rendering, argument parsing | Functional |
| `compat-harness` | Upstream manifest extraction for parity testing | Functional |
| `providers` | Multi-provider abstraction layer | Planned |
| `skills` | Bundled and user skill registry | Planned |
| `plugins` | Plugin loading and lifecycle | Planned |

### Data Flow

```
User Input (REPL or one-shot)
  |
  v
CLI Layer (rusty-claude-cli)
  |-- parse args / slash commands
  |-- dispatch to command handlers OR
  |-- forward prompt to runtime
  |
  v
Engine Runtime (runtime)
  |-- assemble system prompt (project context, tools, permissions)
  |-- send to provider via streaming API
  |-- receive streaming response
  |-- extract tool calls from response
  |-- check permissions for each tool call
  |-- execute tools (possibly concurrently)
  |-- append tool results to conversation
  |-- loop until model stops or budget exceeded
  |
  v
Provider Adapter (api / providers)
  |-- normalize request to provider format
  |-- stream SSE events
  |-- map provider events to ProviderEvent contract
  |
  v
Tool System (tools)
  |-- validate tool input against schema
  |-- check permission level required
  |-- execute tool implementation
  |-- return structured result
  |
  v
Session Store
  |-- persist messages to disk
  |-- support resume, export, compaction
```

## Installation

### From Source (Current)

Requires Rust 1.75+ and Cargo:

```bash
git clone https://github.com/Sam9239/Anvil.git
cd Anvil/rust
cargo build --release
```

The binary will be at `rust/target/release/anvil` (or `anvil.exe` on Windows).

Optionally add to your PATH:

```bash
# Linux/macOS
cp rust/target/release/anvil ~/.local/bin/

# Windows (PowerShell)
Copy-Item rust\target\release\anvil.exe "$env:USERPROFILE\.local\bin\"
```

### Package Manager (Planned)

```bash
# Planned: cargo install
cargo install anvil-agent

# Planned: shell script installer
curl -fsSL https://anvil.dev/install.sh | sh

# Planned: Windows
winget install anvil
```

## Usage

### Interactive REPL

```bash
# Start a new session
anvil

# Start with a specific model
anvil --model anthropic:claude-sonnet-4-20250514

# Resume the most recent session
anvil --resume

# Resume a specific session
anvil --resume abc123
```

Once in the REPL:

```
anvil> explain the authentication flow in this project

[Anvil reads your codebase and responds with analysis]

anvil> refactor the login handler to use async/await

[Anvil edits the file and shows the diff]

anvil> /cost
Session cost: $0.04 (1,247 input + 832 output tokens)

anvil> /compact
Session compacted: 47 messages -> summary + last 5

anvil> /help
Available commands: /help /status /compact /clear /model /permissions
  /cost /resume /config /memory /init /diff /version /export /session
```

### One-Shot Prompt Mode

```bash
# Single prompt, output to stdout
anvil "what does the main function do?"

# Pipe input
cat error.log | anvil "explain this error and suggest a fix"

# Use in scripts
anvil "generate a unit test for src/auth.rs" > tests/auth_test.rs
```

### Configuration

```bash
# Initialize project config
anvil init

# This creates .anvil/ directory with:
#   settings.json - project settings
#   ANVIL.md - project instructions for the agent
```

## Commands

| Command | Description |
|---------|-------------|
| `/help` | Show available commands and usage |
| `/status` | Show session status (model, tokens, tools) |
| `/compact` | Compact session history to save context |
| `/clear` | Clear current session and start fresh |
| `/model` | Change the active model mid-session |
| `/permissions` | View or change permission mode |
| `/cost` | Show token usage and estimated cost |
| `/resume` | Resume a previous session |
| `/config` | View or modify configuration |
| `/memory` | Manage persistent memory files |
| `/init` | Initialize project configuration |
| `/diff` | Show git diff of changes made |
| `/version` | Show Anvil version |
| `/export` | Export session as JSON |
| `/session` | Session management (list, switch, delete) |

### Planned Commands

| Command | Description | Phase |
|---------|-------------|-------|
| `/plan` | Enter planning mode for complex tasks | 4 |
| `/review` | Review and summarize recent changes | 4 |
| `/commit` | Stage and commit with generated message | 4 |
| `/pr` | Create pull request with generated description | 4 |
| `/tasks` | Manage background tasks | 4 |
| `/mcp` | Manage MCP server connections | 4 |
| `/skills` | List and manage skills | 4 |
| `/plugins` | Manage plugins | 4 |
| `/doctor` | Run system diagnostics | 4 |
| `/login` / `/logout` | Authentication management | 4 |

## Tools

Anvil provides built-in tools that the LLM can use during conversations:

### File Operations

| Tool | Description | Status |
|------|-------------|--------|
| `read_file` | Read file contents with line numbers | Complete |
| `write_file` | Create new files | Complete |
| `edit_file` | Replace text in existing files | Complete |
| `glob_search` | Find files by pattern | Complete |
| `grep_search` | Search file contents with regex | Complete |
| `notebook_edit` | Edit Jupyter notebook cells | Partial |

### Execution

| Tool | Description | Status |
|------|-------------|--------|
| `bash` | Execute shell commands | Complete |
| `powershell` | Execute PowerShell commands (Windows) | Partial |
| `repl` | Run code in persistent REPL session | Stub |

### Web

| Tool | Description | Status |
|------|-------------|--------|
| `web_fetch` | Fetch URL content | Partial |
| `web_search` | Search the web | Partial |

### Agent

| Tool | Description | Status |
|------|-------------|--------|
| `agent` | Spawn sub-agent for complex tasks | Stub |
| `send_message` | Send message to another agent | Stub |
| `todo_write` | Manage task lists | Partial |

### Meta

| Tool | Description | Status |
|------|-------------|--------|
| `skill` | Invoke a loaded skill | Partial |
| `tool_search` | Search available tools | Stub |
| `config` | Read/write configuration | Stub |
| `sleep` | Wait for specified duration | Complete |

## Configuration

Anvil uses a layered configuration system:

### Configuration Files

| File | Scope | Purpose |
|------|-------|---------|
| `~/.anvil/settings.json` | Global | User-wide defaults |
| `.anvil/settings.json` | Project | Project-specific settings |
| `.anvil/settings.local.json` | Local | Git-ignored local overrides |
| `ANVIL.md` or `CLAUDE.md` | Project | Project instructions for the agent |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `OPENAI_API_KEY` | OpenAI API key (planned) |
| `ANVIL_MODEL` | Default model to use |
| `ANVIL_CONFIG` | Path to config file |
| `ANVIL_LOG` | Log level (debug, info, warn, error) |

### Example settings.json

```json
{
  "model": "anthropic:claude-sonnet-4-20250514",
  "permissions": {
    "defaultMode": "WorkspaceWrite",
    "allow": ["read_file", "glob_search", "grep_search"],
    "deny": ["bash --rm", "bash --sudo"]
  },
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "bash",
        "command": "echo $TOOL_INPUT | security-check"
      }
    ]
  },
  "mcp": {
    "servers": {
      "my-tools": {
        "command": "npx",
        "args": ["-y", "my-mcp-server"]
      }
    }
  }
}
```

## Permissions

Anvil uses a permission system to control what tools can do:

### Permission Modes

| Mode | Description | Tools Allowed |
|------|-------------|---------------|
| `ReadOnly` | Can only read files and search | read_file, glob, grep, web_fetch |
| `WorkspaceWrite` | Can read and write within project | All read + write_file, edit_file, notebook |
| `DangerFullAccess` | Full access including shell execution | All tools including bash, powershell |

### Permission Flow

1. Tool requests execution
2. Check tool's minimum permission level against current mode
3. Check allow/deny rules from config
4. Run PreToolUse hooks (if configured)
5. If still undecided, prompt user interactively
6. Execute or deny

### Auto Mode (Planned)

In auto mode, Anvil will classify tool calls and auto-approve safe operations:

- **Always safe**: read_file, glob_search, grep_search, web_fetch
- **Classified**: bash commands analyzed for destructive patterns
- **Always ask**: write operations to sensitive files, force pushes, deletions

## Skills

Skills are markdown files that define reusable agent workflows:

### Skill Format

```markdown
---
name: commit
description: Stage and commit changes with a generated message
allowedTools: [Bash, Read, Grep, Glob]
---

Review all staged and unstaged changes, then create a commit with
a descriptive message following conventional commit format.

Steps:
1. Run git status and git diff
2. Analyze changes
3. Generate commit message
4. Create the commit
```

### Skill Locations

| Location | Priority | Description |
|----------|----------|-------------|
| `.anvil/skills/` | 1 (highest) | Project-specific skills |
| `~/.anvil/skills/` | 2 | User-wide skills |
| Bundled | 3 (lowest) | Built-in skills shipped with Anvil |

### Using Skills

```bash
# In REPL
anvil> /commit

# Or reference by name
anvil> use the commit skill to commit my changes
```

## MCP Support

Anvil supports the Model Context Protocol for connecting to external tool servers:

### Configuration

```json
{
  "mcp": {
    "servers": {
      "filesystem": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path"]
      },
      "database": {
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-postgres"],
        "env": {
          "DATABASE_URL": "postgresql://localhost/mydb"
        }
      }
    }
  }
}
```

### Supported Transports

| Transport | Status | Description |
|-----------|--------|-------------|
| stdio | Working | Subprocess communication |
| HTTP/SSE | Partial | Server-Sent Events streaming |
| WebSocket | Planned | Bidirectional WebSocket |

## Sessions

### Session Lifecycle

```
Create -> Active -> Persist -> Resume/Export/Compact
```

### Session Commands

```bash
# List recent sessions
anvil> /session list

# Resume most recent
anvil --resume

# Resume specific session
anvil --resume abc123

# Export current session
anvil> /export

# Compact to save context window
anvil> /compact

# Clear and start fresh
anvil> /clear
```

### Session Storage

Sessions are stored as JSON in `~/.anvil/sessions/` with the structure:

```json
{
  "version": 1,
  "session_id": "abc123",
  "messages": [...],
  "metadata": {
    "model": "anthropic:claude-sonnet-4-20250514",
    "created_at": "2026-04-01T12:00:00Z",
    "total_input_tokens": 1247,
    "total_output_tokens": 832
  }
}
```

## Development

### Prerequisites

- Rust 1.75+ with Cargo
- Git
- (Optional) ripgrep for grep tool testing
- (Optional) Node.js for MCP server testing

### Build

```bash
cd rust
cargo build
```

### Test

```bash
cd rust

# Format check
cargo fmt --check

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Run all tests
cargo test --workspace
```

### Project Structure

```
rust/
  crates/
    api/              # Anthropic API client, SSE, OAuth
    commands/         # Slash command registry and dispatch
    compat-harness/   # Upstream parity testing tools
    runtime/          # Core engine: conversation, sessions, permissions
    rusty-claude-cli/ # CLI binary: REPL, rendering, args
    tools/            # Built-in tool implementations
```

### Adding a New Tool

1. Create a new function in `rust/crates/tools/src/lib.rs` (or a new module)
2. Implement the tool execution logic
3. Define input/output schema
4. Register the tool in the tool registry
5. Set the minimum permission level
6. Add tests
7. Update this README

### Adding a New Command

1. Add the command enum variant in `rust/crates/commands/src/lib.rs`
2. Define the command spec (name, description, args)
3. Implement the handler in `rust/crates/rusty-claude-cli/`
4. Add tests
5. Update this README

## Project Status

### Current State (as of 2026-04-01)

Anvil has a stable Rust foundation with a working local agent core. It is **not yet deployment-ready** but is actively progressing toward its first public release.

**What works:**
- Interactive REPL and one-shot prompt mode
- Conversation with Claude models via Anthropic API
- 18 built-in tools (core file/shell/search tools are production quality)
- Session persistence, resume, export, compaction
- 15 slash commands
- Permission system with 3 modes
- OAuth and API key authentication
- MCP stdio server connections
- Project instruction discovery (CLAUDE.md)
- Hook execution (pre/post tool use)
- Cost tracking and reporting
- Full Rust test suite passing on Windows

**What does not work yet:**
- Multi-provider support (Anthropic only)
- Streaming query engine with auto-compaction
- ~8 tools are still stubs (agent, skill, REPL, notebook, etc.)
- Advanced permission features (auto-mode, classifiers)
- Plugin system
- Skills registry (only local file loading)
- Most planned commands (plan, review, commit, pr, etc.)
- Cross-platform packaging and distribution
- CI/CD pipeline

### Release Gates

| Gate | Status | Description |
|------|--------|-------------|
| A: Foundation | Complete | Rust workspace stable, tests green |
| B: Provider Decoupling | Not Started | Provider abstraction + adapters |
| C: Workflow Coverage | Partial | Commands + tools for daily use |
| D: Extension Readiness | Not Started | Skills registry + plugin system |
| E: Release Readiness | Not Started | Packaging + CI + docs |

## Roadmap

See [docs/roadmap.md](docs/roadmap.md) for the full detailed roadmap.

### Phase Summary

| Phase | Name | Status | Focus |
|-------|------|--------|-------|
| 0 | Product Identity | Complete | Repo, branding, docs |
| 1 | Runtime Hardening | Complete | Tests, Windows stability |
| 2 | Provider + Streaming | Next | Multi-LLM, streaming engine |
| 3 | Tool Completion | Planned | All tools production-grade |
| 4 | Command Expansion | Planned | 30+ slash commands |
| 5 | Extensions | Planned | Skills, plugins, hooks |
| 6 | Release Readiness | Planned | Packaging, CI, docs |
| 7 | Expanded Breadth | Future | Advanced orchestration, IDE |

## Repository Layout

```
.
|-- assets/           Product assets and artwork
|-- docs/             Product docs, roadmap, shipping plan, architecture
|   |-- roadmap.md            Detailed phase-by-phase roadmap
|   |-- shipping-plan.md      Remaining work before ship
|   |-- target-architecture.md Target architecture design
|   `-- claude-priority-map.md Feature priority analysis
|-- rust/             Active Rust workspace (shipping implementation)
|   `-- crates/
|       |-- api/              Anthropic API client
|       |-- commands/         Slash command registry
|       |-- compat-harness/   Upstream parity testing
|       |-- runtime/          Core engine
|       |-- rusty-claude-cli/ CLI binary
|       `-- tools/            Tool implementations
|-- src/              Python compatibility and analysis surfaces
|-- tests/            Python test suite for compatibility verification
|-- CLAUDE.md         Project metadata for AI assistants
|-- PARITY.md         Feature coverage gap analysis
`-- README.md         This file
```

## Key Documents

| Document | Description |
|----------|-------------|
| [docs/roadmap.md](docs/roadmap.md) | Detailed development roadmap with phases and deliverables |
| [docs/shipping-plan.md](docs/shipping-plan.md) | Remaining work checklist before release |
| [docs/target-architecture.md](docs/target-architecture.md) | Target architecture with crate design |
| [docs/claude-priority-map.md](docs/claude-priority-map.md) | Feature priority analysis from reference comparison |
| [PARITY.md](PARITY.md) | Gap analysis between current state and target |

## Contributing

Anvil is in active development. Contributions welcome in these areas:

1. **Tool implementations** - Complete stubbed tools or add new ones
2. **Provider adapters** - Add support for new LLM providers
3. **Skills** - Write reusable skill definitions
4. **Tests** - Expand integration and smoke test coverage
5. **Documentation** - Improve guides and examples

### Development Workflow

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/Anvil.git
cd Anvil/rust

# Create a branch
git checkout -b feature/my-feature

# Make changes, then verify
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Commit and push
git commit -m "feat: add my feature"
git push origin feature/my-feature

# Open a pull request
```

## Recent Milestones

- Bootstrap of standalone Anvil repository
- Product rebrand of Rust CLI to `anvil`
- Windows hardening and full Rust workspace test stabilization
- Comprehensive roadmap with deployment-ready phase plan
- Reference analysis against production coding agent (1,900 files)

## License

See repository for license details.
