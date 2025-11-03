# Acceptance Criteria — AI SDK for Rust

**Version**: 1.0  
**Last Updated**: 2025-11-03  
**Status**: Active

This document defines the acceptance criteria that MUST be met before the AI SDK for Rust can be considered feature-complete and ready for initial release (v0.1.0).

---

## 1. Build & Compilation

### 1.1 Workspace Build
- [ ] `cargo build --workspace` completes successfully with no errors
- [ ] `cargo build --workspace --all-features` includes all feature flags
- [ ] `cargo build --workspace --release` produces optimized builds
- [ ] Build time < 2 minutes for clean build on standard hardware

### 1.2 Feature Combinations
- [ ] Default features build successfully
- [ ] Each provider feature builds independently
- [ ] `--no-default-features` builds core functionality
- [ ] `telemetry` feature compiles and links
- [ ] `wasm` feature compiles (if implemented)

### 1.3 Platform Support
- [ ] Builds on macOS (Apple Silicon and Intel)
- [ ] Builds on Linux (x86_64)
- [ ] Builds on Windows (if targeted)
- [ ] WASM target compiles (if `wasm` feature enabled)

---

## 2. Code Quality

### 2.1 Formatting
- [ ] `cargo fmt --all --check` passes with no changes needed
- [ ] All source files use consistent formatting
- [ ] No mixed tabs/spaces

### 2.2 Linting
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes with 0 warnings
- [ ] No `#[allow(clippy::...)]` without justification comments
- [ ] All `unsafe` code has safety documentation (or none exists)

### 2.3 Compiler Warnings
- [ ] 0 compiler warnings across all crates
- [ ] 0 unused imports or dead code
- [ ] All deprecated API usage addressed

### 2.4 Safety
- [ ] No `unsafe` code in default builds (verified by `#![forbid(unsafe_code)]`)
- [ ] Any `unsafe` in optional features is well-documented
- [ ] All public APIs are safe to call from any thread (where applicable)

---

## 3. Testing

### 3.1 Test Execution
- [ ] `cargo test --workspace` passes with 0 failures
- [ ] `cargo test --workspace --all-features` passes
- [ ] All tests complete in < 30 seconds (excluding integration tests)
- [ ] Integration tests complete in < 2 minutes

### 3.2 Test Coverage
- [ ] Line coverage ≥ 80% across all crates
- [ ] Branch coverage ≥ 70% for critical paths
- [ ] All public APIs have at least one test
- [ ] Error paths are tested

### 3.3 Test Categories

#### Unit Tests
- [ ] `ai_error`: Error type conversions and display
- [ ] `ai_core`: Builder patterns and type conversions
- [ ] `ai_ui_protocol`: SSE encoding/decoding
- [ ] `ai_stream`: Stream transformations
- [ ] `ai_tools`: Tool registration and execution
- [ ] `ai_agents`: Loop control logic
- [ ] `ai_rag`: Chunking and similarity search

#### Integration Tests
- [ ] OpenAI provider end-to-end (with mocked API)
- [ ] Streaming text generation
- [ ] Tool calling roundtrip
- [ ] Agent multi-step execution
- [ ] RAG retrieval pipeline

#### Conformance Tests
- [ ] SSE wire format matches golden files (100% match)
- [ ] UI message payloads validate against JSON schema
- [ ] Error codes match specification
- [ ] Keep-alive timing correct (15s ± 1s)

#### Property Tests
- [ ] Partial object merging is associative
- [ ] Chunk strategies preserve all input text
- [ ] Tool schema generation is deterministic

---

## 4. API Compliance

### 4.1 Core API Parity
- [ ] `generate_text()` matches TypeScript API semantics
- [ ] `stream_text()` matches TypeScript API semantics
- [ ] `generate_object()` validates schemas correctly
- [ ] `stream_object()` streams partial objects
- [ ] Error types map to TypeScript equivalents

### 4.2 UI Protocol Compliance
- [ ] SSE events use correct event names: `message`, `data`, `metadata`, `error`, `close`
- [ ] JSON payloads match `UIMessage` structure exactly
- [ ] Message parts include all required fields
- [ ] Transient data parts flagged correctly
- [ ] Source parts include citation metadata
- [ ] Error frames include `code` field

