---
"@sylphx/cue": patch
---

The MCP `initialize` response advertises the canonical product site, and the package gains discovery keywords.

`serverInfo.websiteUrl` still named the retired `video-reader-mcp` slug, so every connecting agent saw the wrong identity; it now advertises `https://sylphxai.github.io/cue/`. The published `keywords` also grow to the real queries users type — video analysis, video search, transcription, subtitles, chapters, scenes, frames, timestamps, timeline, ffmpeg, ffprobe.
