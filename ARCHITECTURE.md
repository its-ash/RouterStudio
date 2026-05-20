# OpenAgent - Architecture & Implementation

## Overview

OpenAgent is a native desktop application built with Rust and egui/eframe that provides a GUI for running AI agents powered by OpenRouter APIs.

## Core Components

### 1. Application State (`src/app.rs`)

The main application struct `OpenAgentApp` manages:
- Current view state (Installed Agents, Community Agents, Create Agent, Settings)
- Agent lists (installed and community)
- Selected agent and input values
- Output rendering
- Async promises for API calls
- Error handling

### 2. Agent System (`src/agents/`)

#### Models (`models.rs`)
Defines the data structures:
- `Agent`: Complete agent definition
- `AgentInput`: Input field specification
- `AgentInputType`: Enum for input types (Text, Textarea, Select, Number, Boolean)
- `AgentModel`: Provider and model configuration
- `AgentOutput`: Output type specification

#### Loader (`loader.rs`)
Manages agent persistence:
- Load installed agents from `~/.openagent/agents/`
- Install new agents (save YAML files)
- Uninstall agents (delete YAML files)

#### Registry (`registry.rs`)
Handles remote agent discovery:
- Fetch `registry.yaml` from GitHub
- Parse agent metadata
- Download agent YAML files
- Return list of available agents

### 3. OpenRouter Client (`src/openrouter/`)

#### Client (`client.rs`)
API integration:
- Execute agents with input values
- Render prompt templates (replace `{{variable}}` placeholders)
- Generate text using chat completions
- Generate images using Pollinations API fallback
- Return typed outputs (Text, Markdown, Image)

### 4. Rendering System (`src/render/`)

#### Output Renderer (`output.rs`)
Displays different output types:
- `Text`: Simple label
- `Markdown`: Rendered with egui_commonmark
- `Image`: Converted to egui texture and displayed

### 5. Storage (`src/storage/`)

File system management:
- Base directory: `~/.openagent/`
- Subdirectories: `agents/`, `cache/`
- Settings file: `settings.json`
- Directory initialization
- Cache clearing

### 6. Settings (`src/settings/`)

Configuration management:
- OpenRouter API key
- GitHub registry URL
- Load/save from JSON
- Default values

### 7. UI Components (`src/ui/`)

#### Input Renderer (`input.rs`)
Dynamic form generation:
- Reads agent input schema
- Renders appropriate widgets:
  - Text: Single-line text edit
  - Textarea: Multi-line text edit
  - Select: Combo box with options
  - Number: Text edit (could be enhanced)
  - Boolean: Checkbox
- Maintains input state in HashMap

#### View (`view.rs`)
Enum defining application views:
- InstalledAgents
- CommunityAgents
- CreateAgent
- Settings

## Data Flow

### Agent Execution Flow

1. User selects an agent
2. UI dynamically generates input form based on agent schema
3. User fills inputs and clicks Run
4. Input values are collected into HashMap
5. Agent prompt template is rendered with input values
6. OpenRouter API is called (async in separate thread)
7. Promise is created and tracked
8. UI polls promise on each frame
9. When complete, output is typed and rendered
10. Result displayed in appropriate renderer

### Community Agent Installation

1. User navigates to Community Agents view
2. App fetches registry.yaml from GitHub (if not cached)
3. Registry contains metadata for each agent
4. App downloads individual agent YAML files
5. Displays list with Install buttons
6. User clicks Install
7. Agent YAML is saved to `~/.openagent/agents/`
8. Installed agents list is refreshed

## Async Architecture

### Challenge
egui is immediate mode and runs on main thread. OpenRouter API calls are async.

### Solution
- Use `poll-promise` crate with `spawn_thread`
- Create tokio runtime in thread
- Block on async operations
- Return result via promise
- Poll promises in main update loop
- Separate promises for different operation types:
  - `agent_promise`: Agent execution
  - `registry_promise`: Registry fetch

## YAML Agent Format

```yaml
id: unique-id
name: Display Name
description: Agent description
version: 1.0.0
author: Author Name

inputs:
  input_name:
    type: text|textarea|select|number|boolean
    required: true|false
    options: [...]  # for select type
    default: "..."  # optional

model:
  provider: openrouter
  model: model-id

output:
  type: text|markdown|image

prompt: |
  Template with {{input_name}} placeholders
```

## Extension Points

### Adding New Input Types
1. Add variant to `AgentInputType` enum
2. Add rendering logic in `InputRenderer::render`
3. Update documentation

### Adding New Output Types
1. Add variant to `OutputType` enum
2. Implement rendering in `OutputRenderer::render`
3. Update OpenRouter client if needed

### Adding New Providers
1. Extend `OpenRouterClient` or create new client
2. Add provider field to agent model
3. Route execution based on provider

## Security Considerations

- API key stored in plaintext JSON (could be improved with system keychain)
- No authentication for GitHub registry (public repos only)
- YAML parsing uses serde_yaml (safe, no code execution)
- Image loading uses image crate (safe, well-tested)

## Performance

- Native code (Rust compiled)
- Immediate mode GUI (minimal state, fast rendering)
- Async operations off main thread
- Minimal dependencies
- No JavaScript runtime overhead

## Cross-Platform Notes

- egui works on Windows, macOS, Linux
- File paths use Rust std (cross-platform)
- Home directory via `dirs` crate
- No OS-specific code needed

## Future Enhancements

- [ ] Agent versioning and updates
- [ ] Local agent creation UI
- [ ] Streaming text output
- [ ] Multiple output formats simultaneously
- [ ] Agent marketplace/rating system
- [ ] Export conversation history
- [ ] Theming support
- [ ] Keyboard shortcuts
- [ ] Search/filter agents
- [ ] Agent categories/tags
