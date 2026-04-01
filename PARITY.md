# PARITY GAP ANALYSIS

Scope: repository-level comparison between the broad compatibility and source-analysis surfaces under `src/` and the active Rust implementation under `rust/crates/`.

Method: compare feature surfaces, registries, entrypoints, runtime plumbing, and current behavior claims. No external source code is copied into Anvil through this process.

## Executive Summary

Anvil now has a serious Rust foundation for:

- API and OAuth basics
- conversation and session state
- a working local tool loop
- MCP stdio bootstrap and routing
- project guidance discovery
- hooks runtime behavior
- a useful built-in tool set
- Windows-verified Rust test coverage

Anvil is still not at the final target breadth.

Largest remaining gaps:

- provider abstraction is not finished
- plugins are not implemented
- skills are local-file only rather than a full registry
- command breadth is still much narrower than the target workflow set
- structured and remote transport breadth is still limited
- broader workflow and service layers are still missing

## Surface Review

### tools/

Current Rust coverage:

- tool registry exists in `rust/crates/tools/src/lib.rs`
- built-ins include shell, PowerShell, file, search, web, todo, notebook, skill, agent, config, tool search, and REPL primitives
- runtime execution is wired through the tools crate and runtime conversation loop

Remaining gaps:

- no full set of workflow, resource, or system tools yet
- MCP-facing breadth is still limited
- LSP-style and richer task or team workflows are still absent
- the registry is still closer to an MVP surface than the final intended surface

Status: partial core only

### hooks/

Current Rust coverage:

- hook config is parsed and merged in `rust/crates/runtime/src/config.rs`
- runtime hook behavior exists, including pre-tool denial and post-tool feedback paths
- hook behavior is covered by runtime tests

Remaining gaps:

- no dedicated hook-management command surface yet
- no broader hook editing or lifecycle UX yet
- hook system still needs productization beyond the core runtime path

Status: partial, with real runtime behavior present

### plugins/

Current Rust coverage:

- no dedicated plugin subsystem under `rust/crates/`

Remaining gaps:

- plugin loader
- plugin lifecycle model
- plugin command surface
- plugin-provided tool and hook extension path

Status: missing

### skills/

Current Rust coverage:

- `Skill` tool resolves and reads local `SKILL.md` files
- project guidance discovery exists in runtime prompt assembly

Remaining gaps:

- bundled skills registry
- skill-management commands
- discovery and ranking flows
- MCP-derived skill generation

Status: basic local skill loading only

### cli/

Current Rust coverage:

- shared slash-command registry exists in `rust/crates/commands/src/lib.rs`
- the current CLI supports a functional local command core
- REPL, prompt mode, rendering, and session-facing flows exist in `rust/crates/rusty-claude-cli/src/main.rs`

Remaining gaps:

- planning, review, tasks, MCP management, plugin, and skills command families are still missing
- structured automation transport still needs further cleanup and breadth
- remote transport breadth is still limited

Status: functional local CLI core, but still narrower than the target product surface

### assistant runtime and orchestration

Current Rust coverage:

- core loop exists in `rust/crates/runtime/src/conversation.rs`
- session persistence exists in `rust/crates/runtime/src/session.rs`
- hook-aware runtime behavior exists
- sub-agent primitive exists in the tools crate

Remaining gaps:

- provider abstraction is not done
- richer plan, execute, verify product workflows are not done
- broader background and remote orchestration paths are not done

Status: strong local core, incomplete orchestration breadth

### services/

Current Rust coverage:

- core API client in `rust/crates/api`
- OAuth support in `rust/crates/runtime/src/oauth.rs`
- MCP config and stdio support in runtime
- usage accounting and remote support primitives in runtime

Remaining gaps:

- broader service ecosystem is still missing
- plugin, analytics, sync, team-memory, and related service layers are not present as first-class Rust subsystems
- provider ergonomics remain too narrow for the final product target

Status: core foundation exists, broader service ecosystem missing

## Current Assessment

What can already be called strong:

- Rust foundation and test stability
- local runtime core
- session handling
- permission handling
- hooks runtime behavior
- useful built-in tool coverage

What is still blocking the main goal:

- provider abstraction and multi-provider support
- plugin system
- skills registry
- missing command families
- broader tool and transport breadth
- packaging and release readiness outside of local development

## Working Rule

Update this document whenever Anvil materially closes or opens a feature-coverage gap.
