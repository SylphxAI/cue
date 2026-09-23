# Predictable defaults

Omitted `profile` is `fast`. Fast returns container metadata, streams, chapters,
and embedded subtitles. It does not detect scenes, extract frames, or run speech
recognition.

`profile` `quality` sets `include_scenes` only. It does not extract frames or
run speech recognition. `include_keyframes` and `include_transcript` stay false
unless the caller sets them, including on `quality`. On the shipped server those
flags warn and return nothing. A frame is `video_evidence`.

No cloud video API or frame-by-frame vision model is required.
