# Tool surface — Cue

## Public MCP tools only

| Tool | Role |
| --- | --- |
| `read_video` | Default `fast` profile: container metadata, streams, chapters, and embedded subtitles. `quality` or `include_scenes` adds ffmpeg scene boundaries. `include_keyframes` and `include_transcript` warn and stay empty on the shipped server |
| `search_video` | Timestamped matches in embedded subtitles. Transcript matches only when a transcript is already present. Does not run speech recognition |
| `video_evidence` | Named follow-up, not part of the default read. `op`: `render_frame` \| `crop_frame` \| `ocr_frame` |

## Not in tools/list

`hash_source`, `build_cache_key`, `assemble_probe_timeline`, bare `render_frame`/`crop_frame` — internal core only.

## Rules

1. Always `read_video` first.
2. The default read does not detect scenes, extract frames, or run speech recognition.
3. The shipped server does not extract keyframes. `video_evidence` renders, crops, or OCRs one frame.
4. No cloud vision or speech recognition is required for the default read to succeed.
5. Composition with companion tools is through public MCP and SDK contracts only; the host composes Cue with Iris when needed.
