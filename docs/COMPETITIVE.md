# Cue — competitive positioning

## Job

Video timeline evidence for agents

## Wedge

Local timeline proof: ffprobe metadata, chapters, and embedded subtitles by default. Scenes, frames, and crops are separate requests, with time locators — without frame-by-frame LLM vision.

## Local-first

Operates on local media with ffmpeg/ffprobe; no required paid video API.

## Peer anchors (learn; do not clone)

| Peer | Gap we exploit |
| --- | --- |
| mcp-video-analyzer / YouTube transcript MCPs | Remote platforms + transcripts; often not local-file timeline engines |
| anthropics/popcorn | Local long-form video understanding with frames/transcripts — heavier agent skill path |
| whisper.cpp transcriber MCPs | Speech-recognition focused; not a local probe, subtitle, and optional scene or crop kit |

## Non-goals

- Becoming a cloud SaaS wrapper as the default path
- Generative summaries as the sole evidence authority

## Zero-config CTA

```bash
npx -y @sylphx/cue
```

Bare MCP stdio for agents.
