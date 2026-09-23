---
layout: home

hero:
  name: Cue
  text: Video answers with timestamp-level proof.
  tagline: The default read is container metadata, streams, chapters, and embedded subtitles. Scenes are a separate request. A frame is a separate tool.
  actions:
    - theme: brand
      text: Get started
      link: /guide/quickstart
    - theme: alt
      text: Star on GitHub
      link: https://github.com/SylphxAI/cue
---

<div class="lk-section">
  <span class="lk-eyebrow">The difference</span>
  <h2 class="lk-h2">A caption is not a timestamp.<br />A subtitle cue you can seek to is.</h2>
  <p class="lk-lead">Cue reads a local file with ffprobe and ffmpeg. The default profile is <code>fast</code>. It does not detect scenes, extract frames, or run speech recognition. <code>quality</code> adds scene boundaries. One frame is <code>video_evidence</code>.</p>
  <div class="lk-compare" style="margin-top:28px">
    <div class="side">
      <h3>What a guess says</h3>
      <p>“The pricing line is somewhere in the middle.” No stream, no cue, no warning if the file has no subtitles.</p>
    </div>
    <div class="side good">
      <h3>What Cue returns</h3>
      <p>container and streams from ffprobe · chapter <span class="lk-cite">start_ms</span> when the file has chapters · subtitle <span class="lk-cite">start_ms</span> / <span class="lk-cite">end_ms</span> / <span class="lk-cite">text</span> when a subtitle stream exists · a warning when it does not.</p>
    </div>
  </div>
</div>

## The subtitle object. Not a sample file.

When an embedded subtitle stream exists, each cue has this shape. The numbers and the words come from that file. This page does not invent them.

```json
{
  "index": 0,
  "start_ms": 0,
  "end_ms": 0,
  "text": "cue text",
  "provenance": { "method": "ffmpeg_extract", "format": "srt" }
}
```

<p class="lk-fine"><code>method</code> is <code>ffmpeg_extract</code> and <code>format</code> is <code>srt</code>. <code>index</code>, <code>start_ms</code>, <code>end_ms</code>, and <code>text</code> are filled from the first subtitle stream. If that stream is missing, the read still returns the probe and adds the warning “No readable embedded subtitle stream was found.”</p>

<div class="lk-section">
  <span class="lk-eyebrow">How it works</span>
  <h2 class="lk-h2">Three steps from a file to a timestamp</h2>
  <div class="lk-steps" style="margin-top:26px">
    <div class="lk-step">
      <div class="n">Step 1</div>
      <h3>Add it to your agent</h3>
      <p>One <code>npx</code> line. A stdio MCP server starts for Claude, Cursor, VS Code, Codex, or any other MCP client. No API key. ffmpeg and ffprobe must be on PATH.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 2</div>
      <h3>Read the timeline first</h3>
      <p><code>read_video</code> with no profile is <code>fast</code>. <code>search_video</code> matches embedded subtitle text only. It does not run speech recognition.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 3</div>
      <h3>Open the moment</h3>
      <p>Use <code>start_ms</code> and the source hash. For a picture of that moment, call <code>video_evidence</code> with the timestamp. Do not describe a frame the tool did not return.</p>
    </div>
  </div>
</div>

<div class="lk-section">
  <span class="lk-eyebrow">What you call</span>
  <h2 class="lk-h2">Name the work. The expensive pass stays off.</h2>
  <p class="lk-lead">Scene detection decodes the file. It does not run unless you set <code>profile</code> to <code>quality</code> or set <code>include_scenes</code>. Nothing in the default read samples frames.</p>
  <div class="lk-grid three" style="margin-top:26px">
    <div class="lk-card">
      <h3>read_video</h3>
      <p><code>fast</code> returns format, streams, chapters, and embedded subtitles. <code>quality</code> adds ffmpeg scene changes at threshold 0.4 unless you set another. A scene object is <code>time_ms</code> plus <code>provenance.method</code> <code>ffmpeg_scene_filter</code>.</p>
    </div>
    <div class="lk-card">
      <h3>search_video</h3>
      <p>Finds the query in embedded subtitle cues and returns <code>start_ms</code> and <code>end_ms</code>. No transcript is created for the search.</p>
    </div>
    <div class="lk-card">
      <h3>video_evidence</h3>
      <p>Named follow-up. <code>op</code> is <code>render_frame</code>, <code>crop_frame</code>, or <code>ocr_frame</code> at one timestamp. It is not part of <code>read_video</code>.</p>
    </div>
  </div>
</div>

<div class="lk-section">
  <span class="lk-eyebrow">Limits</span>
  <h2 class="lk-h2">What the shipped server does not do.</h2>
  <div class="lk-limits" style="margin-top:26px">
    <div class="stat"><div class="num">fast</div><div class="lbl">default profile. Container, streams, chapters, embedded subtitles. No scene pass.</div></div>
    <div class="stat"><div class="num">0.4</div><div class="lbl">default ffmpeg scene threshold, only after <code>quality</code> or <code>include_scenes</code>.</div></div>
    <div class="stat"><div class="num">None</div><div class="lbl">API key. No cloud speech recognition and no vision model on this path.</div></div>
    <div class="stat"><div class="num">Empty</div><div class="lbl"><code>include_keyframes</code> and <code>include_transcript</code> warn and return no frames and no transcript.</div></div>
  </div>
  <p class="lk-fine">These are the switches in the shipped Rust server, not a speed benchmark. <code>npx -y @sylphx/cue</code> runs that server. A missing ffprobe is an error, not a guessed timeline.</p>
</div>

## Install

```bash
npx -y @sylphx/cue
```

::: code-group
```json [Claude Desktop / Cursor / VS Code]
{
  "mcpServers": {
    "cue": { "command": "npx", "args": ["-y", "@sylphx/cue"] }
  }
}
```

```bash [Claude Code]
claude mcp add cue -- npx -y @sylphx/cue
```

```bash [Any agent / CLI]
npx -y @sylphx/cue
```
:::

<div class="lk-cta">
  <h2>Read the timeline. Seek the cue.</h2>
  <p>No API key. Scenes stay off until you ask. A frame is a tool you name.</p>
  <p style="margin-top:18px"><a class="VPButton brand" href="./guide/quickstart">Read the quickstart</a> <a class="VPButton alt" href="https://github.com/SylphxAI/cue">Star the repo</a></p>
</div>
