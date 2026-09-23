pub mod http_transport;
pub mod read_video;
pub mod search_video;
pub mod tool_routes;
mod family_envelope;
pub mod video_evidence;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Free-form MCP tool args object (root type=object required by rmcp ≥1.8 schema gate).
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
struct FreeformToolArgs(Map<String, Value>);

impl FreeformToolArgs {
    fn into_value(self) -> Value {
        Value::Object(self.0)
    }
}

pub const SERVER_NAME: &str = "cue";
/// The product version, injected at build time from package.json (see the
/// `build:rust` script). Falls back to the release this source last shipped so a
/// plain `cargo build` still compiles. Never hand-edit the fallback except when
/// cutting a release that does not go through `build:rust`.
pub const SERVER_VERSION: &str = match option_env!("CUE_PRODUCT_VERSION") {
    Some(version) => version,
    None => "0.3.3",
};
pub const SERVER_INSTRUCTIONS: &str =
    "Video answers with timestamp-level proof. read_video defaults to profile fast: container metadata, streams, chapters, and embedded subtitles. It does not detect scenes, extract frames, or run speech recognition. profile quality or include_scenes detects scenes. video_evidence renders, crops, or OCRs one frame at a timestamp. No per-frame vision model.";

#[derive(Clone)]
pub struct VideoReaderMcp {
    pub tool_router: ToolRouter<Self>,
}

impl VideoReaderMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl VideoReaderMcp {
    #[tool(
        description = "Read a local video timeline. The default fast profile returns container metadata, streams, chapters, and embedded subtitles. It does not detect scenes, extract frames, or run speech recognition. Set profile to quality, or set include_scenes, for scene boundaries. Use video_evidence for a frame. No per-frame vision model."
    )]
    fn read_video(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        read_video::read_video(args.into_value())
    }

    #[tool(
        description = "Search embedded subtitle cues and return timestamped matches. Transcript matches are included only when a transcript is already present. This tool does not run speech recognition."
    )]
    fn search_video(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        search_video::search_video(args.into_value())
    }

    #[tool(
        description = "Named follow-up after read_video, not part of the default read. render_frame, crop_frame, or ocr_frame at a timestamp."
    )]
    fn video_evidence(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        video_evidence::video_evidence(args.into_value())
    }
}

#[tool_handler]
impl ServerHandler for VideoReaderMcp {
    fn get_info(&self) -> ServerInfo {
        // rmcp >=1.8: ServerInfo/Implementation are #[non_exhaustive] — use builders only.
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(
                Implementation::new(SERVER_NAME, SERVER_VERSION)
                    .with_description(
                        "Rust-native MCP server for Cue (@sylphx/cue) (modelcontextprotocol/rust-sdk rmcp)",
                    )
                    .with_website_url("https://sylphxai.github.io/cue/"),
            )
            .with_instructions(SERVER_INSTRUCTIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::VideoReaderMcp;
    #[test]
    fn exposes_primary_tool_surface() {
        let tools = VideoReaderMcp::new().tool_router.list_all();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.to_string()).collect();
        assert!(names.contains(&"read_video".to_string()));
        assert!(names.contains(&"video_evidence".to_string()));
        assert!(names.contains(&"search_video".to_string()));
    }

    #[test]
    fn read_video_tool_text_keeps_scenes_off_the_default_call() {
        let tools = VideoReaderMcp::new().tool_router.list_all();
        let read_video = tools
            .iter()
            .find(|tool| tool.name == "read_video")
            .expect("read_video");
        let description = read_video.description.as_deref().unwrap_or("");
        assert!(description.contains("default fast profile"));
        assert!(description.contains("does not detect scenes"));
        let search = tools
            .iter()
            .find(|tool| tool.name == "search_video")
            .expect("search_video");
        let search_description = search.description.as_deref().unwrap_or("");
        assert!(search_description.contains("does not run speech recognition"));
    }

    #[test]
    fn server_info_is_brand_sole_cue() {
        use rmcp::ServerHandler;
        use super::{SERVER_NAME, SERVER_VERSION};
        let info = VideoReaderMcp::new().get_info();
        let name = info.server_info.name.to_string();
        let version = info.server_info.version.to_string();
        assert_eq!(name, SERVER_NAME);
        assert_eq!(version, SERVER_VERSION);
        assert_eq!(SERVER_NAME, "cue");
        assert!(!name.contains("video-reader"));
    }

}