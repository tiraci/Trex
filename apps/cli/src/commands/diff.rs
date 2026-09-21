use serde_json::{Value, json};
use crate::output::Failure;

pub fn comment(
    file: &str,
    line: u32,
    side: &str,
    text: &str,
) -> Result<(Value, String), Failure> {
    let human = format!("Added comment on {}:{} ({})", file, line, side);
    Ok((
        json!({
            "file": file,
            "line": line,
            "side": side,
            "text": text,
        }),
        human,
    ))
}

pub fn ls(file: &Option<String>) -> Result<(Value, String), Failure> {
    let human = match file {
        Some(f) => format!("No comments for {}", f),
        None => "No diff comments.".to_string(),
    };
    Ok((json!({ "comments": [] }), human))
}

pub fn format(file: &str) -> Result<(Value, String), Failure> {
    let human = format!("No comments to format for {}", file);
    Ok((json!({ "formatted": null }), human))
}

pub fn clear(file: &Option<String>) -> Result<(Value, String), Failure> {
    let human = match file {
        Some(f) => format!("Cleared comments for {}", f),
        None => "Cleared all diff comments.".to_string(),
    };
    Ok((json!({ "cleared": true }), human))
}
