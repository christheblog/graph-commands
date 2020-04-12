use hg_core::path;


const OVERALL_SCORE: &str = "{score}";
const OVERALL_SCORE_SHORT: &str = "%S";
const NODE_ID: &str = "{id}";
const NODE_ID_SHORT: &str = "%i";
const REPEAT: &str = "...";

pub fn format_path(_path: &path::Path, _format: &str) -> String {
    unimplemented!()
}

pub fn format_scored_path(_path: &path::ScoredPath, _format: &str) -> String {
    unimplemented!()
}
