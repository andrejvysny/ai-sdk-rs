# AI SDK for Rust

[![CI](https://github.com/ai-sdk/ai-sdk-rs/workflows/CI/badge.svg)](https://github.com/ai-sdk/ai-sdk-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/ai_core.svg)](https://crates.io/crates/ai_core)
[![Documentation](https://docs.rs/ai_core/badge.svg)](https://docs.rs/ai_core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

> **Status**: Early Development (v0.1.0-alpha)

A production-ready Rust SDK for building AI applications with feature parity to the [Vercel AI SDK](https://ai-sdk.dev). Build chat applications, agents, and RAG systems with type-safe, async-first Rust APIs.

## Features

- 🚀 **Type-safe AI APIs** - Generate text, structured outputs, and embeddings
- 🔄 **Streaming Support** - SSE-compatible streaming for real-time chat UIs
- 🛠️ **Tool Calling** - Define and execute tools with automatic schema generation
- 🤖 **Agent Loops** - Multi-step agent orchestration with stop conditions
- 📚 **RAG Primitives** - Chunking, embeddings, and vector search
- 🔌 **Provider Agnostic** - OpenAI, Anthropic, Google, Ollama support
- 🎯 **UI Compatible** - Wire-format compatible with AI SDK UI (`useChat`, `useCompletion`)
- 📊 **Observability** - Built-in tracing and OpenTelemetry support

## Quick Start

```rust
use ai_core::{generate_text, GenerateTextRequest};
use ai_providers_openai::OpenAiProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider
    let provider = OpenAiProvider::from_env()?;
    let model = provider.model("gpt-4");
    
    // Generate text
    let response = generate_text(GenerateTextRequest {
        model: Box::new(model),
        prompt: "Explain Rust's ownership system".into(),
        temperature: Some(0.7),
        ..Default::default()
    }).await?;
    
    println!("{}", response.text);
    Ok(())
}
```

## Chat Example with Streaming

```rust
use ai_core::{stream_text, StreamTextRequest};
use ai_ui_protocol::to_sse_response;
use axum::{routing::post, Router};

async fn chat_handler(
    Json(req): Json<ChatRequest>,
) -> Result<Sse<impl Stream>, ChatError> {
    let provider = OpenAiProvider::from_env()?;
    let model = provider.model("gpt-4");
    
    let stream = stream_text(StreamTextRequest {
        model: Box::new(model),
        messages: req.messages,
        temperature: Some(0.7),
        ..Default::default()
    }).await?;
    
    Ok(to_sse_response(stream.to_ui_message_stream()))
}
```

## Project Structure

- `ai_core` - Core traits and types
- `ai_error` - Error handling
- `ai_ui_protocol` - SSE wire format for UI clients
- `ai_tools` - Tool calling framework
- `ai_agents` - Agent loop orchestration
- `ai_rag` - Retrieval-augmented generation primitives
- `ai_providers/*` - Provider implementations

## Documentation

- [Getting Started](docs/getting-started.md)
- [API Reference](https://docs.rs/ai_core)
- [Examples](examples/)
- [Migration from TypeScript](docs/migration.md)

## Examples

- [Chat with Axum](examples/chats/axum_sse) - SSE streaming chat endpoint
- [Agent Tool Loop](examples/agents/tool_loop) - Multi-step agent with tools
- [RAG System](examples/rag/axum_retriever) - Document retrieval with citations

## Requirements

- Rust 1.75 or later
- Tokio async runtime

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ai_core = "0.1"
ai_providers_openai = "0.1"
tokio = { version = "1.35", features = ["full"] }
```

## Development

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run examples
cargo run -p axum_sse --example chat

# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Inspired by and compatible with the [Vercel AI SDK](https://ai-sdk.dev).
