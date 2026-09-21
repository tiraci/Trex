use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffComment {
    pub id: Uuid,
    pub file_path: String,
    pub line_number: u32,
    pub side: CommentSide,
    pub content: String,
    pub author: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub diff_identity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommentSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCommentArgs {
    pub file_path: String,
    pub line_number: u32,
    pub side: CommentSide,
    pub content: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedComments {
    pub file_path: String,
    pub comments: Vec<FormattedComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedComment {
    pub line_number: u32,
    pub side: CommentSide,
    pub content: String,
}

pub struct DiffAnnotator {
    comments: Vec<DiffComment>,
}

impl DiffAnnotator {
    pub fn new() -> Self {
        DiffAnnotator {
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, args: AddCommentArgs) -> DiffComment {
        let comment = DiffComment {
            id: Uuid::new_v4(),
            file_path: args.file_path,
            line_number: args.line_number,
            side: args.side,
            content: args.content,
            author: args.author,
            created_at: chrono::Utc::now(),
            sent_at: None,
            diff_identity: None,
        };
        self.comments.push(comment.clone());
        comment
    }

    pub fn comments_for_file(&self, file_path: &str) -> Vec<&DiffComment> {
        self.comments
            .iter()
            .filter(|c| c.file_path == file_path)
            .collect()
    }

    pub fn all_comments(&self) -> &[DiffComment] {
        &self.comments
    }

    pub fn mark_sent(&mut self, comment_id: Uuid) -> bool {
        if let Some(comment) = self.comments.iter_mut().find(|c| c.id == comment_id) {
            comment.sent_at = Some(chrono::Utc::now());
            true
        } else {
            false
        }
    }

    pub fn clear_for_file(&mut self, file_path: &str) {
        self.comments.retain(|c| c.file_path != file_path);
    }

    pub fn clear_all(&mut self) {
        self.comments.clear();
    }

    pub fn format_for_agent(&self, file_path: &str) -> Option<String> {
        let comments = self.comments_for_file(file_path);
        if comments.is_empty() {
            return None;
        }

        let mut output = format!("## Diff Comments for {}\n\n", file_path);
        for comment in &comments {
            let side = match comment.side {
                CommentSide::Left => "left",
                CommentSide::Right => "right",
            };
            output.push_str(&format!(
                "**Line {} ({})**: {}\n\n",
                comment.line_number, side, comment.content
            ));
        }
        Some(output)
    }
}
