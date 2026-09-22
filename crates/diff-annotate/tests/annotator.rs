use trex_diff_annotate::{AddCommentArgs, CommentSide, DiffAnnotator};

fn add_comment(a: &mut DiffAnnotator, file: &str, line: u32, side: CommentSide, content: &str) -> trex_diff_annotate::DiffComment {
    a.add_comment(AddCommentArgs {
        file_path: file.to_string(),
        line_number: line,
        side,
        content: content.to_string(),
        author: "alice".to_string(),
    })
}

#[test]
fn comment_is_persisted_with_identity() {
    let mut a = DiffAnnotator::new();
    let c = add_comment(&mut a, "src/lib.rs", 10, CommentSide::Right, "rename this");
    assert!(c.id != uuid::Uuid::nil());
    assert_eq!(a.all_comments().len(), 1);
    assert_eq!(a.all_comments()[0].sent_at, None);
}

#[test]
fn comments_are_filtered_by_file() {
    let mut a = DiffAnnotator::new();
    add_comment(&mut a, "src/lib.rs", 10, CommentSide::Right, "a");
    add_comment(&mut a, "src/lib.rs", 12, CommentSide::Right, "b");
    add_comment(&mut a, "src/other.rs", 1, CommentSide::Left, "c");
    assert_eq!(a.comments_for_file("src/lib.rs").len(), 2);
    assert_eq!(a.comments_for_file("src/missing.rs").len(), 0);
}

#[test]
fn mark_sent_flags_the_comment() {
    let mut a = DiffAnnotator::new();
    let c = add_comment(&mut a, "src/lib.rs", 5, CommentSide::Left, "fix");
    assert!(a.mark_sent(c.id));
    assert!(a.all_comments()[0].sent_at.is_some());
    assert!(!a.mark_sent(uuid::Uuid::new_v4()));
}

#[test]
fn clear_removes_only_the_targeted_file() {
    let mut a = DiffAnnotator::new();
    add_comment(&mut a, "src/lib.rs", 1, CommentSide::Right, "a");
    add_comment(&mut a, "src/other.rs", 2, CommentSide::Right, "b");
    a.clear_for_file("src/lib.rs");
    assert_eq!(a.all_comments().len(), 1);
    assert_eq!(a.all_comments()[0].file_path, "src/other.rs");
    a.clear_all();
    assert_eq!(a.all_comments().len(), 0);
}

#[test]
fn format_for_agent_renders_comments_or_none() {
    let mut a = DiffAnnotator::new();
    assert!(a.format_for_agent("src/lib.rs").is_none());
    add_comment(&mut a, "src/lib.rs", 42, CommentSide::Right, "rename");
    let out = a.format_for_agent("src/lib.rs").unwrap();
    assert!(out.contains("## Diff Comments for src/lib.rs"));
    assert!(out.contains("**Line 42 (right)**: rename"));
}

#[test]
fn serde_round_trip_preserves_comment() {
    let mut a = DiffAnnotator::new();
    let c = add_comment(&mut a, "a.rs", 3, CommentSide::Left, "x");
    let json = serde_json::to_string(&c).unwrap();
    let back: trex_diff_annotate::DiffComment = serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, c.id);
    assert_eq!(back.line_number, 3);
}