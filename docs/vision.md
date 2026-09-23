# Vision — Cue

Cue is the local-first video timeline evidence tool for agents.

- **Identity:** package `@sylphx/cue`, bin `cue`, MCP `io.github.SylphxAI/cue`, site <https://sylphxai.github.io/cue/>.
- **User:** an agent that must find a quote, a chapter, a code demo, or a visual moment in a local video.
- **Job:** the default read is a searchable timeline of streams, chapters, and embedded subtitles. Scene boundaries are a separate request. A frame, crop, or OCR is a `video_evidence` follow-up.
- **Promise:** every claim points at a timestamp, stream index, subtitle range, or source hash. A frame index appears only after a frame follow-up. Missing subtitles are reported as gaps. The shipped server does not run speech recognition.
- **Defaults:** omitted `profile` is `fast` (container metadata, streams, chapters, embedded subtitles). `quality` adds ffmpeg scene detection. It does not extract frames or run speech recognition.
- **Boundaries:** Cue owns the video timeline. It does not own cloud video APIs, frame-by-frame VLM summaries, or generic media-library management.
