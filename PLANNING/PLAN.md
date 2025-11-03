# AI SDK for Rust — Implementation Plan v1.0

**Document Status**: Active Implementation Guide  
**Target**: Feature parity with Vercel AI SDK v6 (TypeScript)  
**Spec Version**: AI-SDK-for-Rust-Spec-v1.1  
**Last Updated**: 2025-11-03

---

## 1. Executive Summary

### 1.1 Objective
Build a production-ready Rust SDK for AI applications that:
- Mirrors Vercel AI SDK semantics and ergonomics
- Maintains wire-format compatibility with `useChat` and other AI SDK UI clients
- Provides extensible architecture for providers, tools, agents, and RAG
- Delivers type-safe, async-first Rust APIs with comprehensive error handling

### 1.2 Success Criteria
- ✅ Workspace builds cleanly with `cargo build --workspace`
- ✅ All tests pass with `cargo test --workspace`
- ✅ Clippy clean with `cargo clippy -D warnings`
- ✅ SSE wire format matches appendix specification
- ✅ Chat example works with `useChat` client without modifications
- ✅ OpenAI provider fully functional for text generation and streaming
- ✅ Agent demo executes multi-step tool loops with `stop_when`
- ✅ RAG demo returns grounded answers with citations
- ✅ Documentation complete in `docs/` directory

### 1.3 Technical Constraints
- **Rust**: Stable ≥ 1.75
- **Async Runtime**: `tokio` (1.35+)
- **HTTP Client**: `reqwest` with TLS (rustls)
- **Serialization**: `serde` + `schemars` for JSON Schema
- **Error Handling**: `thiserror` with single `AiError` taxonomy
- **Tracing**: `tracing` with optional `opentelemetry` feature
- **Safety**: `#![forbid(unsafe_code)]` by default
- **WASM**: Feature-gated support via adapters

---

## 2. Architecture Overview

### 2.1 Crate Structure
```
ai-sdk-rs/
├── ai_error/              # Error taxonomy (thiserror-based)
├── ai_core/               # Core traits, types, request builders
├── ai_stream/             # Stream primitives and transforms
├── ai_ui_protocol/        # SSE encoding, UI message types
├── ai_middleware/         # Logging, retry, rate-limiting
├── ai_structured/         # Structured outputs, JSON Schema
├── ai_tools/              # Tool trait, registry, execution
├── ai_tools_derive/       # Proc macro for #[derive(Tool)]
├── ai_agents/             # Agent loop controller
├── ai_rag/                # RAG primitives (chunking, embeddings, retrieval)
├── ai_providers/          # Provider implementations
│   ├── openai/           # OpenAI (priority 1)
│   ├── anthropic/        # Anthropic (stub)
│   ├── google/           # Google Generative AI (stub)
│   └── ollama/           # Ollama (stub)
├── examples/
│   ├── chats/axum_sse/   # Chat streaming endpoint
│   ├── agents/tool_loop/ # Multi-step agent demo
│   └── rag/axum_retriever/ # RAG with citations
├── tests/conformance/     # SSE golden tests, tool loop tests
└── docs/                  # mdBook documentation
```

### 2.2 Dependency Flow
```
ai_core ← ai_providers/openai
   ↑
ai_error
   ↑
ai_ui_protocol ← examples/chats/axum_sse
   ↑
ai_stream

ai_tools ← ai_agents ← examples/agents/tool_loop
   ↑
ai_tools_derive

ai_rag ← examples/rag/axum_retriever
```

### 2.3 Feature Flags
Root-level features:
- `provider-openai` (default)
- `provider-anthropic`
- `provider-google`
- `provider-ollama`
- `image` - Image generation support
- `speech` - Speech/TTS support
- `telemetry` - OpenTelemetry integration
- `wasm` - WASM-compatible HTTP client

---

## 3. Implementation Phases

### Phase 0: Planning & Setup ✅
**Status**: Complete  
**Duration**: 1 hour

