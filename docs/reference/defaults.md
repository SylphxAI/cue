# Predictable defaults

| Profile | Behavior |
| --- | --- |
| `fast` (default) | Container metadata, streams, chapters, and embedded subtitles. No scenes, frames, or speech recognition |
| `quality` | Adds ffmpeg scene detection. It does not extract frames or run speech recognition |
| Transcript | Off. `include_transcript` on the shipped server returns a warning and an empty transcript |
| OCR | `video_evidence` only, not part of `quality` |
| Frames | A named `video_evidence` follow-up. `include_keyframes` on the shipped server returns a warning and no frames |

Missing ffprobe or embedded subtitles is reported as a gap. No cloud video API
or frame-by-frame vision model is required.
