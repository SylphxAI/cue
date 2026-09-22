//! Explicit shipped routing table for Cue public tools only.
//! Internal helpers must never appear in tools/list.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolRoute {
    RustCore,
}

pub fn route_for_tool(tool: &str) -> Option<ToolRoute> {
    match tool {
        "read_video" | "search_video" | "video_evidence" => Some(ToolRoute::RustCore),
        _ => None,
    }
}

/// Public MCP tools (tools/list).
pub const PUBLIC_TOOLS: [&str; 3] = ["read_video", "search_video", "video_evidence"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_only_public_tools() {
        assert_eq!(route_for_tool("read_video"), Some(ToolRoute::RustCore));
        assert_eq!(route_for_tool("search_video"), Some(ToolRoute::RustCore));
        assert_eq!(route_for_tool("video_evidence"), Some(ToolRoute::RustCore));
        assert_eq!(route_for_tool("hash_source"), None);
        assert_eq!(route_for_tool("build_cache_key"), None);
        assert_eq!(route_for_tool("assemble_probe_timeline"), None);
        assert_eq!(route_for_tool("render_frame"), None);
        assert_eq!(route_for_tool("crop_frame"), None);
    }

    #[test]
    fn public_surface_is_three_tools() {
        assert_eq!(PUBLIC_TOOLS.len(), 3);
    }
}
