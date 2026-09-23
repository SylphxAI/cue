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
| Timeline document | `read_video` | default: streams, chapters, embedded subtitles. `quality` or `include_scenes` adds ffmpeg scene boundaries. The shipped server does not extract keyframes |
| Timestamped search | `search_video` | embedded subtitle matches with timestamps. No speech recognition |
| Frame evidence | `video_evidence` (`render_frame`, `crop_frame`, `ocr_frame`) | a separate follow-up: frame, crop, and OCR at a known timestamp |

## Evidence contract

Every result carries `timestamp_ms`, stream or subtitle locators, source hash, route, warnings and gaps. A frame locator is present only after a frame follow-up. See [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md).

## Not owned

Cloud video or speech-recognition APIs as a requirement, per-frame vision summaries, media-library management, and still-image semantics.
