# Local-first frontier (Cue)

## Principles (hard)

1. **Less dependency** — ffmpeg/ffprobe on PATH; no npm ML bundles
2. **Zero config** — the default read returns container metadata, streams, chapters, and embedded subtitles with ffmpeg/ffprobe alone. Scene detection is not the default read
3. **Local first, cloud optional** — no cloud speech recognition or vision model on the default path
4. **Timeline first** — pixel frames only on a follow-up, not in the default read
5. **Rust first** — timeline assembly, and frame follow-ups when requested, via Rust engines when built

## Extraction stack (priority)

| Layer | Default | Optional |
| --- | --- | --- |
| Container/streams | **ffprobe** (+ Rust assemble) | — |
| Chapters | embedded chapter markers | — |
| Subtitles | embedded extract | — |
| Scenes | off | ffmpeg scene filter when `profile` is `quality` or `include_scenes` is set |
| Frames | off | `video_evidence` renders, crops, or OCRs one frame. `include_keyframes` on the shipped server warns and returns no frames |
| Speech recognition | off | `include_transcript` on the shipped server warns and returns an empty transcript |
| OCR | off | `video_evidence` `ocr_frame` |
| Outline | chapters and embedded subtitles | the shipped server does not return a separate `agent_index` |

## Zero-config usage

```bash
npm i -g @sylphx/cue
# system: ffmpeg/ffprobe
read_video { "sources":[{"path":"/abs/a.mp4"}] }
```

That call does not detect scenes, extract frames, or run speech recognition.

Optional scene detection:

```bash
read_video { "sources":[{"path":"/abs/a.mp4"}], "profile": "quality" }
```

`quality` sets `include_scenes` only. It does not extract frames or run speech recognition.

## Non-negotiable

1. Zero API key for default path
2. Prefer Rust native MCP when present
3. Few tools; primary path documented in TOOL_SURFACE.md
4. Cloud / LLM only optional and non-authority
5. Product SSOT is this repository only (no products monorepo)

## Composition: Iris optional semantics on structural keyframes

The default `read_video` does not detect scenes. The shipped server does not extract keyframes. This composition is an explicit path, not the default read.

Cue owns timeline structure (probe, chapters, embedded subtitles, and scene boundaries only when requested).
Open-vocab objects, masks, and captions belong to Iris:

1. `read_video` with `profile` `quality` for scene boundaries, then `video_evidence` for a frame at a timestamp you already have
2. For that frame, the agent calls Iris `read_image` with `include_semantics: true`
3. Merge by timestamp locator — do not run per-frame VLM inside Cue

### Compose: Cue → Iris semantic timeline

`bun run compose:iris -- /abs/clip.mp4` renders structural keyframes and queries
Iris optional semantics (`include_semantics`) per keyframe, emitting timestamped objects.
Scenes and keyframes are not the default read. Structure stays in Cue; semantics stays in Iris. See [compose-iris.md](./compose-iris.md).

## Zero-config CTA

```bash
npx -y @sylphx/cue
```

Bare MCP stdio for agents.
