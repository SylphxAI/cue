# Tool surface — Cue

## Public MCP tools only

| Tool | Role |
| --- | --- |
| `read_video` | Timeline document: ffprobe, subtitles, structural scenes/keyframes, agent_index |
| `search_video` | Timestamped subtitle and transcript matches |
| `video_evidence` | Follow-up ops via `op`: `render_frame` \| `crop_frame` \| `ocr_frame` |

## Not in tools/list

`hash_source`, `build_cache_key`, `assemble_probe_timeline`, bare `render_frame`/`crop_frame` — internal core only.

## Rules

1. Always `read_video` first.
2. Structural keyframes only — not N-second grid spam.
3. No cloud vision/ASR required for success.
4. Prism is not part of the public product surface — host composes Cue with Iris when needed.