**Deliverables**:
- [x] Read spec + appendices
- [x] This document (PLAN.md)
- [x] ACCEPTANCE.md with test criteria
- [x] Risk assessment

### Phase 1: Workspace Bootstrap
**Status**: In Progress  
**Duration**: 1-2 hours  
**Owner**: Build System Lead

**Tasks**:
1. Create workspace `Cargo.toml` with all crates
2. Add workspace dependencies (centralized versions)
3. Create `rust-toolchain.toml` (stable 1.75+)
4. Setup CI pipeline (`.github/workflows/ci.yml`)
   - Format check (`cargo fmt --check`)
   - Clippy (`cargo clippy -D warnings`)
   - Test (`cargo test --workspace`)
   - Doc build (`cargo doc --no-deps`)
5. Add repository files:
   - `.gitignore`
   - `.editorconfig`
   - `CONTRIBUTING.md`
   - `CODE_OF_CONDUCT.md`
   - `LICENSE-MIT` + `LICENSE-APACHE`
   - `README.md`
6. Initialize all crate directories with `cargo new --lib`

**Acceptance**:
- `cargo build --workspace` succeeds
- CI runs successfully
- All crates visible in workspace

### Phase 2: Core API & Types
**Status**: Not Started  
**Duration**: 2-3 hours  
**Dependencies**: Phase 1

**Tasks**:
1. **ai_error**: Complete error taxonomy
   - `AiError` enum with all variants
   - Error code mapping for wire format
   - `Result<T>` type alias
   - Tests for error display and conversion

2. **ai_core**: Core abstractions
   - `LanguageModel` trait
   - `EmbeddingsModel` trait
   - `Message`, `MessagePart`, `MessageRole` types
   - `GenerateTextRequest/Response` builders
   - `StreamTextRequest/Response` types
   - `ModelCapabilities` struct
   - Retry/backoff utilities
   - Tests for builders and type conversions

**Acceptance**:
- All core types compile
- Builder patterns work ergonomically
- Unit tests pass
- Doc comments complete

### Phase 3: UI Protocol & SSE
**Status**: Not Started  
**Duration**: 1-2 hours  
**Dependencies**: Phase 2

**Tasks**:
1. **ai_ui_protocol**: SSE wire format
   - `UiEvent` enum (message, data, metadata, error, close)
   - `UiMessage` struct matching spec
   - `encode_sse_event()` function
   - Axum SSE response helpers
   - Keep-alive implementation (15s interval)

2. **ai_stream**: Stream primitives
   - `TextDelta` event type
   - Stream transformation traits
   - Smoothing transforms
   - Aggregation utilities

3. **Tests**: Golden file tests
   - SSE event ordering
   - JSON payload validation
   - Keep-alive timing
   - Error frame format

**Acceptance**:
- SSE encoding matches appendix exactly
- Golden tests pass
- Axum integration compiles

### Phase 4: OpenAI Provider
**Status**: Not Started  
**Duration**: 3-4 hours  
**Dependencies**: Phase 3

**Tasks**:
1. **ai_providers/openai**: Full implementation
   - `OpenAiProvider` struct with auth
   - `OpenAiModel` implementing `LanguageModel`
   - Chat Completions API integration
   - Message format conversion
   - Streaming support (SSE from OpenAI)
   - Tool calling support
   - Error mapping to `AiError`
   - Reasoning field extraction
   - Usage metadata collection

2. **Tests**:
   - Mock server tests
   - Message conversion tests
   - Streaming delta aggregation
   - Tool call roundtrip
   - Error handling

**Acceptance**:
- Can generate text with OpenAI
- Streaming works end-to-end
- Tool calls execute correctly
- Errors map properly
- Integration tests pass

### Phase 5: Chat Example
**Status**: Not Started  
**Duration**: 1-2 hours  
**Dependencies**: Phase 4

**Tasks**:
1. **examples/chats/axum_sse**: Web server
   - Axum server setup
   - `/api/chat/stream` endpoint
   - Request payload parsing
   - SSE response streaming
   - CORS configuration
   - Environment variable config

