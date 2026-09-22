use video_reader_core::search_video_from_value;
use serde_json::json;

#[test]
fn requires_a_non_empty_search_query() {
    let error = search_video_from_value(&json!({ "query": "  " })).expect_err("empty query");
    assert!(error.message.contains("query"));
}
