# OpenAgent

Native desktop AI agent runner powered by OpenRouter APIs.

## Features

- Native Rust GUI with egui/eframe
- Dynamic agent installation from GitHub
- Custom YAML-based agent definitions
- OpenRouter API integration
- Text and Image output rendering
- Cross-platform (Windows, macOS, Linux)

## Quick Start

```bash
# Build and run
cargo run

# Or use the setup script
./setup.sh
```

## Installation

```bash
cargo build --release
```

**Note for macOS users**: This project uses eframe 0.29 which resolves previous macOS compatibility issues. See [MACOS_FIX.md](MACOS_FIX.md) for details.

## Usage

```bash
cargo run
```

## Configuration

Settings are stored in `~/.openagent/settings.json`

Required:
- OpenRouter API key
- GitHub registry URL (optional)

## Agent Format

Example agent YAML:

```yaml
id: image-generator
name: Image Generator
description: Generate AI images
version: 1.0.0
author: Ash

inputs:
  prompt:
    type: textarea
    required: true

  style:
    type: select
    options:
      - anime
      - realistic

model:
  provider: openrouter
  model: openai/gpt-4

output:
  type: image

prompt: |
  Generate a {{style}} image of:
  {{prompt}}
```

**Note**: Markdown output type currently displays as plain text. Full markdown rendering coming soon.

## Directory Structure

```
~/.openagent/
├── agents/          # Installed agents
├── cache/           # Cache directory
└── settings.json    # User settings
```

## Building

### Development

```bash
cargo run
```

### Release

```bash
cargo build --release
```

The binary will be in `target/release/openagent`

## License

MIT
