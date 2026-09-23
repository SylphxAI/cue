# Quickstart

## Install

```bash
npx -y @sylphx/cue
```

For Claude Code:

```bash
claude mcp add cue -- npx -y @sylphx/cue
```

Then ask one concrete question and inspect the returned locators, route, warnings,
and gaps before relying on the answer.

## Predictable defaults

Omitted `profile` is `fast`: container metadata, streams, chapters, and embedded
subtitles. That read does not detect scenes, extract frames, or run speech
recognition.

Choose `quality` only when scene detection and keyframes are worth the extra
work. `quality` does not enable OCR or speech recognition. OCR stays on
`video_evidence`. Speech recognition stays off unless `include_transcript` is
set.