2. **Testing**:
   - Manual test with curl
   - Simulated `useChat` client test
   - SSE compliance verification

**Acceptance**:
- Server starts successfully
- Endpoint accepts messages
- SSE stream matches wire format
- Works with `useChat` (if available for testing)

### Phase 6: Tools & Agents
**Status**: Not Started  
**Duration**: 2-3 hours  
**Dependencies**: Phase 4

**Tasks**:
1. **ai_tools**: Tool system
   - `Tool` trait
   - `ToolRegistry` for discovery
   - `ToolExecutionContext`
   - Tool error handling
   - Basic built-in tools (weather, calculator)

2. **ai_tools_derive**: Proc macro
   - `#[derive(Tool)]` implementation
   - Schema generation via `schemars`
   - Async executor glue

3. **ai_agents**: Agent loop
   - `AgentLoop` orchestrator
   - `StopWhen` trait and implementations
   - `PrepareStep` hooks
   - Tool execution with retries
   - Transcript/history preservation
   - Usage tracking

4. **examples/agents/tool_loop**: Demo
   - Multi-step weather agent
   - Tool calling with retries
   - `stop_when(step_count_is(6))`

**Acceptance**:
- Tools register and execute
- Agent loop runs to completion
- `stop_when` works correctly
- Transcript captures all steps
- Example runs successfully

### Phase 7: RAG Implementation
**Status**: Not Started  
**Duration**: 2-3 hours  
**Dependencies**: Phase 4

**Tasks**:
1. **ai_rag**: RAG primitives
   - `ChunkStrategy` trait + recursive chunker
   - `VectorStore` trait
   - In-memory vector store implementation
   - `SimilarityQuery` types
   - `RagPipeline` orchestrator
   - Citation assembly

2. **examples/rag/axum_retriever**: Demo
   - Document ingestion endpoint
   - Query endpoint with retrieval
   - Citation formatting
   - Source streaming

**Acceptance**:
- Documents chunk correctly
- Embeddings integrate
- Similarity search works
- Citations appear in response
- Example runs end-to-end

### Phase 8: Structured Outputs
**Status**: Not Started  
**Duration**: 1-2 hours  
**Dependencies**: Phase 4

**Tasks**:
1. **ai_structured**: Structured generation
   - JSON Schema validation
   - Partial object streaming
   - Merge strategies
   - `generate_object<T>()` implementation
   - `stream_object<T>()` implementation

**Acceptance**:
- Schema validation works
- Partial objects merge correctly
- Type safety preserved
- Property tests pass

### Phase 9: Middleware & Observability
**Status**: Not Started  
**Duration**: 1-2 hours  
**Dependencies**: Phase 2

**Tasks**:
1. **ai_middleware**: Interceptors
   - Logging middleware
   - Retry middleware
   - Rate-limiting middleware
   - Redaction/PII filtering

2. **Telemetry**: Optional feature
   - OpenTelemetry spans
   - Structured logging
   - Usage accounting

**Acceptance**:
- Middleware composes correctly
- Tracing works when enabled
- Logs are structured
- No overhead when disabled

### Phase 10: Additional Providers (Stubs)
**Status**: Not Started  
**Duration**: 1 hour  
**Dependencies**: Phase 4

**Tasks**:
1. Create stub implementations:
   - `ai_providers/anthropic`
   - `ai_providers/google`
   - `ai_providers/ollama`
2. Capability matrices
3. Documentation placeholders

**Acceptance**:
- Stubs compile
- Capability maps documented
- `unimplemented!()` with helpful messages

### Phase 11: Testing & Conformance
**Status**: Not Started  
**Duration**: 2 hours  
**Dependencies**: Phases 3-7

**Tasks**:
1. **tests/conformance**: Comprehensive tests
   - SSE golden file tests
   - Partial object merge property tests
   - Tool loop transcript tests
   - Provider error mapping tests
   - End-to-end integration tests

