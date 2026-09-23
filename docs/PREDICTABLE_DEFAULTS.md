# Predictable defaults

Omitted `profile` is `fast`. Fast returns container metadata, streams, chapters,
and embedded subtitles. It does not detect scenes, extract frames, or run speech
recognition.

`profile` `quality` sets `include_scenes` and `include_keyframes` only. It does
not enable OCR or speech recognition. `include_transcript` stays false unless
the caller sets it, including on `quality`. OCR stays on `video_evidence`.

No cloud video API or frame-by-frame vision model is required.
