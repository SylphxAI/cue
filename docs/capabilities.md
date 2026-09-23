# Capabilities — Cue

## Surfaces

| Surface | Identity |
| --- | --- |
| MCP | `io.github.SylphxAI/cue` over stdio, `npx -y @sylphx/cue` |
| CLI | `cue` |
| SDK | `@sylphx/cue/sdk` |

## Owned capabilities

| Capability | Tool | Evidence |
| --- | --- | --- |
| Timeline document | `read_video` | streams, chapters, subtitles, structural scenes/keyframes, agent index |
| Timestamped search | `search_video` | subtitle/transcript matches with timestamps |
| Frame evidence | `video_evidence` (`render_frame`, `crop_frame`, `ocr_frame`) | frame, crop and OCR evidence at a known timestamp |

## Evidence contract

Every result carries `timestamp_ms`, stream/subtitle/frame locators, source hash, route, warnings and gaps. See [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md).

## Not owned

Cloud video or ASR APIs as a requirement, per-frame vision summaries, media-library management, and still-image semantics.