2. **CI Enhancements**:
   - Code coverage tracking
   - Example compilation tests
   - Benchmark suite (optional)

**Acceptance**:
- All conformance tests pass
- Code coverage >80%
- Examples compile and run
- CI is green

### Phase 12: Documentation
**Status**: Not Started  
**Duration**: 2-3 hours  
**Dependencies**: All previous phases

**Tasks**:
1. **docs/**: Complete documentation
   - Getting Started guide
   - API reference (from rustdoc)
   - Architecture diagrams
   - Provider setup guides
   - Tool development guide
   - Agent patterns
   - RAG integration
   - Wire format specification
   - Migration from TypeScript guide
   - Security best practices
   - Testing strategies

2. **README.md**: Comprehensive overview
3. **CHANGELOG.md**: Version history
4. **Examples documentation**: Usage guides

**Acceptance**:
- All public APIs documented
- Examples have READMEs
- Migration guide complete
- Docs build successfully (`mdbook build`)

---

## 4. Risk Assessment & Mitigation

### 4.1 Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| SSE wire format drift from spec | High | Medium | Golden tests, continuous verification against AI SDK UI |
| OpenAI API changes | High | Low | Version pinning, integration tests, monitoring |
| Async runtime compatibility issues | Medium | Low | Stick to tokio, comprehensive testing |
| Tool execution security vulnerabilities | High | Medium | Sandboxing, resource limits, input validation |
| Streaming performance bottlenecks | Medium | Medium | Benchmarking, profiling, buffering strategies |
| WASM compatibility issues | Medium | High | Feature-gated, separate test suite |
| Provider-specific quirks | Medium | High | Isolated provider modules, extensive testing |

### 4.2 Schedule Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Spec ambiguity requiring clarification | Medium | Medium | Document assumptions, create test cases early |
| Dependency conflicts | Low | Low | Workspace-level version management |
| Underestimated complexity | Medium | Medium | Iterative development, regular checkpoints |
| CI/CD setup delays | Low | Low | Use standard GitHub Actions templates |

### 4.3 Quality Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Insufficient test coverage | High | Medium | Mandatory coverage checks, property testing |
| Documentation gaps | Medium | High | Docs-first approach, review checklist |
| API ergonomics issues | Medium | Medium | Early user feedback, examples-driven design |
| Memory leaks in streaming | High | Low | Careful resource management, leak testing |

---

## 5. Quality Gates

Each phase must pass the following gates before proceeding:

### 5.1 Code Quality
- ✅ `cargo fmt --check` passes
- ✅ `cargo clippy --all-targets --all-features -- -D warnings` passes
- ✅ `cargo test --workspace` passes
- ✅ No `TODO` or `FIXME` comments without associated issues
- ✅ All public APIs have doc comments
- ✅ No compiler warnings

### 5.2 Functional Quality
- ✅ Phase-specific acceptance criteria met
- ✅ Examples compile and run
- ✅ Integration tests pass
- ✅ Manual testing completed

### 5.3 Documentation Quality
- ✅ README updated
- ✅ CHANGELOG updated
- ✅ API docs complete
- ✅ Examples documented
- ✅ PLANNING/STATUS.md updated

---

## 6. Development Workflow

### 6.1 Iterative Protocol
1. **Plan**: Review phase tasks, update STATUS.md
2. **Implement**: Write code following Rust best practices
3. **Test**: Write tests first when possible (TDD)
4. **Verify**: Run quality gates
5. **Document**: Update docs and examples
6. **Review**: Self-review checklist
7. **Commit**: Atomic commits with clear messages
8. **Status**: Update STATUS.md with progress

### 6.2 Git Strategy
- **Main branch**: Stable, always buildable
- **Feature branches**: `phase-N/feature-name`
- **Commit messages**: Conventional Commits format
  - `feat:` - New features
  - `fix:` - Bug fixes
  - `docs:` - Documentation
  - `test:` - Tests
  - `refactor:` - Code refactoring
  - `chore:` - Build/tooling

### 6.3 Testing Strategy
- **Unit tests**: Per-module in `mod tests`
- **Integration tests**: In `tests/` directory
- **Golden tests**: SSE format, API responses
- **Property tests**: Using `proptest` or `quickcheck`
- **Example tests**: Verify examples compile and run
- **Conformance tests**: Spec compliance verification

---

## 7. Milestones & Timeline

### Milestone 1: Foundation (Phases 0-3)
**Target**: Day 1  
**Deliverables**: Workspace setup, core types, UI protocol

### Milestone 2: Provider & Chat (Phases 4-5)
**Target**: Day 2  
**Deliverables**: Working OpenAI integration, chat example

### Milestone 3: Advanced Features (Phases 6-7)
**Target**: Day 3  
**Deliverables**: Tools, agents, RAG

### Milestone 4: Completeness (Phases 8-10)
**Target**: Day 4  
**Deliverables**: Structured outputs, middleware, provider stubs

### Milestone 5: Production Ready (Phases 11-12)
**Target**: Day 5  
**Deliverables**: Full test suite, complete documentation

**Total Estimated Duration**: 15-20 focused hours (1 week part-time)

---

## 8. Next Actions

### Immediate (Phase 1)
- [ ] Create workspace `Cargo.toml`
- [ ] Initialize all crate directories
- [ ] Setup `rust-toolchain.toml`
- [ ] Create CI pipeline
- [ ] Add repository files
- [ ] Initial git commit

### Short-term (Phase 2)
- [ ] Implement `ai_error` crate
- [ ] Implement `ai_core` traits
- [ ] Define message types
- [ ] Create request builders
- [ ] Write unit tests

### Medium-term (Phases 3-5)
- [ ] Implement UI protocol
- [ ] Build OpenAI provider
- [ ] Create chat example
- [ ] Verify SSE compliance

---

## 9. Open Questions

1. **WASM Support**: What level of WASM compatibility is required initially?
   - **Decision**: Feature-flagged, not critical for MVP
   
2. **Vector Store**: Should we include a default production-ready vector store?
   - **Decision**: In-memory for MVP, trait allows pluggable backends
   
3. **Telemetry**: OpenTelemetry required or optional?
   - **Decision**: Optional feature flag, no runtime overhead when disabled
   
4. **Testing with Real APIs**: Should CI hit real provider APIs?
   - **Decision**: No, use mocks. Manual integration tests for real APIs.
   
5. **Versioning**: Start at 0.1.0 or 0.0.1?
   - **Decision**: 0.1.0, following semver pre-release conventions

---

## 10. Success Metrics

### Technical Metrics
- **Build Time**: < 2 minutes for clean build
- **Test Coverage**: > 80% line coverage
- **Clippy Warnings**: 0
- **Documentation**: 100% public API coverage
- **Examples**: All compile and run successfully

### Functional Metrics
- **SSE Compliance**: 100% wire format match
- **Provider Coverage**: OpenAI complete, 3+ stubs
- **Feature Coverage**: Text, tools, agents, RAG all working
- **Performance**: < 100ms overhead vs direct API calls

### Quality Metrics
- **CI Success Rate**: > 99%
- **Issue-Free Compilation**: All feature combinations
- **Example Success**: All examples run without errors

---

## 11. References

- **Spec**: `specs/ai-sdk-rs/AI-SDK-for-Rust-Spec-v1.md`
- **Wire Format**: `specs/ai-sdk-rs/appendices/wire-format.md`
- **Provider Matrix**: `specs/ai-sdk-rs/appendices/provider-matrix.md`
- **API Comparison**: `specs/ai-sdk-rs/appendices/api-comparison.md`
- **Vercel AI SDK Docs**: https://ai-sdk.dev/docs

---

**Document Owner**: Build System Lead  
**Review Frequency**: After each phase completion  
**Status**: ACTIVE
