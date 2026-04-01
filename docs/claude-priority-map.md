# Claude Priority Map For Anvil

This document converts the local `Claude/` analysis into practical Anvil priorities.

## Source Basis

Derived from:

- `Claude/claude_code_src_report.md`
- `Claude/claude_code_file_inventory.md`
- `Claude/claude_code_tree_depth3.txt`
- `Claude/analysis.json`

## What The Local Analysis Says Matters Most

The TypeScript snapshot is large, but its complexity is concentrated in a few areas.

Top subsystem weights from the local analysis:

- `utils`: 564 files, 180,487 LOC
- `components`: 389 files, 81,892 LOC
- `services`: 130 files, 53,683 LOC
- `tools`: 184 files, 50,863 LOC
- `commands`: 207 files, 26,528 LOC
- `hooks`: 104 files, 19,232 LOC
- `ink`: 96 files, 19,859 LOC
- `bridge`: 31 files, 12,613 LOC
- `cli`: 19 files, 12,355 LOC

## What To Port First

Not all of that should be copied into Anvil with equal urgency.

### Tier 1: Must-Have Product Core

- query and conversation runtime
- tool contracts and tool execution loop
- command registry and slash command surface
- permissions and safety checks
- session persistence and compaction
- CLI and structured output transports

Representative high-signal files from the local analysis:

- `main.tsx`
- `query.ts`
- `QueryEngine.ts`
- `Tool.ts`
- `commands.ts`
- `tools.ts`
- `cli/print.ts`
- `cli/structuredIO.ts`
- `bootstrap/state.ts`

### Tier 2: High-Value Extension Surfaces

- plugin loader and marketplace model
- skill loader and bundled skill registry
- MCP lifecycle and resources
- agent/sub-agent orchestration
- task runtime

Representative high-signal files:

- `utils/plugins/pluginLoader.ts`
- `utils/plugins/marketplaceManager.ts`
- `services/plugins/pluginOperations.ts`
- `skills/loadSkillsDir.ts`
- `tools/SkillTool/SkillTool.ts`
- `tools/shared/spawnMultiAgent.ts`
- `tools/AgentTool/AgentTool.tsx`

### Tier 3: Later or Optional

- rich terminal UI parity
- buddy/companion UX
- voice mode
- bridge-heavy remote product features
- analytics and experimentation layers

These matter for polish and product breadth, but they are not required for the first strong Anvil release.

## High-Confidence Product Thesis

The most Claude-like result will not come from recreating all UI components.

It will come from recreating these behaviors well:

- reliable query loop
- capable tool orchestration
- strong permissions model
- command-driven workflow
- extensibility through skills and plugins
- solid session and memory handling
- provider flexibility without runtime fragmentation

## Immediate Backlog Targets For Anvil

1. Clean structured output in Rust CLI mode.
2. Expand command parity around review, planning, tasks, MCP, skills, and plugins.
3. Add real hook execution, not just hook config parsing.
4. Build a first-class provider adapter layer.
5. Turn local skill loading into a full registry.
6. Add plugin loading and command/tool injection.

## Why This Matters

The local Claude analysis gives us a map of where the original product invested engineering effort.

Anvil should use that map for prioritization, not imitation.
