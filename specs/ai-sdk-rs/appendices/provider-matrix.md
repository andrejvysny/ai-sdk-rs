# Appendix B — Provider Capability Matrix

Legend: `Yes` = supported at launch, `Partial` = limited feature set available, `Planned` = roadmap item (see Appendix E).

| Provider | Text Generation | Streaming APIs | Structured Outputs | Tool Calling | Reasoning / Advanced Modes | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `Open AI` | Yes | Yes | Yes | Yes | Yes (responses + reasoning summaries) | Requires per-model options for reasoning, tools, and image generation.
| Azure Open AI | Yes | Yes (chunky) | Yes | Yes | Partial (depends on deployment) | Deployment-based configuration; mirrors Open AI options with Azure headers.
| Anthropic | Yes | Yes | Partial (beta header) | Yes | Yes (`thinking` budgets, code/computer tools) | Tool/structured streaming gated behind beta header `fine-grained-tool-streaming-2025-05-14`.
| Amazon Bedrock | Yes | Yes | Partial | Partial | Partial | Prioritize Titan and third-party model mappings; confirm guardrail integration.
| `Groq` | Yes | Yes | Planned | Partial | Planned | Focus on low-latency text models; align with `Groq` SDK settings.
| Google Generative AI | Yes | Yes | Yes | Partial | Partial (Gemini reasoning) | Validate source streaming and safety settings.
| `Mistral` | Yes | Yes | Partial | Partial | Planned | Provide pay-as-you-go and on-premise connectivity support.
| `Ollama` | Yes | Partial | Planned | Planned | N/A | Local runtime; ensure streaming delta compatibility.
| OpenRouter | Yes | Yes | Planned | Planned | Planned | Proxy aggregator; support per-route capability introspection.

## Follow-up Tasks
- Confirm parity details for providers marked `Partial`/`Planned` before GA; update matrix accordingly.
- Extend matrix once additional community providers are onboarded.

---

**Reference Links**
```
https://ai-sdk.dev/providers/ai-sdk-providers/openai
https://ai-sdk.dev/providers/ai-sdk-providers/azure
https://ai-sdk.dev/providers/ai-sdk-providers/anthropic
```
