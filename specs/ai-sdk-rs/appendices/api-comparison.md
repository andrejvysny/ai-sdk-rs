# Appendix C — TypeScript ↔ Rust API Comparison

| TypeScript API | Rust Equivalent | Notes |
| --- | --- | --- |
| `generateText({ model, prompt })` | `ai_core::generate_text(GenerateTextRequest)` | Returns `GenerateTextResponse` with async getters for reasoning, tool calls, sources. |
| `streamText({ model, prompt, onFinish })` | `ai_core::stream_text(StreamTextConfig)` | Produces `StreamTextHandle` exposing async streams plus `on_finish` callback. |
| `generateObject({ schema })` | `ai_core::generate_object(GenerateObjectRequest)` | Accepts `serde`/`schemars` schema; validates via `Result`. |
| `streamObject({ schema })` | `ai_core::stream_object(StreamObjectConfig)` | Streams partial objects and elements using `futures::Stream`. |
| `tool({ description, inputSchema, execute })` | `ai_tooling::tool! { ... }` procedural macro | Macro emits strongly typed tool struct with schema metadata and async executor. |
| `dynamicTool(...)` | `ai_tooling::DynamicTool::new(...)` | Input/output typed as `serde_json::Value`; manual validation required. |
| `stopWhen(stepCountIs(n))` | `ai_agents::StopWhen::step_count(n)` | Used with `AgentLoop::builder()` for multi-step orchestration. |
| `prepareStep(...)` | `AgentLoop::prepare_step(handler)` | Handler receives mutable step context and message list. |
| `createUIMessageStream()` | `ai_stream::UiMessageStream::new()` | Provides writer/reader pair for SSE bridging. |
| `createUIMessageStreamResponse({ stream })` | `ai_transport::sse::response_from_stream(stream)` | Converts stream into Axum/`Actix`/Hyper SSE response. |
| `DefaultChatTransport` | `ai_transport::HttpChatTransport` | Configurable headers, body transforms, and retry policy. |
| Provider factory (e.g., `openai('gpt-4.1')`) | `providers::open_ai::model("gpt-4-1")` | Returns provider-specific model implementing `LanguageModel` trait. |

## Migration Notes
- Rust APIs favor builders or structs over large option bags; default traits supply parity settings.
- Async streams use `futures` combinators; integrate via `tokio` or `async-std` runtime adaptors.
- Provider options map to strongly typed config structs rather than loose dictionaries.
