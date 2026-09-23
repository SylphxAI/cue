# Cue

### Video answers with timestamp-level proof

Cue gives agents a local video timeline they can search and cite. The default
read returns container metadata, streams, chapters, and embedded subtitles.
Scenes and frames are separate.

```bash
npx -y @sylphx/cue
```

For Claude Code:

```bash
claude mcp add cue -- npx -y @sylphx/cue
```

## Read a local video

```json
{
  "sources": [{ "path": "/absolute/path/to/demo.mp4" }]
}
```

That call uses the `fast` profile. It returns container metadata, streams,
chapters, and embedded subtitles. It does not detect scenes, extract frames,
or run speech recognition.

Then ask:

> “Which chapter covers the pricing change?”

Cue returns chapter and subtitle locators with `timestamp_ms` and the source
hash. It does not invent a transcript. A quote search matches embedded
subtitles only. Render or crop a frame afterwards with `video_evidence`, once
you have a timestamp.

## Jobs Cue is built for

| Ask your agent | Cue returns |
| --- | --- |
| “Find this quote.” | a timestamped match in embedded subtitles, when those subtitles exist |
| “Summarize this meeting.” | chapters and embedded subtitles |
| “Where is the code shown?” | one frame from `video_evidence` at a known timestamp |
| “What changed in this demo?” | scene boundaries when `profile` is `quality` or `include_scenes` is set |
| “Give me the useful moments.” | chapters, embedded subtitles, warnings, and gaps |

## Tool surface

| Tool | Purpose |
| --- | --- |
| `read_video` | Default `fast` profile: container metadata, streams, chapters, and embedded subtitles. Scenes are opt-in. |
| `search_video` | Search embedded subtitle cues. It does not run speech recognition. |
| `video_evidence` | Named follow-up: render, crop, or OCR one frame at a timestamp |

## Predictable defaults

- Omitted `profile` is `fast`: container metadata, streams, chapters, and embedded subtitles. No scenes, frames, or speech recognition.
- `profile` `quality` sets scene detection and keyframes only. It does not enable OCR or speech recognition.
- `include_transcript` stays off unless the caller sets it, including on `quality`.
- OCR stays on `video_evidence`.
- No cloud video API or frame-by-frame vision model is required.
- Missing ffprobe or embedded subtitles is reported as a gap, not guessed around.

## Why agents trust it

Every claim can point back to `timestamp_ms`, a stream index, a subtitle range,
or the source hash. A frame index is present only after `video_evidence`. When
keyframes are requested, structural keyframes are preferred over sampling every
frame.

## Companion MCP tools

| Product | Job |
| --- | --- |
| [Citra](https://github.com/SylphxAI/citra) | PDF answers with page-level proof |
| [Iris](https://github.com/SylphxAI/iris) | Image facts and pixel evidence |
| [Spine](https://github.com/SylphxAI/spine) | Repository architecture and impact |
| [Locus](https://github.com/SylphxAI/locus) | Exact code-chunk retrieval |
| [Lookout](https://github.com/SylphxAI/lookout) | Web research with source excerpts |

Each product is independent. Install only the tools your agent needs.

## Development

```bash
bun install
bun run build
bun test
cargo test
bun run benchmark:public-proof
bun run benchmark:release-gate
```

## License

MIT
