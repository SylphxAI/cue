# Predictable defaults

| Profile | Behavior |
| --- | --- |
| `fast` | Probe streams and read embedded metadata and subtitles |
| `quality` | Explicitly enables scene detection, structural keyframes, OCR, or local ASR |
| Evidence | Structural keyframes are preferred to sampling every frame |

Missing ffprobe, subtitles, or ASR is reported as a gap. No cloud video API or
frame-by-frame vision model is required.
