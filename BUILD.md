# Build Instructions

## Prerequisites

- Rust (latest stable)
- Cargo

**macOS Note**: Uses eframe 0.29 which resolves previous compatibility issues. See [MACOS_FIX.md](MACOS_FIX.md) for details.

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

## Development

```bash
cargo check  # Check for errors
cargo clippy # Run linter
cargo fmt    # Format code
```

## Features

- ✅ Native Rust GUI with egui/eframe (v0.29)
- ✅ Dynamic agent loading from YAML
- ✅ OpenRouter API integration
- ✅ Text and Image rendering
- ✅ Community agent installation
- ✅ Local agent management
- ✅ Cross-platform support
- ⚠️ Markdown rendering (simplified - full rendering coming soon)

## Setup

1. Run the application
2. Go to Settings
3. Enter your OpenRouter API key
4. Optionally set GitHub registry URL
5. Save settings

## Using Agents

1. Install example agents from the `examples/` directory:
   - Copy YAML files to `~/.openagent/agents/`
2. Or browse Community Agents (if registry URL is configured)
3. Select an agent from Installed Agents
4. Fill in the required inputs
5. Click Run
6. View the output

## Example Agents

The `examples/` directory contains:
- `image-generator.yaml` - Generate AI images
- `text-assistant.yaml` - AI text assistant
- `code-generator.yaml` - Code generation
- `registry.yaml` - Example registry file

## Project Structure

```
src/
├── main.rs              # Entry point
├── app.rs               # Main application logic
├── agents/              # Agent system
│   ├── loader.rs        # Load/save agents
│   ├── models.rs        # Agent data structures
│   └── registry.rs      # Remote registry
├── openrouter/          # API client
│   └── client.rs        # OpenRouter integration
├── render/              # Output rendering
│   └── output.rs        # Text/Markdown/Image
├── storage/             # File system
│   └── mod.rs           # Storage management
├── settings/            # Configuration
│   └── mod.rs           # Settings management
└── ui/                  # User interface
    ├── input.rs         # Dynamic input forms
    └── view.rs          # View states
```
