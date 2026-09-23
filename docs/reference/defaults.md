# Predictable defaults

| Profile | Behavior |
| --- | --- |
| `fast` (default) | Container metadata, streams, chapters, and embedded subtitles. No scenes, frames, or speech recognition |
| `quality` | Sets scene detection and keyframes only. Does not enable OCR or speech recognition |
| Transcript | Off unless `include_transcript` is set, including on `quality` |
| OCR | `video_evidence` only, not part of `quality` |
| Frames | A named `video_evidence` follow-up, or keyframes when `quality` or `include_keyframes` is set. Structural keyframes are preferred to sampling every frame |

Missing ffprobe or embedded subtitles is reported as a gap. No cloud video API
or frame-by-frame vision model is required.
