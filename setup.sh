#!/bin/bash

echo "🚀 Setting up OpenAgent..."

echo "📦 Installing example agents..."
mkdir -p ~/.openagent/agents
cp examples/*.yaml ~/.openagent/agents/ 2>/dev/null || true

echo "⚙️  Building project..."
cargo build --release

echo "✅ Setup complete!"
echo ""
echo "To run OpenAgent:"
echo "  cargo run"
echo ""
echo "Or use the release binary:"
echo "  ./target/release/openagent"
echo ""
echo "⚠️  Don't forget to:"
echo "  1. Open Settings in the app"
echo "  2. Add your OpenRouter API key"
echo "  3. Save settings"
