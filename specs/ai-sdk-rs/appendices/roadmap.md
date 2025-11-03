# Appendix E — Delivery Roadmap

## Phase 0 — Foundations (Weeks 1-3)
- Finalize crate scaffolding (`ai_core`, `ai_stream`, `ai_tooling`, `ai_transport`).
- Implement `generate_text`/`stream_text` happy paths against mock provider.
- Stand up SSE transport in Axum example service.
- Establish CI with formatting, static analysis, and doc tests.

## Phase 1 — Provider Adapters (Weeks 4-7)
- Ship `Open AI`, `Azure Open AI`, and `Anthropic` adapters with token accounting.
- Add capability registry consumed by transport and tooling layers.
- Provide golden-stream fixtures validating SSE parity against TypeScript reference traces.

## Phase 2 — Structured Outputs & Tools (Weeks 8-10)
- Implement `generate_object`/`stream_object` with schema validation and repair hooks.
- Deliver procedural macro for tool definitions plus dynamic tool fallback.
- Integrate multi-step agent controller (`stop_when`, `prepare_step`).

## Phase 3 — Extended Providers & Local Runtimes (Weeks 11-13)
- Add adapters for Amazon Bedrock, `Groq`, Google Generative AI.
- Provide local runner integrations (`Ollama`, `Mistral`) with streaming compatibility tests.
- Harden error taxonomy and telemetry exporters.

## Phase 4 — GA Hardening (Weeks 14-16)
- Security review (auth, secret handling, sandboxed tool execution).
- Documentation pass (cookbook, migration guide, API docs).
- Performance benchmarks and load testing across transports.
- Cut release candidates, finalize versioning commitments.
