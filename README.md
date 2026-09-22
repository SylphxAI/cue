# Cue

### Video answers with timestamp-level proof

Cue gives agents a local video timeline they can search and cite: streams,
chapters, subtitles, scenes, frames, and exact timestamps.

```bash
npx -y @sylphx/cue
```

For Claude Code:

```bash
claude mcp add cue -- npx -y @sylphx/cue
```

## The fastest useful workflow

```json
{
  "sources": [{ "path": "/absolute/path/to/demo.mp4" }],
  "include_subtitles": true
}
```

Then ask:

> “Find the moment where the presenter explains the pricing change.”

Cue returns the matching timestamp, surrounding transcript, source hash, and
a route to render or crop the exact frame.

## Jobs Cue is built for

| Ask your agent | Cue returns |
| --- | --- |
| “Find this quote.” | timestamped transcript evidence |
| “Summarize this meeting.” | chapters and subtitle-backed notes |
| “Where is the code shown?” | timestamped frame and crop evidence |
| “What changed in this demo?” | timeline and structural scene boundaries |
| “Give me the useful moments.” | compact timeline with warnings and gaps |

## Tool surface

| Tool | Purpose |
| --- | --- |
| `read_video` | Build a local timeline from streams, subtitles, scenes, and keyframes |
| `search_video` | Search subtitles/transcripts and return timestamped matches |
| `video_evidence` | Render, crop, or OCR a frame at a known timestamp |

## Predictable defaults

- `fast` probes streams and reads embedded metadata/subtitles.
- `quality` explicitly requests scene detection, keyframes, OCR, or local ASR.
- No cloud video API or frame-by-frame vision model is required.
- Missing ffprobe, subtitles, or ASR is reported as a gap, not guessed around.

## Why agents trust it

Every claim can point back to `timestamp_ms`, stream index, subtitle range,
frame index, or source hash. Structural keyframes are preferred over sampling
every frame.

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
