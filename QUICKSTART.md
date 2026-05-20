# Quick Start Guide

## 🚀 Get Started in 3 Steps

### 1. Build the Application

```bash
./setup.sh
```

Or manually:
```bash
cargo build --release
```

### 2. Run OpenAgent

```bash
cargo run
```

Or use the release binary:
```bash
./target/release/openagent
```

### 3. Configure

1. Click **Settings** in the sidebar
2. Enter your **OpenRouter API Key**
   - Get one at: https://openrouter.ai/keys
3. (Optional) Set **GitHub Registry URL** for community agents
4. Click **Save Settings**

## 📝 Using Your First Agent

### Option A: Use Example Agents

Example agents are automatically installed in `~/.openagent/agents/` when you run `setup.sh`.

1. Click **Installed Agents** in sidebar
2. Select an agent (e.g., "Text Assistant")
3. Fill in the required fields
4. Click **Run**
5. View the output!

### Option B: Browse Community Agents

1. Set GitHub Registry URL in Settings
2. Click **Community Agents** in sidebar
3. Browse available agents
4. Click **Install** on any agent
5. Find it in **Installed Agents**

## 📦 Example Agents Included

- **Image Generator** - Generate AI images with style options
- **Text Assistant** - General purpose AI assistant
- **Code Generator** - Generate code in multiple languages

## 🔑 Getting an OpenRouter API Key

1. Visit https://openrouter.ai
2. Sign up for an account
3. Go to Keys section
4. Create a new API key
5. Copy and paste into OpenAgent settings

## 📁 File Locations

- **Agents**: `~/.openagent/agents/`
- **Settings**: `~/.openagent/settings.json`
- **Cache**: `~/.openagent/cache/`

## 🛠️ Creating Custom Agents

Create a YAML file in `~/.openagent/agents/` or use this template:

```yaml
id: my-agent
name: My Custom Agent
description: Does something cool
version: 1.0.0
author: Your Name

inputs:
  prompt:
    type: textarea
    required: true

model:
  provider: openrouter
  model: openai/gpt-4

output:
  type: markdown

prompt: |
  {{prompt}}
```

## ❓ Troubleshooting

### App won't start
- Make sure Rust is installed: `rustc --version`
- Try: `cargo clean && cargo build --release`

### No output after clicking Run
- Check your API key is set correctly
- Check internet connection
- Look for error messages in status bar

### Community agents not loading
- Verify GitHub Registry URL is correct
- Check internet connection
- URL should point to raw YAML file

### Can't find installed agents
- Check `~/.openagent/agents/` directory exists
- Verify YAML files are valid
- Try copying example agents again

## 📚 Learn More

- [README.md](README.md) - Project overview
- [BUILD.md](BUILD.md) - Build instructions
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical details
- [examples/](examples/) - Example agent YAML files

## 💡 Tips

- Use Markdown output type for formatted text
- Select type inputs provide dropdown menus
- Template variables use `{{variable_name}}` syntax
- Images are displayed directly in the app
- Settings are auto-saved to `~/.openagent/settings.json`

🤖
