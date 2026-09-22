use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::{json, Value};

use crate::envelope::build_read_video_envelope;
use crate::ffprobe::{is_ffprobe_available, run_ffprobe};
use crate::hash::{build_cache_key, hash_source_file, CacheOptions};
use crate::timeline::{assemble_probe_timeline, AssembleOptions, TIMELINE_ROUTE};
use crate::AgentEvidenceEnvelope;

pub const READ_VIDEO_ROUTE: &str = "rust-read-video-v1";
pub const SERVER_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadVideoErrorCode {
    InvalidParams,
    InvalidRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadVideoError {
    pub code: ReadVideoErrorCode,
    pub message: String,
}

impl ReadVideoError {
    pub(crate) fn invalid_params(message: impl Into<String>) -> Self {
        Self {
            code: ReadVideoErrorCode::InvalidParams,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: ReadVideoErrorCode::InvalidRequest,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TimelineProvenance {
    pub source: String,
    pub tool: &'static str,
    pub version: &'static str,
    pub extracted_at: String,
    pub source_hash: String,
    pub cache_key: String,
    pub assembly_route: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimelineDocument {
    pub provenance: TimelineProvenance,
    pub format: crate::timeline::FormatInfo,
    pub streams: Vec<crate::timeline::StreamInfo>,
    pub chapters: Vec<crate::timeline::ChapterInfo>,
    pub scenes: Vec<Value>,
    pub subtitles: Vec<Value>,
    pub transcript: Vec<Value>,
    pub keyframes: Vec<Value>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoSourceResult {
    pub success: bool,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<TimelineDocument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadVideoResponse {
    pub route: &'static str,
    pub engine: &'static str,
    pub results: Vec<VideoSourceResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub envelope: Option<AgentEvidenceEnvelope>,
}

#[derive(Debug, Clone)]
pub struct ReadVideoOptions {
    pub include_streams: bool,
    pub include_chapters: bool,
    pub include_subtitles: bool,
    pub include_scenes: bool,
    pub include_transcript: bool,
    pub include_keyframes: bool,
    pub include_keyframe_images: bool,
    pub keyframe_limit: u32,
    pub keyframe_max_dimension: Option<u32>,
    pub scene_threshold: f64,
}

impl Default for ReadVideoOptions {
    fn default() -> Self {
        Self {
            include_streams: true,
            include_chapters: true,
            include_subtitles: true,
            include_scenes: true,
            include_transcript: false,
            include_keyframes: false,
            include_keyframe_images: false,
            keyframe_limit: 8,
            keyframe_max_dimension: None,
            scene_threshold: 0.4,
        }
    }
}

fn parse_srt_timestamp(value: &str) -> Option<u64> {
    let mut parts = value.trim().split(':');
    let hours: u64 = parts.next()?.parse().ok()?;
    let minutes: u64 = parts.next()?.parse().ok()?;
    let seconds_part = parts.next()?;
    let mut sec_parts = seconds_part.split(',');
    let seconds: u64 = sec_parts.next()?.parse().ok()?;
    let millis: u64 = sec_parts.next().unwrap_or("0").parse().ok()?;
    Some(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis)
}

fn extract_subtitles(path: &Path) -> (Vec<Value>, Vec<String>) {
    let output = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-i"])
        .arg(path)
        .args(["-map", "0:s:0", "-c:s", "srt", "-f", "srt", "-"])
        .output();
    let Ok(output) = output else {
        return (Vec::new(), vec!["Embedded subtitle extraction is unavailable.".into()]);
    };
    if !output.status.success() {
        return (Vec::new(), vec!["No readable embedded subtitle stream was found.".into()]);
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut subtitles = Vec::new();
    for (index, block) in text.split("\n\n").enumerate() {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 2 { continue; }
        let time_line = lines.iter().find(|line| line.contains("-->")).copied().unwrap_or("");
        let mut times = time_line.split("-->");
        let start = times.next().and_then(parse_srt_timestamp);
        let end = times.next().and_then(parse_srt_timestamp);
        let body = lines.iter().skip_while(|line| !line.contains("-->")).skip(1).cloned().collect::<Vec<_>>().join(" ");
        if let (Some(start), Some(end), true) = (start, end, !body.trim().is_empty()) {
            subtitles.push(json!({
                "index": index,
                "start_ms": start,
                "end_ms": end,
                "text": body.trim(),
                "provenance": { "method": "ffmpeg_extract", "format": "srt" }
            }));
        }
    }
    if subtitles.is_empty() {
        (Vec::new(), vec!["Embedded subtitle stream was empty or unparsable.".into()])
    } else {
        (subtitles, Vec::new())
    }
}

fn detect_scenes(path: &Path, threshold: f64) -> (Vec<Value>, Vec<String>) {
    let output = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "info", "-i"])
        .arg(path)
        .args(["-vf", &format!("select='gt(scene,{threshold})',showinfo"), "-f", "null", "-"])
        .output();
    let Ok(output) = output else {
        return (Vec::new(), vec!["Scene detection is unavailable.".into()]);
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut scenes = Vec::new();
    for (index, segment) in stderr.split("pts_time:").skip(1).enumerate() {
        let token: String = segment
            .chars()
            .take_while(|ch| ch.is_ascii_digit() || *ch == '.')
            .collect();
        if let Ok(seconds) = token.parse::<f64>() {
            scenes.push(json!({
                "index": index,
                "time_ms": (seconds * 1000.0).round() as u64,
                "provenance": { "method": "ffmpeg_scene_filter", "threshold": threshold }
            }));
        }
    }
    if scenes.is_empty() {
        (Vec::new(), vec!["No scene boundaries were detected.".into()])
    } else {
        (scenes, Vec::new())
    }
}

pub fn read_video_source(path: &Path, options: &ReadVideoOptions) -> Result<TimelineDocument, ReadVideoError> {
    if !path.is_file() {
        return Err(ReadVideoError::invalid_request(format!(
            "Unable to read video at '{}': not a regular file.",
            path.display()
        )));
    }

    if !is_ffprobe_available() {
        return Err(ReadVideoError::invalid_request(
            "ffprobe is unavailable on the default Rust read_video route. Install ffmpeg/ffprobe or use VIDEO_READER_MCP_TRANSPORT=ts.",
        ));
    }

    let ffprobe = run_ffprobe(path).map_err(ReadVideoError::invalid_request)?;
    let assembled = assemble_probe_timeline(
        &ffprobe,
        &AssembleOptions {
            include_streams: options.include_streams,
            include_chapters: options.include_chapters,
        },
    );

    let source_hash = hash_source_file(path).map_err(ReadVideoError::invalid_request)?;
    let cache_key = build_cache_key(
        &source_hash,
        &CacheOptions {
            include_streams: options.include_streams,
            include_chapters: options.include_chapters,
            include_subtitles: options.include_subtitles,
            include_scenes: options.include_scenes,
            include_transcript: options.include_transcript,
            include_keyframes: options.include_keyframes,
            include_keyframe_images: options.include_keyframe_images,
            keyframe_limit: options.keyframe_limit,
            keyframe_max_dimension: options.keyframe_max_dimension,
            scene_threshold: options.scene_threshold,
        },
    );

    let mut warnings = assembled.warnings.clone();
    let subtitles = if options.include_subtitles {
        let (subtitles, subtitle_warnings) = extract_subtitles(path);
        warnings.extend(subtitle_warnings);
        subtitles
    } else { Vec::new() };
    let scenes = if options.include_scenes {
        let (scenes, scene_warnings) = detect_scenes(path, options.scene_threshold);
        warnings.extend(scene_warnings);
        scenes
    } else { Vec::new() };
    if options.include_transcript {
        warnings.push(
            "ASR transcript extraction is not available on the default Rust read_video route; use VIDEO_READER_MCP_TRANSPORT=ts.".into(),
        );
    }
    if options.include_keyframes {
        warnings.push(
            "Keyframe extraction is not available on the default Rust read_video route; use VIDEO_READER_MCP_TRANSPORT=ts or video_evidence.".into(),
        );
    }

    Ok(TimelineDocument {
        provenance: TimelineProvenance {
            source: path.display().to_string(),
            tool: "read_video",
            version: SERVER_VERSION,
            extracted_at: chrono_now_iso(),
            source_hash,
            cache_key,
            assembly_route: TIMELINE_ROUTE.into(),
        },
        format: assembled.format,
        streams: assembled.streams,
        chapters: assembled.chapters,
        scenes,
        subtitles,
        transcript: Vec::new(),
        keyframes: Vec::new(),
        warnings,
    })
}

pub fn read_video_from_value(input: &Value) -> Result<ReadVideoResponse, ReadVideoError> {
    let sources = input
        .get("sources")
        .and_then(Value::as_array)
        .ok_or_else(|| ReadVideoError::invalid_params("sources array is required"))?;

    if sources.is_empty() {
        return Err(ReadVideoError::invalid_params(
            "At least one source path is required.",
        ));
    }

    let profile = input.get("profile").and_then(Value::as_str).unwrap_or("fast");
    if !matches!(profile, "fast" | "quality") {
        return Err(ReadVideoError::invalid_params(
            "profile must be one of: fast, quality",
        ));
    }
    let options = ReadVideoOptions {
        include_streams: input
            .get("include_streams")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        include_chapters: input
            .get("include_chapters")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        include_subtitles: input
            .get("include_subtitles")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        include_scenes: input
            .get("include_scenes")
            .and_then(Value::as_bool)
            .unwrap_or(profile == "quality"),
        include_transcript: input
            .get("include_transcript")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        include_keyframes: input
            .get("include_keyframes")
            .and_then(Value::as_bool)
            .unwrap_or(profile == "quality"),
        include_keyframe_images: input
            .get("include_keyframe_images")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        keyframe_limit: input
            .get("keyframe_limit")
            .and_then(Value::as_u64)
            .map(|value| value as u32)
            .unwrap_or(8),
        keyframe_max_dimension: input
            .get("keyframe_max_dimension")
            .and_then(Value::as_u64)
            .map(|value| value as u32),
        scene_threshold: input
            .get("scene_threshold")
            .and_then(Value::as_f64)
            .unwrap_or(0.4),
    };

    let mut results = Vec::with_capacity(sources.len());
    for source in sources {
        let path = source
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| ReadVideoError::invalid_params("each source requires a path"))?;

        match read_video_source(PathBuf::from(path).as_path(), &options) {
            Ok(timeline) => results.push(VideoSourceResult {
                success: true,
                source: path.to_string(),
                timeline: Some(timeline),
                error: None,
            }),
            Err(error) => results.push(VideoSourceResult {
                success: false,
                source: path.to_string(),
                timeline: None,
                error: Some(error.message),
            }),
        }
    }

    if results.iter().all(|result| !result.success) {
        let messages: Vec<_> = results
            .iter()
            .filter_map(|result| result.error.as_deref())
            .collect();
        return Err(ReadVideoError::invalid_request(format!(
            "All video sources failed to process: {}",
            messages.join("; ")
        )));
    }

    let primary = results
        .iter()
        .find(|result| result.success)
        .expect("at least one successful result");
    let primary_path = PathBuf::from(primary.source.as_str());
    let envelope = build_read_video_envelope(primary_path.as_path(), &ReadVideoResponse {
        route: READ_VIDEO_ROUTE,
        engine: crate::ENGINE_NAME,
        results: results.clone(),
        envelope: None,
    }, primary);

    Ok(ReadVideoResponse {
        route: READ_VIDEO_ROUTE,
        engine: crate::ENGINE_NAME,
        results,
        envelope: Some(envelope),
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoSearchResponse {
    pub profile: &'static str,
    pub query: String,
    pub matches: Vec<Value>,
    pub warnings: Vec<String>,
    pub gaps: Vec<String>,
}

pub fn search_video_from_value(input: &Value) -> Result<VideoSearchResponse, ReadVideoError> {
    let query = input.get("query").and_then(Value::as_str).unwrap_or("").trim().to_lowercase();
    if query.is_empty() {
        return Err(ReadVideoError::invalid_params("query must not be empty"));
    }
    let mut read_input = input.clone();
    let obj = read_input.as_object_mut().ok_or_else(|| ReadVideoError::invalid_params("input must be an object"))?;
    obj.insert("include_subtitles".into(), json!(true));
    obj.insert("include_transcript".into(), json!(true));
    obj.entry("sources".to_string()).or_insert(json!([]));
    let response = read_video_from_value(&read_input)?;
    let mut matches = Vec::new();
    let mut warnings = Vec::new();
    let mut gaps = Vec::new();
    for result in &response.results {
        let Some(timeline) = result.timeline.as_ref() else { continue };
        for subtitle in &timeline.subtitles {
            let text = subtitle.get("text").and_then(Value::as_str).unwrap_or("");
            if text.to_lowercase().contains(&query) {
                matches.push(json!({ "source": result.source, "kind": "subtitle", "text": text, "start_ms": subtitle.get("start_ms"), "end_ms": subtitle.get("end_ms") }));
            }
        }
        for segment in &timeline.transcript {
            let text = segment.get("text").and_then(Value::as_str).unwrap_or("");
            if text.to_lowercase().contains(&query) {
                matches.push(json!({ "source": result.source, "kind": "transcript", "text": text, "start_ms": segment.get("start_ms"), "end_ms": segment.get("end_ms") }));
            }
        }
        warnings.extend(timeline.warnings.iter().cloned());
    }
    if matches.is_empty() {
        gaps.push("No subtitle or transcript match was found; frame-level evidence is available through video_evidence.".into());
    }
    Ok(VideoSearchResponse { profile: "video_search_results", query, matches, warnings, gaps })
}

fn chrono_now_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}Z", elapsed.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn read_video_from_fixture_probe_when_ffprobe_available() {
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../test/fixtures/no-subtitle.mp4");
        if !fixture.is_file() || !is_ffprobe_available() {
            return;
        }

        let response = read_video_from_value(&serde_json::json!({
            "sources": [{ "path": fixture }],
            "include_subtitles": false,
            "include_scenes": false
        }))
        .expect("read_video");

        assert_eq!(response.route, READ_VIDEO_ROUTE);
        let result = response.results.first().expect("result");
        assert!(result.success);
        let timeline = result.timeline.as_ref().expect("timeline");
        assert_eq!(timeline.provenance.assembly_route, TIMELINE_ROUTE);
        assert_eq!(timeline.provenance.source_hash.len(), 64);
    }
}