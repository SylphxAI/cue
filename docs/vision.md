# Vision — Cue

Cue is the local-first video timeline evidence tool for agents.

- **Identity:** package `@sylphx/cue`, bin `cue`, MCP `io.github.SylphxAI/cue`, site <https://sylphxai.github.io/cue/>.
- **User:** an agent that must find a quote, a chapter, a code demo or a visual moment in a local video.
- **Job:** build a searchable timeline — streams, chapters, subtitles, scenes, structural keyframes — and return timestamped evidence for a follow-up frame, crop or OCR.
- **Promise:** every claim points at a timestamp, stream index, subtitle range, frame index or source hash; missing subtitles or ASR are reported as gaps.
- **Defaults:** `fast` probes streams and reads embedded subtitles; `quality` explicitly enables scene detection, keyframes, OCR or local ASR.
- **Boundaries:** Cue owns the video timeline. It does not own cloud video APIs, frame-by-frame VLM summaries, or generic media-library management.