### 4.3 Wire Format Validation
- [ ] Every SSE frame ends with `\n\n`
- [ ] Multi-line JSON splits into multiple `data:` fields
- [ ] Event IDs are monotonic when present
- [ ] Keep-alive comments sent every 15 seconds
- [ ] No trailing whitespace in events

---

## 5. Provider Implementation

### 5.1 OpenAI Provider (Required)
- [ ] Can authenticate with API key
- [ ] Supports environment variable `OPENAI_API_KEY`
- [ ] `generate_text()` calls Chat Completions API
- [ ] `stream_text()` processes SSE stream from OpenAI
- [ ] Tool calls convert to/from OpenAI format
- [ ] Error responses map to `AiError` correctly
- [ ] Reasoning fields extracted (when present)
- [ ] Usage metadata captured (prompt/completion tokens)
- [ ] Supports temperature, max_tokens, stop_sequences
- [ ] Handles rate limiting with retry-after

### 5.2 Provider Stubs
- [ ] `anthropic` crate compiles with capability map
- [ ] `google` crate compiles with capability map
- [ ] `ollama` crate compiles with capability map
- [ ] All stubs return `unimplemented!()` with helpful messages

---

## 6. Examples

### 6.1 Chat Example (`examples/chats/axum_sse`)
- [ ] Compiles successfully
- [ ] Server starts on specified port
- [ ] `/api/chat/stream` endpoint accepts POST requests
- [ ] Request payload validates (messages, system, tools)
- [ ] Streams SSE responses
- [ ] SSE format matches wire specification
- [ ] CORS headers configured
- [ ] Graceful shutdown on Ctrl+C
- [ ] Environment variables for API keys
- [ ] README with usage instructions

### 6.2 Agent Example (`examples/agents/tool_loop`)
- [ ] Compiles successfully
- [ ] Executes multi-step tool loop
- [ ] `stop_when(step_count_is(N))` terminates correctly
- [ ] Tool results fed back into next step
- [ ] Final transcript includes all messages
- [ ] Handles tool errors gracefully
- [ ] README with expected output

### 6.3 RAG Example (`examples/rag/axum_retriever`)
- [ ] Compiles successfully
- [ ] Document ingestion works
- [ ] Chunking produces reasonable chunks
- [ ] Embeddings generated (mocked or real)
- [ ] Similarity search returns top-k results
- [ ] Citations included in response
- [ ] Source parts streamed correctly
- [ ] README with setup instructions

---

## 7. Features

### 7.1 Text Generation
- [ ] Simple prompt-to-text works
- [ ] Multi-message conversations supported
- [ ] System prompts respected
- [ ] Temperature affects output variety
- [ ] max_tokens limits response length
- [ ] Stop sequences halt generation

### 7.2 Streaming
- [ ] Text streams incrementally (deltas)
- [ ] Full stream includes metadata events
- [ ] Can convert to UI message stream
- [ ] Keep-alive prevents timeout
- [ ] Abort/cancellation works
- [ ] Final event indicates completion

### 7.3 Tool Calling
- [ ] Tools register with registry
- [ ] Tool schemas generate correctly
- [ ] Tool calls parsed from provider
- [ ] Tools execute asynchronously
- [ ] Results serialize back to provider
- [ ] Multi-tool calls in single turn
- [ ] Tool errors don't crash loop

### 7.4 Agent Loops
- [ ] Loop executes until stop condition
- [ ] `step_count_is(N)` works
- [ ] Custom stop predicates work
- [ ] `prepare_step` hooks called
- [ ] Transcript captures all turns
- [ ] Usage aggregates across steps
- [ ] Reasoning preserved (if available)

### 7.5 Structured Outputs
- [ ] JSON Schema validation works
- [ ] Type-safe deserialization
- [ ] Partial objects stream incrementally
- [ ] Invalid outputs rejected
- [ ] Schema errors descriptive

### 7.6 RAG
- [ ] Documents chunk correctly
- [ ] Chunk strategies configurable
- [ ] Overlap preserved
- [ ] Embeddings integrate
- [ ] Similarity search accurate
- [ ] Top-k retrieval works
- [ ] Min score filtering works
- [ ] Citations assemble correctly

---

## 8. Documentation

### 8.1 API Documentation
- [ ] All public modules have doc comments
- [ ] All public types have doc comments
- [ ] All public functions have doc comments
- [ ] Examples in doc comments compile
- [ ] `cargo doc --no-deps --open` generates clean docs
- [ ] No broken intra-doc links

