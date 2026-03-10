use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostStatus {
    Draft,
    Published,
}

impl PostStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PostStatus::Draft => "draft",
            PostStatus::Published => "published",
        }
    }
}

impl TryFrom<&str> for PostStatus {
    type Error = InvalidStatus;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "draft" => Ok(PostStatus::Draft),
            "published" => Ok(PostStatus::Published),
            other => Err(InvalidStatus {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentStatus {
    Approved,
    Pending,
    Spam,
}

impl CommentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CommentStatus::Approved => "approved",
            CommentStatus::Pending => "pending",
            CommentStatus::Spam => "spam",
        }
    }
}

impl TryFrom<&str> for CommentStatus {
    type Error = InvalidStatus;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "approved" => Ok(CommentStatus::Approved),
            "pending" => Ok(CommentStatus::Pending),
            "spam" => Ok(CommentStatus::Spam),
            other => Err(InvalidStatus {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatus {
    Active,
    Pending,
    Broken,
}

impl LinkStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkStatus::Active => "active",
            LinkStatus::Pending => "pending",
            LinkStatus::Broken => "broken",
        }
    }
}

impl TryFrom<&str> for LinkStatus {
    type Error = InvalidStatus;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(LinkStatus::Active),
            "pending" => Ok(LinkStatus::Pending),
            "broken" => Ok(LinkStatus::Broken),
            other => Err(InvalidStatus {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteStatus {
    Published,
    Draft,
    Private,
}

impl NoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            NoteStatus::Published => "published",
            NoteStatus::Draft => "draft",
            NoteStatus::Private => "private",
        }
    }
}

impl TryFrom<&str> for NoteStatus {
    type Error = InvalidStatus;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "published" => Ok(NoteStatus::Published),
            "draft" => Ok(NoteStatus::Draft),
            "private" => Ok(NoteStatus::Private),
            other => Err(InvalidStatus {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteMood {
    Happy,
    Thinking,
    Sad,
    Angry,
    Calm,
    Excited,
    Tired,
    Nostalgic,
}

impl NoteMood {
    pub fn as_str(self) -> &'static str {
        match self {
            NoteMood::Happy => "happy",
            NoteMood::Thinking => "thinking",
            NoteMood::Sad => "sad",
            NoteMood::Angry => "angry",
            NoteMood::Calm => "calm",
            NoteMood::Excited => "excited",
            NoteMood::Tired => "tired",
            NoteMood::Nostalgic => "nostalgic",
        }
    }
}

impl TryFrom<&str> for NoteMood {
    type Error = InvalidStatus;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "happy" => Ok(NoteMood::Happy),
            "thinking" => Ok(NoteMood::Thinking),
            "sad" => Ok(NoteMood::Sad),
            "angry" => Ok(NoteMood::Angry),
            "calm" => Ok(NoteMood::Calm),
            "excited" => Ok(NoteMood::Excited),
            "tired" => Ok(NoteMood::Tired),
            "nostalgic" => Ok(NoteMood::Nostalgic),
            other => Err(InvalidStatus {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid status: {value}")]
pub struct InvalidStatus {
    pub value: String,
}
