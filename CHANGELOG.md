# Changelog

## 0.3.4

### Patch Changes

- 4bfa0f3: The default `read_video` profile is fast: container metadata, streams, chapters, and embedded subtitles. Scenes, frames, OCR, and speech recognition stay off unless a caller asks for them, and the public docs say so.

## 0.3.1

### Patch Changes

- 4001b30: The MCP `initialize` response advertises the canonical product site, and the package gains discovery keywords.

  `serverInfo.websiteUrl` still named the retired `video-reader-mcp` slug, so every connecting agent saw the wrong identity; it now advertises `https://sylphxai.github.io/cue/`. The published `keywords` also grow to the real queries users type — video analysis, video search, transcription, subtitles, chapters, scenes, frames, timestamps, timeline, ffmpeg, ffprobe.

## 0.3.0

- Add `search_video` with timestamped subtitle and transcript matches.
- Extract embedded subtitles and scene boundaries on the Rust route.
- Add explicit `fast` and `quality` profiles and refresh Cue documentation.

## 0.2.0

### Breaking

- Brand-sole `@sylphx/cue` (bin `cue`).
- Public MCP tools only: `read_video`, `video_evidence` (internal helpers removed from tools/list).
- Family envelope v1; Prism retired.

## 0.1.0

### Minor Changes

- 44427c9: Ship v0.1.0 `read_video` MCP tool with ffprobe timeline extraction, embedded subtitle support, optional scene detection, and unit-tested parsers.

All notable changes are documented here. Releases use [Changesets](https://github.com/changesets/changesets).
