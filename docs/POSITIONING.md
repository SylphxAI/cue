# Positioning — Cue

**One-liner:** Video answers with timestamp-level proof.

- **User:** an agent that must find a quote, chapter, code demo, or visual moment in a local video.
- **Job:** the default read returns container metadata, streams, chapters, and embedded subtitles. Scene boundaries, frames, crops, and OCR are separate requests.
- **Promise:** every claim points at a timestamp or a stream locator. A frame locator appears only after a frame follow-up. Missing subtitles are reported as gaps. Speech recognition stays off unless requested.
- **Identity:** package `@sylphx/cue`, bin `cue`, MCP `io.github.SylphxAI/cue`, site <https://sylphxai.github.io/cue/>.
- **Companion tools:** Citra, Iris, Cue, Spine, Locus and Lookout are independent products composed through public MCP and SDK contracts.

See [vision.md](./vision.md) and [capabilities.md](./capabilities.md) for the destination and the owned capabilities. [TOOL_SURFACE.md](./TOOL_SURFACE.md) is the tool policy and [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md) is the result contract.
