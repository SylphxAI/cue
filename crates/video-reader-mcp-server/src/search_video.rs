use rmcp::model::CallToolResult;
use serde_json::Value;

pub fn search_video(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let response = video_reader_core::search_video_from_value(&args).map_err(|error| {
        rmcp::ErrorData::invalid_params(error.message, None)
    })?;
    Ok(CallToolResult::structured(serde_json::to_value(response).map_err(|error| {
        rmcp::ErrorData::internal_error(format!("Failed to serialize search_video: {error}"), None)
    })?))
}