### 8.2 User Documentation
- [ ] `README.md` in root with quick start
- [ ] `docs/` directory with comprehensive guide
- [ ] Getting Started tutorial
- [ ] Provider setup guides (OpenAI at minimum)
- [ ] Tool development guide
- [ ] Agent patterns guide
- [ ] RAG integration guide
- [ ] Wire format specification
- [ ] Migration from TypeScript guide
- [ ] Security best practices
- [ ] Troubleshooting guide

### 8.3 Example Documentation
- [ ] Each example has README
- [ ] Usage instructions clear
- [ ] Environment variables documented
- [ ] Expected output shown
- [ ] Error scenarios covered

### 8.4 Internal Documentation
- [ ] Architecture diagrams in `docs/`
- [ ] Design decisions recorded
- [ ] PLANNING/STATUS.md current
- [ ] CHANGELOG.md up to date

---

## 9. Security

### 9.1 Secrets Management
- [ ] No API keys in source code
- [ ] No API keys in tests (use mocks)
- [ ] Environment variable loading secure
- [ ] Secrets redacted in logs
- [ ] Secrets redacted in telemetry

### 9.2 Input Validation
- [ ] All user inputs validated
- [ ] Tool inputs sanitized
- [ ] JSON payloads size-limited
- [ ] Path traversal prevented (if file ops)

### 9.3 Network Security
- [ ] TLS enforced for provider APIs
- [ ] Certificate validation enabled
- [ ] Timeouts configured on all requests
- [ ] Request size limits enforced

### 9.4 Tool Execution
- [ ] Tool execution sandboxed (or documented risks)
- [ ] Resource limits on tool execution
- [ ] Tool execution timeouts
- [ ] Tool errors isolated

---

## 10. Performance

### 10.1 Latency
- [ ] Overhead vs direct API calls < 100ms (p50)
- [ ] Overhead vs direct API calls < 200ms (p99)
- [ ] Streaming starts within 500ms
- [ ] First token latency < provider latency + 50ms

### 10.2 Throughput
- [ ] Can handle 100 concurrent requests (example server)
- [ ] Memory usage stable under load
- [ ] No memory leaks in long-running streams

### 10.3 Resource Usage
- [ ] Memory usage < 50MB baseline
- [ ] CPU usage minimal when idle
- [ ] File descriptor cleanup
- [ ] Connection pooling working

---

## 11. CI/CD

### 11.1 GitHub Actions
- [ ] CI runs on every push
- [ ] CI runs on every PR
- [ ] Format check job
- [ ] Clippy check job
- [ ] Test job (all features)
- [ ] Doc build job
- [ ] Example compilation job

### 11.2 CI Results
- [ ] All jobs passing
- [ ] Build time < 10 minutes
- [ ] Flaky test rate < 1%
- [ ] No ignored/skipped tests without reason

---

## 12. Release Readiness

### 12.1 Versioning
- [ ] Version set to 0.1.0
- [ ] Semantic versioning followed
- [ ] CHANGELOG.md complete
- [ ] Git tags for releases

### 12.2 Publishing Prep
- [ ] Crate metadata complete (Cargo.toml)
- [ ] License files present (MIT + Apache-2.0)
- [ ] README in each crate
- [ ] Repository URL set
- [ ] Keywords/categories set
- [ ] No `path` dependencies (or published)

### 12.3 Community
- [ ] CONTRIBUTING.md present
- [ ] CODE_OF_CONDUCT.md present
- [ ] Issue templates created
- [ ] PR template created
- [ ] Security policy (SECURITY.md)

---

## 13. Final Acceptance Checklist

Before declaring **DONE**, all of the following MUST be true:

- [ ] All sections above completed
- [ ] No blocking issues in issue tracker
- [ ] All quality gates passed
- [ ] Manual testing completed
- [ ] Peer review completed (if applicable)
- [ ] PLANNING/STATUS.md marked COMPLETE
- [ ] README.md finalized
- [ ] Release notes drafted

---

## 14. Sign-off

**Acceptance Test Date**: _____________  
**Tested By**: _____________  
**Result**: [ ] PASS / [ ] FAIL  
**Notes**:

---

**Document Owner**: QA Lead / Build System Lead  
**Review Frequency**: End of each phase  
**Status**: ACTIVE
