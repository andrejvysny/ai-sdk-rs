# Appendix A — UI Message SSE Wire Format

This appendix specifies the Server-Sent Events protocol that the Rust SDK MUST produce to remain compatible with existing AI SDK UI clients that consume `UIMessageStream` payloads.[^ui-stream]

## A.1 Event Structure
Each SSE frame is emitted using UTF-8 text with standard `event:` and `data:` fields. Frames are terminated by a double newline (`\n\n`).

| Field | Required | Description |
| --- | --- | --- |
| `event` | yes | Logical event name. Common values: `message`, `data`, `metadata`, `error`, `close`. |
| `id` | optional | Monotonic identifier per stream, used for resume semantics. |
| `retry` | optional | Suggested reconnection delay in milliseconds. |
| `data` | yes | JSON payload encoded on a single line. Multi-line JSON MUST be split into multiple `data:` fields. |

Example:
```
event: message
data: {"type":"assistant","id":"msg_01","parts":[{"type":"text","text":"Hello"}]}

```

## A.2 Payload Shapes
The `data` payload reflects strongly typed Rust structs that mirror the TypeScript `UIMessage` definitions.

### Message Envelope
```json
{
  "type": "assistant",            // 'assistant' | 'user' | 'system' | 'tool'
  "id": "msg_01",                 // unique string
  "createdAt": "2025-11-03T10:15:00Z",
  "parts": [ ... ],                // array of message parts
  "metadata": {                    // optional per-message metadata
    "model": "gpt-4.1-mini",
    "totalTokens": 1234
  }
}
```
- Rust SDK MUST preserve ordering of message parts.
- Metadata is emitted separately when using `messageMetadata` callbacks; clients merge on arrival.[^ui-stream]

### Data Parts
Persistent data parts augment the message history.
```json
{ "type": "data-weather", "id": "weather-1", "data": { "city": "SF", "status": "loading" } }
```
- Reconciliation occurs whenever a subsequent part reuses the same `id`.
- SDK MUST include the `id` field whenever updates are expected.

### Transient Parts
Transient parts are delivered to clients but excluded from stored history.
```json
{ "type": "data-notification", "transient": true, "data": { "message": "Processing...", "level": "info" } }
```
- Clients access transient parts exclusively through `onData` listeners; Rust SSE encoder MUST still emit them to maintain parity.

### Sources
Sources reference external documents, typically for RAG use cases.
```json
{
  "type": "source",
  "value": {
    "type": "source",
    "sourceType": "url",
    "id": "source-1",
    "url": "https://example.com",
    "title": "Example Source"
  }
}
```

## A.3 Error Frames
Errors encountered mid-stream are surfaced without terminating the SSE connection unless classified as fatal.
```json
{ "type": "error", "message": "Tool execution failed", "code": "TOOL_ERROR" }
```
- SDK SHOULD map structured errors into `code` enums (`TOOL_ERROR`, `PROVIDER_ERROR`, `VALIDATION_ERROR`).
- Fatal errors MUST be followed by a terminal `close` event.

## A.4 Termination Semantics
- Normal completion: emit `{ "type": "finish", ... }` followed by `event: close`.
- Aborted streams: include `{ "type": "finish", "finishReason": "cancelled" }`.
- Keep-alive: send comment frames (`:") at least every 15 seconds when idle to prevent proxies from timing out the connection.

## A.5 Resume Support (Future)
To align with AI SDK UI resume streams, the Rust transport SHOULD expose checkpoint IDs. Future iterations will define replay mechanics once parity with JavaScript `resumeStream` APIs stabilizes. Track progress in `appendices/roadmap.md`.

---

[^ui-stream]: AI SDK UI — Streaming Custom Data. https://ai-sdk.dev/docs/ai-sdk-ui/streaming-data
