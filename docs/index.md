---
layout: home
hero:
  name: "Cue"
  text: "Video answers with timestamp-level proof"
  tagline: "Container metadata, streams, chapters, and embedded subtitles. Scenes and frames are separate."
  actions:
    - theme: brand
      text: Quickstart
      link: /guide/quickstart
    - theme: alt
      text: Tool reference
      link: /reference/tools
features:
  - title: Local-first
    details: The default path keeps source material on your machine and requires no API key.
  - title: Predictable work
    details: The default read returns container metadata, streams, chapters, and embedded subtitles. Scenes and frames are a separate request. OCR and speech recognition stay off unless you ask for them.
  - title: Citeable output
    details: Results carry source locators, routes, warnings, and gaps so agents can verify claims.
---

## Install

```bash
npx -y @sylphx/cue
```

Use `cue` from the CLI or connect the same tools to Claude Code, Codex, Cursor,
VS Code, and any MCP client.
