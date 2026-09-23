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

Choose `quality`, or set `include_scenes`, only when scene boundaries are worth
the extra ffmpeg pass. That profile does not extract frames or run speech
recognition. A frame, crop, or OCR is `video_evidence`. Setting
`include_keyframes` or `include_transcript` on the shipped server returns a
warning and empty arrays.
